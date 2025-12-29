from pylogrust import debug, init, set_request_id
import time

init(log_name="demo")


@debug(crash=False)
def hello():
    print(1 / 0)


hello()
hello()
hello()
hello()
time.sleep(2)
