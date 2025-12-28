import functools
import traceback
import PyLogRust


def debug(func=None, *, crash=False):
    """
    Decorators that automatically catch exceptions and send logs to the Rust core.
    """
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
            PyLogRust.submit_error(func_name, error_msg, tb_str)
            if crash:
                raise e
            else:
                pass

    return wrapper


@debug(crash=False)
def dangerous_math(a, b):
    return a / b


print(dangerous_math(10, 2))
print(dangerous_math(10, 0))
