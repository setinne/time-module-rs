#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
time_module.dll Python 调用示例 (ctypes)
最低支持: Python 3.6
运行: python test_time.py
"""

import ctypes
from ctypes import c_int, c_longlong, c_char_p, c_void_p, Structure, byref, create_string_buffer

# 定义结构体
class FullTime(Structure):
    _fields_ = [
        ("year", c_int), ("month", c_int), ("day", c_int),
        ("hour", c_int), ("minute", c_int), ("second", c_int),
        ("ms", c_int), ("us", c_int)
    ]

# 加载 DLL
dll = ctypes.CDLL("time_module.dll")

# 设置函数原型
dll.api_GetVersion.argtypes = []
dll.api_GetVersion.restype = c_int

dll.api_GetVersionString.argtypes = []
dll.api_GetVersionString.restype = c_char_p

dll.api_FreeString.argtypes = [c_void_p]
dll.api_FreeString.restype = None

dll.api_SetTimezoneOffset.argtypes = [c_int]
dll.api_SetTimezoneOffset.restype = c_int

dll.api_GetLocalTime.argtypes = []
dll.api_GetLocalTime.restype = FullTime

dll.api_GetFormattedTime.argtypes = []
dll.api_GetFormattedTime.restype = c_char_p

dll.api_GetFormattedTimeBuf.argtypes = [c_char_p, c_int]
dll.api_GetFormattedTimeBuf.restype = c_int

dll.api_GetWeekday.argtypes = [c_int, c_int, c_int]
dll.api_GetWeekday.restype = c_int

dll.api_GetUnixTimestamp.argtypes = []
dll.api_GetUnixTimestamp.restype = c_longlong

dll.api_IsLeapYearEx.argtypes = [c_int]
dll.api_IsLeapYearEx.restype = c_int

dll.api_Shutdown.argtypes = []
dll.api_Shutdown.restype = None

# 1. 版本
ver = dll.api_GetVersion()
print(f"DLL 版本: {ver>>16}.{(ver>>8)&0xFF}.{ver&0xFF}")

ver_str = dll.api_GetVersionString().decode('utf-8')
print(f"版本字符串: {ver_str}")
dll.api_FreeString(ver_str)

# 2. 设置时区 UTC+8
dll.api_SetTimezoneOffset(28800)

# 3. 本地时间
ft = dll.api_GetLocalTime()
print(f"本地时间: {ft.year:04d}-{ft.month:02d}-{ft.day:02d} {ft.hour:02d}:{ft.minute:02d}:{ft.second:02d}.{ft.ms:03d}")

# 4. 格式化时间（缓冲区版本）
buf = create_string_buffer(64)
ret = dll.api_GetFormattedTimeBuf(buf, 64)
if ret > 0:
    print(f"格式化时间: {buf.value.decode('utf-8')}")
else:
    print("格式化时间失败")

# 5. 星期
wd = dll.api_GetWeekday(ft.year, ft.month, ft.day)
print(f"星期: {wd} (0=星期日)")

# 6. Unix 时间戳
ts = dll.api_GetUnixTimestamp()
print(f"Unix时间戳: {ts} 秒")

# 7. 闰年
leap = dll.api_IsLeapYearEx(2000)
print(f"2000年是闰年: {'是' if leap else '否'}")

# 8. 关闭
dll.api_Shutdown()