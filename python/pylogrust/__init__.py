import functools
import traceback
import uuid
import contextvars
from . import _pylogrust_core


request_id_ctx = contextvars.ContextVar("request_id", default="system")


def set_request_id():
    """Generate a new Request ID and bind it to the current context."""
    req_id = str(uuid.uuid4())[:8]
    request_id_ctx.set(req_id)
    return req_id


def init(log_name="app", file_path=None, throttle_sec=1):
    """
    Initialize the logging system.
    :param log_name: Log name/service name
    :param file_path: Log file path (optional)
    :param throttle_sec: Throttling timeout (seconds)
    """

    _pylogrust_core.init_logger(log_name, file_path, throttle_sec)


def debug(func=None, *, crash=False):
    if func is None:
        return functools.partial(debug, crash=crash)

    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)

        except Exception as e:
            func_name = func.__name__
            error_msg = str(e)
            tb_str = traceback.format_exc()

            req_id = request_id_ctx.get()

            _pylogrust_core.submit_error(func_name, error_msg, tb_str, req_id, crash)

            if crash:
                raise e
            else:
                return None

    return wrapper
