import functools
import traceback
import uuid
import contextvars
import time
from . import _pylogrust_core

# --- Trace Context 上下文管理 ---
# 用于在深层嵌套的函数调用中追踪同一个请求 ID
request_id_ctx = contextvars.ContextVar("request_id", default="system")


def set_request_id():
    """生成一个新的 Request ID 并绑定到当前上下文"""
    req_id = str(uuid.uuid4())[:8]  # 取前8位即可
    token = request_id_ctx.set(req_id)
    return token


# --- 增强版装饰器 ---
def debug(func=None, *, crash=False):
    if func is None:
        return functools.partial(debug, crash=crash)

    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)

        except Exception as e:
            # 1. 收集信息
            func_name = func.__name__
            error_msg = str(e)
            tb_str = traceback.format_exc()

            # 2. 获取当前的 Trace ID
            req_id = request_id_ctx.get()

            # 3. 🚀 异步发送给 Rust (极快，不会卡顿)
            PyLogRust.submit_error(func_name, error_msg, tb_str, req_id, crash)

            # 4. 决定是否崩溃
            if crash:
                raise e
            else:
                return None  # 吞掉错误

    return wrapper


# --- 主程序逻辑 ---

if __name__ == "__main__":
    # 1. 初始化 Rust 日志核心
    # log_name: 自定义日志名称
    # file_path: 日志文件路径 (传 None 则不写文件)
    # throttle_sec: 限流时间 (例如 2秒内相同的错误只记录一次)
    print("🚀 Initializing PyLogRust Core...")
    PyLogRust.init_logger(
        log_name="PaymentService", file_path="app_errors.log", throttle_sec=2
    )

    # 为了演示异步效果，我们等待一下线程启动
    time.sleep(0.1)

    print("\n--- Test 1: 正常 Request ID 追踪 ---")

    # 模拟一个 Web 请求入口
    def handle_web_request():
        set_request_id()  # 生成新的 ID
        risky_calculation(10, 0)  # 内部调用出错函数

    @debug(crash=False)
    def risky_calculation(a, b):
        return a / b

    # 模拟 3 次请求
    for _ in range(3):
        handle_web_request()
        time.sleep(0.1)

    print("\n--- Test 2: 智能限流 (Smart Throttling) ---")
    print("准备快速触发 5 次相同的错误...")
    start = time.time()
    for i in range(100):
        risky_calculation(100000, 0)
        time.sleep(0.1)  # 你会发现尽管调用了5次，但因为 throttle_sec=2，日志只会出现1次
    print(time.time() - start)
    # 防止主线程退出太快，导致后台 Rust 线程没来得及写文件
    print("\nMain thread sleeping to wait for logs...")
    time.sleep(1)
