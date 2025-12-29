from pylogrust import debug, init, set_request_id

init(log_prefix="demo")


@debug
def hello():
    print(1 / 0)


hello()
