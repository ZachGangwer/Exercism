from datetime import timedelta
from datetime import datetime


def add_gigasecond(epoch):
    giga = timedelta(seconds=10**9)

    return epoch + giga