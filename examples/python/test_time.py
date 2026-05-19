#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
time_module.dll Python 调用示例 (ctypes)
最低支持: Python 3.6
运行: python test_time.py

v0.2.18 更新: 添加错误码处理和错误字符串演示
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

dll.api_GetErrorString.argtypes = [c_int]
dll.api_GetErrorString.restype = c_char_p

dll.api_Shutdown.argtypes = []
dll.api_Shutdown.restype = None

def print_error(func_name, error_code):
    if error_code != 0:
        err_str = dll.api_GetErrorString(error_code).decode('utf-8')
        print(f"  [错误] {func_name} 失败: 错误码 {error_code} - {err_str}")

print("========== time_module.dll 示例 (v0.2.18) ==========\n")

# 1. 版本
ver = dll.api_GetVersion()
print(f"[1] 版本信息")
print(f"    DLL 版本: {ver>>16}.{(ver>>8)&0xFF}.{ver&0xFF}")

ver_str = dll.api_GetVersionString().decode('utf-8')
print(f"    版本字符串: {ver_str}")
dll.api_FreeString(ver_str)
print()

# 2. 设置时区
print(f"[2] 时区设置")
ret = dll.api_SetTimezoneOffset(28800)
if ret != 0:
    print_error("SetTimezoneOffset", ret)
else:
    print("    设置时区 UTC+8 成功")

# 3. 演示无效时区偏移
print(f"\n[3] 无效时区偏移测试")
ret = dll.api_SetTimezoneOffset(50400)  # UTC+14，超出范围
if ret == 18:
    print("    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)")
dll.api_SetTimezoneOffset(28800)  # 恢复
print()

# 4. 本地时间
print(f"[4] 本地时间")
ft = dll.api_GetLocalTime()
print(f"    本地时间: {ft.year:04d}-{ft.month:02d}-{ft.day:02d} {ft.hour:02d}:{ft.minute:02d}:{ft.second:02d}.{ft.ms:03d}")
print()

# 5. 格式化时间
print(f"[5] 格式化时间")
buf = create_string_buffer(64)
ret = dll.api_GetFormattedTimeBuf(buf, 64)
if ret > 0:
    print(f"    格式化时间: {buf.value.decode('utf-8')}")
print()

# 6. 星期
print(f"[6] 星期信息")
wd = dll.api_GetWeekday(ft.year, ft.month, ft.day)
weekdays = ["星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"]
print(f"    星期: {weekdays[wd]} ({wd})")
print()

# 7. Unix 时间戳
print(f"[7] Unix 时间戳")
ts = dll.api_GetUnixTimestamp()
print(f"    Unix时间戳: {ts} 秒")
print()

# 8. 闰年
print(f"[8] 闰年判断")
leap = dll.api_IsLeapYearEx(2000)
print(f"    2000年是闰年: {'是' if leap else '否'}")
print()

# 9. 错误字符串演示
print(f"[9] 错误字符串演示")
err_str = dll.api_GetErrorString(18).decode('utf-8')
print(f"    错误码 18: {err_str}")
dll.api_FreeString(err_str)
err_str = dll.api_GetErrorString(19).decode('utf-8')
print(f"    错误码 19: {err_str}")
dll.api_FreeString(err_str)
print()

# 10. 关闭
print(f"[10] 关闭 DLL")
dll.api_Shutdown()
print("    完成")