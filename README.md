
# time_module - Windows 高精度时间处理库

[![License](https://img.shields.io/badge/license-LGPL--2.1--only-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.64%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20Vista%2B-brightgreen.svg)]()

time_module.dll 是一个用 Rust 编写的 Windows 动态链接库，提供高精度网络时间获取、时区转换、夏令时处理等功能。该 DLL **不修改系统时间**，仅提供时间查询和计算服务，无需管理员权限。

---

## ✨ 主要特性

- ✅ **高精度时间** - NTP 协议获取网络时间，精确到微秒
- ✅ **时区处理** - 支持偏移秒数/时区名称/经纬度三种设置方式
- ✅ **夏令时 (DST)** - 内建规则表 + Windows 系统 API 双后端
- ✅ **自动校准** - 后台线程每小时自动同步，也可手动强制同步
- ✅ **零依赖** - 纯 Rust 实现，无任何外部运行时依赖
- ✅ **旧系统兼容** - 支持 Windows Vista/7/8/10/11（32/64 位）
- ✅ **外部配置** - 支持通过 `resources/` 目录覆盖内嵌配置

---

## 📁 文件说明

### 必需文件

| 文件 | 说明 |
|------|------|
| `time_module.dll` | 主动态库文件（约 360-400 KB，UPX 压缩版） |

### 可选文件（高级用户自定义配置）

放在与 DLL 同级的 `resources/` 目录下：

| 文件 | 说明 |
|------|------|
| `countries_tz.txt` | 国家代码到时区偏移的映射表 |
| `ntp_servers.txt` | NTP 服务器列表 |
| `tz_offsets.txt` | 时区名称到偏移秒的映射表 |
| `dst_rules.txt` | 夏令时规则表 |

### 目录结构示例

```
your_app_folder/
├── your_program.exe
├── time_module.dll
└── resources/                （可选）
    ├── countries_tz.txt
    ├── ntp_servers.txt
    ├── tz_offsets.txt
    └── dst_rules.txt
```

> 注：如果 `resources/` 文件夹不存在，DLL 将使用内嵌的默认数据。

---

## 🚀 快速开始

### C/C++ 示例

```c
#include <windows.h>
#include <stdio.h>

typedef struct {
    int year, month, day, hour, minute, second, ms, us;
} FullTime;

typedef FullTime (*pfn_GetLocalTime)(void);
typedef const char* (*pfn_GetFormattedTime)(void);
typedef void (*pfn_FreeString)(char*);
typedef int (*pfn_SetTimezoneOffset)(int);

int main() {
    HMODULE dll = LoadLibraryA("time_module.dll");
    if (!dll) { printf("加载 DLL 失败\n"); return 1; }

    pfn_GetLocalTime GetLocalTime = (pfn_GetLocalTime)GetProcAddress(dll, "api_GetLocalTime");
    pfn_GetFormattedTime GetFormattedTime = (pfn_GetFormattedTime)GetProcAddress(dll, "api_GetFormattedTime");
    pfn_FreeString FreeString = (pfn_FreeString)GetProcAddress(dll, "api_FreeString");
    pfn_SetTimezoneOffset SetOffset = (pfn_SetTimezoneOffset)GetProcAddress(dll, "api_SetTimezoneOffset");

    // 设置时区为 UTC+8
    SetOffset(28800);

    // 获取并打印本地时间
    FullTime ft = GetLocalTime();
    printf("本地时间: %04d-%02d-%02d %02d:%02d:%02d\n",
           ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second);

    // 获取格式化时间字符串
    const char* timeStr = GetFormattedTime();
    printf("格式化时间: %s\n", timeStr);
    FreeString((char*)timeStr);

    FreeLibrary(dll);
    return 0;
}
```

### Python 示例（ctypes）

```python
import ctypes
from ctypes import c_int, c_char_p, Structure

class FullTime(Structure):
    _fields_ = [
        ("year", c_int), ("month", c_int), ("day", c_int),
        ("hour", c_int), ("minute", c_int), ("second", c_int),
        ("ms", c_int), ("us", c_int)
    ]

dll = ctypes.CDLL("time_module.dll")
dll.api_GetLocalTime.restype = FullTime
dll.api_GetFormattedTime.restype = c_char_p
dll.api_FreeString.argtypes = [c_char_p]
dll.api_SetTimezoneOffset.argtypes = [c_int]
dll.api_SetTimezoneOffset.restype = c_int

# 设置时区为 UTC+8
dll.api_SetTimezoneOffset(28800)

# 获取本地时间
ft = dll.api_GetLocalTime()
print(f"本地时间: {ft.year}-{ft.month:02d}-{ft.day:02d} {ft.hour:02d}:{ft.minute:02d}:{ft.second:02d}")

# 获取格式化时间
time_str = dll.api_GetFormattedTime()
print(f"格式化时间: {time_str.decode('utf-8')}")
dll.api_FreeString(time_str)
```

### C# 示例（P/Invoke）

```csharp
using System;
using System.Runtime.InteropServices;

class TimeModule
{
    [StructLayout(LayoutKind.Sequential)]
    public struct FullTime
    {
        public int year, month, day, hour, minute, second, ms, us;
    }

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern FullTime api_GetLocalTime();

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr api_GetFormattedTime();

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void api_FreeString(IntPtr ptr);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_SetTimezoneOffset(int seconds);

    static void Main()
    {
        api_SetTimezoneOffset(28800);  // UTC+8

        FullTime ft = api_GetLocalTime();
        Console.WriteLine($"本地时间: {ft.year}-{ft.month:D2}-{ft.day:D2} {ft.hour:D2}:{ft.minute:D2}:{ft.second:D2}");

        IntPtr ptr = api_GetFormattedTime();
        string timeStr = Marshal.PtrToStringAnsi(ptr);
        Console.WriteLine($"格式化时间: {timeStr}");
        api_FreeString(ptr);
    }
}
```

---

## 📚 API 函数列表

### 时间获取函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetLocalTime()` | `FullTime` | 获取经校准的本地时间（含 DST） |
| `api_GetNetworkTime()` | `FullTime` | 获取 NTP 网络时间（微秒级精度） |
| `api_GetFormattedTime()` | `const char*` | 获取格式化的时间字符串 |
| `api_FreeString()` | `void` | 释放格式化字符串内存 |
| `api_IsNTPSynced()` | `bool` | 检查 NTP 是否已同步 |

### 时区与 DST 函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetTimezoneOffset()` | `int` | 获取当前时区偏移（秒） |
| `api_SetTimezoneOffset()` | `int` | 设置时区偏移（秒） |
| `api_SetTimezoneByName()` | `int` | 通过名称设置时区 |
| `api_SetTimezoneByLocation()` | `int` | 通过经纬度设置时区 |
| `api_IsDST()` | `bool` | 判断指定国家是否处于夏令时 |
| `api_GetDSTOffset()` | `int` | 获取指定国家的 DST 偏移（秒） |
| `api_SetAutoDST()` | `void` | 启用/禁用自动 DST |
| `api_SetDSTBackend()` | `void` | 设置 DST 后端（0=规则表，1=系统API） |
| `api_GetDSTBackend()` | `int` | 获取当前 DST 后端 |
| `api_GetSystemTimezoneOffset()` | `int` | 获取系统完整时区偏移（含 DST） |
| `api_IsSystemDST()` | `bool` | 判断系统当前是否处于 DST |

### NTP 同步函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_ForceResync()` | `bool` | 强制同步 NTP（旧版） |
| `api_ForceResyncEx()` | `int` | 强制同步 NTP（返回错误码） |
| `api_SetAutoSyncEnabled()` | `void` | 启用/禁用自动同步 |
| `api_Shutdown()` | `void` | 停止后台 NTP 同步线程 |

### 错误处理函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetErrorString()` | `const char*` | 获取错误码对应的描述文字 |
| `api_GetLastError()` | `int` | 获取最后发生的错误码 |
| `api_SetLastError()` | `void` | 设置错误码 |

---

## 📊 数据结构

### FullTime 结构体

```c
typedef struct {
    int year;      // 年份（如 2026）
    int month;     // 月份（1-12）
    int day;       // 日期（1-31）
    int hour;      // 小时（0-23）
    int minute;    // 分钟（0-59）
    int second;    // 秒（0-59）
    int ms;        // 毫秒（0-999）
    int us;        // 微秒（0-999）
} FullTime;
```

### 错误码定义

| 错误码 | 名称 | 说明 |
|--------|------|------|
| 0 | `Success` | 成功 |
| 1 | `InvalidParam` | 无效参数 |
| 2 | `NtpTimeout` | NTP 请求超时 |
| 3 | `NoNtpServer` | 没有可用的 NTP 服务器 |
| 4 | `Timeout` | 操作超时 |
| 5 | `NotSynced` | NTP 未同步 |
| 6 | `FileNotFound` | 找不到资源文件 |
| 7 | `ParseError` | 解析错误 |
| 8 | `CountryNotFound` | 国家代码不在时区数据库中 |

---

## ⚙️ 配置文件格式

### countries_tz.txt（国家时区映射表）

**格式**：`国家代码,偏移秒1,偏移秒2,...`

**示例**：
```
CN,28800
US,-28800,-25200,-21600,-18000,-14400
AU,28800,34200,36000
```

**说明**：
- 国家代码：ISO 3166-1 alpha-2 标准
- 多个偏移秒：系统会根据经度自动选择

### ntp_servers.txt（NTP 服务器列表）

**格式**：每行一个服务器地址 `[:端口]`，默认端口 123

**示例**：
```
203.107.6.88:123
ntp.ntsc.ac.cn:123
time.windows.com:123
```

### tz_offsets.txt（时区名称映射表）

**格式**：`时区名称,偏移秒`

**示例**：
```
UTC+8,28800
UTC-5,-18000
UTC+5:30,19800
```

### dst_rules.txt（夏令时规则表）

**格式**：`ISO_3166-2,开始月,开始周,开始星期,开始小时,结束月,结束周,结束星期,结束小时,偏移秒,...`

**周表示**：A=1, B=2, C=3, D=4, E=5, 0=最后一周  
**星期**：SUN, MON, TUE, WED, THU, FRI, SAT（周日=1）

**示例**：
```
US,3,B,SUN,2,11,A,SUN,2,3600,0,0,0,0,0,0
GB,3,0,SUN,1,10,0,SUN,2,3600,0,0,0,0,0,0
CN,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
```

---

## ❓ 常见问题

**Q1: DLL 需要管理员权限吗？**  
A: 不需要。本 DLL 只读取系统时间，不修改系统时间。

**Q2: 支持哪些 Windows 版本？**  
A: Windows Vista SP2、Windows 7、Windows 8、Windows 10、Windows 11（32/64 位）。

**Q3: 如何自定义 NTP 服务器？**  
A: 在与 DLL 同级的 `resources/` 文件夹中创建 `ntp_servers.txt`，每行一个服务器地址。

**Q4: 网络时间不可用怎么办？**  
A: 检查防火墙是否允许 UDP 123 端口出站，或手动调用 `api_ForceResyncEx()`。

**Q5: 时区偏移超出范围会怎样？**  
A: 设置函数会返回错误码 1（InvalidParam），时区偏移保持原值不变。

**Q6: 夏令时如何工作？**  
A: 支持两种后端：① 内建规则表（`dst_rules.txt`），② Windows 系统 API。可通过 `api_SetDSTBackend()` 切换。

**Q7: 支持 2038 年之后的时间吗？**  
A: 支持。内部使用 64 位时间戳，可正确处理到 5849 年。

**Q8: 程序退出时需要调用 `api_Shutdown()` 吗？**  
A: 推荐在卸载 DLL 前调用，确保后台线程正确退出。

---

## 📥 下载

### 最新版本：v0.2.3

| 文件 | 架构 | 大小 |
|------|------|------|
| [time_module_upx_x64.dll](https://github.com/setinne/time-module-rs/releases/latest) | 64位 | 360 KB |
| [time_module_upx_x86.dll](https://github.com/setinne/time-module-rs/releases/latest) | 32位 | 387 KB |

> Windows Vista 32 位用户若 UPX 压缩版不可用，请使用原始版本。

---

## 📄 许可证

LGPL-2.1-only - 详见 [LICENSE](LICENSE) 文件

---

## 🙏 技术支持

作者: Setinne

如有问题或建议，请在 GitHub 提交 [Issue](https://github.com/setinne/time-module-rs/issues)。
```
