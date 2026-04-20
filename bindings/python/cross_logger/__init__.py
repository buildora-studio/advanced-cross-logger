from .cross_logger import LoggerConfig


class LogLevel:
    OFF   = -1
    SILLY =  0
    DEBUG =  1
    INFO  =  2
    WARN  =  3
    ERROR =  4
    FATAL =  5


__all__ = ["LoggerConfig", "LogLevel"]
