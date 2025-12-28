use pyo3::prelude::*;

/// 这是暴露给 Python 调用的函数
#[pyfunction]
fn submit_error(func_name: String, error_msg: String, traceback: String) {
    let log_entry = format!(
        "\n[PyLogRust Debug] Catch the ERROR!\n -> Funciton: {}\n -> Reason: {}\n -> Traceback:\n{}", 
        func_name, error_msg, traceback
    );
    println!("{}", log_entry);
}

/// 模块定义 (注意这里的签名变化)
/// 旧写法: fn ironlog_core(_py: Python, m: &PyModule)
/// 新写法: fn ironlog_core(m: &Bound<'_, PyModule>)
#[pymodule]
fn PyLogRust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // wrap_pyfunction! 的用法基本不变，但传入 m 的方式变了
    m.add_function(wrap_pyfunction!(submit_error, m)?)?;
    Ok(())
}
