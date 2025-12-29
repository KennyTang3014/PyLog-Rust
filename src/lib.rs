use pyo3::prelude::*;
use crossbeam_channel::{unbounded, Sender, Receiver};
use std::thread;
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use once_cell::sync::OnceCell;
use colored::*;
use sysinfo::System;use chrono::Local;


struct LogEvent {
    func_name: String,
    error_msg: String,
    traceback: String,
    request_id: String,
    is_crash: bool,
}

struct LoggerConfig {
    log_name: String,
    file_path: Option<String>,
    throttle_sec: u64,
}

static SENDER: OnceCell<Sender<LogEvent>> = OnceCell::new();
static CONFIG: OnceCell<LoggerConfig> = OnceCell::new();


fn start_background_worker(receiver: Receiver<LogEvent>) {
    thread::spawn(move || {
        let mut sys = System::new_all();
        let mut throttle_map: HashMap<String, Instant> = HashMap::new();
        
        while let Ok(event) = receiver.recv() {
            let config = CONFIG.get().unwrap();
            
            if config.throttle_sec > 0 {
                if let Some(last_time) = throttle_map.get(&event.func_name) {
                    if last_time.elapsed() < Duration::from_secs(config.throttle_sec) {
                        continue;
                    }
                }
                throttle_map.insert(event.func_name.clone(), Instant::now());
            }

            sys.refresh_cpu_all(); 
            sys.refresh_memory();
            
            let cpu_usage = sys.global_cpu_usage(); 
            let mem_usage = sys.used_memory() / 1024 / 1024; // MB
            let metrics = format!("CPU: {:.1}% | Mem: {}MB", cpu_usage, mem_usage);

            let time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            
            let console_output = format!(
                "\n{} {} | {} | {} | ReqID: {}\n{} {}\n{} {}\n{}\n",
                "[PyLogRust]".bold().red(),
                time_str.blue(),
                config.log_name.yellow(),
                metrics.purple(),
                event.request_id.cyan(),
                "-> Function:".green(), event.func_name,
                "-> Error:   ".green(), event.error_msg.red().bold(),
                event.traceback.dimmed()
            );

            let file_output = format!(
                "[{}] {} [metrics: {}] [req_id: {}] Func: {} | Err: {}\n",
                time_str, config.log_name, metrics, event.request_id, event.func_name, event.error_msg
            );


            if let Some(path) = &config.file_path {
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                    let _ = writeln!(file, "{}", file_output);
                }
            }
        }
    });
}


#[pyfunction]
fn init_logger(log_name: String, file_path: Option<String>, throttle_sec: u64) {
    let config = LoggerConfig {
        log_name,
        file_path,
        throttle_sec
    };
    let _ = CONFIG.set(config);

    let (s, r) = unbounded();
    let _ = SENDER.set(s);

    start_background_worker(r);
    
    println!("✅ [Rust Core] Logger initialized with crossbeam-channel.");
}

#[pyfunction]
fn submit_error(func_name: String, error_msg: String, traceback: String, request_id: String, crash: bool) {
    if let Some(sender) = SENDER.get() {
        let event = LogEvent {
            func_name,
            error_msg,
            traceback,
            request_id,
            is_crash: crash,
        };
        let _ = sender.send(event);
    } else {
        println!("❌ Logger not initialized! Call init_logger() first.");
    }
}

#[pymodule]
fn _pylogrust_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_logger, m)?)?;
    m.add_function(wrap_pyfunction!(submit_error, m)?)?;
    Ok(())
}
