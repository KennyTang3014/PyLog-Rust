use pyo3::prelude::*;

/// 这是暴露给 Python 调用的函数
/// 接收函数名、错误信息、完整的堆栈跟踪
#[pyfunction]
fn submit_error(func_name: String, error_msg: String, traceback: String) {
    // 模拟高性能处理：
    // 在这里，我们将收到的错误信息格式化并输出。
    // 未来：这里会变成 channel.send() 扔给后台线程，实现非阻塞。
    
    let log_entry = format!(
        "\n[Rust Core 🚨] 捕获到异常!\n -> 函数: {}\n -> 错误: {}\n -> 堆栈:\n{}", 
        func_name, error_msg, traceback
    );
    
    // 暂时直接打印到控制台，验证通信是否成功
    println!("{}", log_entry);
}

/// 模块定义，将函数注册到 Python 模块中
#[pymodule]
fn ironlog_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(submit_error, m)?)?;
    Ok(())
}
