
# time_module - Windows 高精度时间处理库

[![License](https://img.shields.io/badge/license-LGPL--2.1--only-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.64%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20Vista%2B-brightgreen.svg)]()

time_module.dll 是一个用 Rust 编写的 Windows 动态链接库，提供高精度网络时间获取、时区转换、夏令时处理等功能。该 DLL **不修改系统时间**，仅提供时间查询和计算服务，无需管理员权限。

---

## ✨ 主要特性

- ✅ **高精度时间** - NTP 协议获取网络时间，精确到微秒；支持纳秒级本地时间
- ✅ **时区处理** - 支持偏移秒数/时区名称/经纬度三种设置方式
- ✅ **夏令时 (DST)** - 内建规则表 + Windows 系统 API 双后端
- ✅ **双历法支持** - 公历（格里高利历）与儒略历可切换
- ✅ **年份范围** - 支持公元前 4713 年至公元后 9999 年
- ✅ **自动校准** - 后台线程默认每小时自动同步 NTP 时间（间隔可配置）
- ✅ **零依赖** - 纯 Rust + Windows API，无任何外部运行时依赖
- ✅ **旧系统兼容** - 支持 Windows Vista/7/8/10/11（32/64 位）
- ✅ **小体积** - UPX 压缩后仅 ~366 KB
- ✅ **外部配置** - 支持通过 `resources/` 目录覆盖内嵌配置文件
- ✅ **Panic 安全** - 所有 FFI 函数均有 `catch_unwind` 保护
- ✅ **内存安全选择** - 提供安全的 `api_GetFormattedTimeBuf` 接口，避免手动释放

---

## 📁 文件说明

### 必需文件

| 文件 | 说明 |
|------|------|
| `time_module.dll` | 主动态库文件 |

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

### C/C++ 示例（安全缓冲区版本）

```c
#include <windows.h>
#include <stdio.h>

typedef struct {
    int year, month, day;
    int hour, minute, second;
    int ms, us;
} FullTime;

int main() {
    HMODULE dll = LoadLibraryA("time_module.dll");
    if (!dll) { printf("加载 DLL 失败\n"); return 1; }

    // 推荐使用安全版本，避免手动内存管理
    int (*pfn_GetFormattedTimeBuf)(char*, int) = (void*)GetProcAddress(dll, "api_GetFormattedTimeBuf");
    int (*pfn_SetTimezoneOffset)(int) = (void*)GetProcAddress(dll, "api_SetTimezoneOffset");

    // 设置时区为 UTC+8
    pfn_SetTimezoneOffset(28800);

    // 使用调用者分配的缓冲区
    char buf[64];
    int len = pfn_GetFormattedTimeBuf(buf, sizeof(buf));
    if (len > 0) {
        printf("时间: %s\n", buf);
    }

    FreeLibrary(dll);
    return 0;
}
```

### Python 示例（ctypes - 微秒精度，使用安全接口）

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

# 设置时区为 UTC+8
dll.api_SetTimezoneOffset(28800)

# 推荐使用安全版本（无内存泄漏风险）
dll.api_GetFormattedTimeBuf.argtypes = [ctypes.c_char_p, c_int]
dll.api_GetFormattedTimeBuf.restype = c_int
buf = ctypes.create_string_buffer(64)
length = dll.api_GetFormattedTimeBuf(buf, 64)
if length > 0:
    print(f"时间: {buf.value.decode('utf-8')}")

# 或者用传统版本（需要手动释放）
# dll.api_GetFormattedTime.restype = c_char_p
# dll.api_FreeString.argtypes = [c_char_p]
# time_str = dll.api_GetFormattedTime()
# print(time_str.decode('utf-8'))
# dll.api_FreeString(time_str)
```

---

## 📚 API 函数列表

### 时间获取函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetLocalTime()` | `FullTime` | 获取经校准的本地时间（微秒精度） |
| `api_GetLocalTimeNs()` | `FullTimeNs` | 获取经校准的本地时间（纳秒精度） |
| `api_GetNetworkTime()` | `FullTime` | 获取 NTP 网络时间（微秒精度） |
| `api_GetFormattedTime()` | `const char*` | 获取格式化时间字符串（**必须**调用 `api_FreeString` 释放） |
| `api_GetFormattedTimeBuf()` | `int` | **安全版本**：写入调用者提供的缓冲区，返回字节数 |
| `api_FreeString()` | `void` | 释放由 `api_GetFormattedTime`、`api_GetVersionString`、`api_GetErrorString` 返回的字符串 |
| `api_IsNTPSynced()` | `bool` | 检查 NTP 是否已同步 |

### 时区与 DST 函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetTimezoneOffset()` | `int` | 获取当前时区偏移（秒） |
| `api_SetTimezoneOffset()` | `int` | 设置时区偏移（秒），返回 0 成功 |
| `api_SetTimezoneByName()` | `int` | 通过名称设置时区（如 "UTC+8"） |
| `api_SetTimezoneByLocation()` | `int` | 通过经纬度设置时区 |
| `api_IsDST()` | `bool` | 判断指定国家是否处于夏令时 |
| `api_GetDSTOffset()` | `int` | 获取指定国家的 DST 偏移（秒） |
| `api_SetAutoDST()` | `void` | 启用/禁用自动 DST |
| `api_SetDSTBackend()` | `void` | 设置 DST 后端（0=规则表，1=系统API） |
| `api_GetDSTBackend()` | `int` | 获取当前 DST 后端 |
| `api_GetSystemTimezoneOffset()` | `int` | 获取系统完整时区偏移（含 DST） |
| `api_IsSystemDST()` | `bool` | 判断系统当前是否处于 DST |

### 历法函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_SetCalendarType()` | `void` | 设置历法类型（0=公历，1=儒略历） |
| `api_GetCalendarType()` | `int` | 获取当前历法类型 |

### NTP 同步控制函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_ForceResync()` | `bool` | [已弃用] 强制同步 NTP |
| `api_ForceResyncEx()` | `int` | 强制同步 NTP（返回错误码） |
| `api_SetAutoSyncEnabled()` | `void` | 启用/禁用自动 NTP 同步 |
| `api_SetSyncInterval()` | `void` | **新增** 设置自动同步间隔（秒，最小 10 秒，默认 3600） |
| `api_GetSyncInterval()` | `int` | **新增** 获取当前自动同步间隔（秒） |
| `api_GetNTPStatus()` | `int` | **新增** 获取 NTP 同步状态（0=未启动, 1=同步中, 2=已同步, 3=偏移过大） |
| `api_IsNetworkTimeAvailable()` | `bool` | 检查 NTP 网络时间是否可用 |

### 工具函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetVersion()` | `int` | 获取 DLL 版本号（如 0x000208 表示 v0.2.8） |
| `api_GetVersionString()` | `const char*` | 获取版本号字符串（**必须**调用 `api_FreeString` 释放） |
| `api_GetErrorString()` | `const char*` | 获取错误码描述文字（**必须**调用 `api_FreeString` 释放） |
| `api_GetLastError()` | `int` | 获取最后发生的错误码 |
| `api_SetLastError()` | `void` | 设置错误码 |
| `api_IsDSTAvailable()` | `bool` | 检查指定国家是否有 DST 规则 |
| `api_IsValidTimezoneOffset()` | `bool` | 检查时区偏移是否有效（-50400 ~ 50400） |
| `api_Shutdown()` | `void` | 关闭 DLL，停止后台线程 |

---

## 📊 数据结构

### FullTime 结构体（微秒精度）

```c
typedef struct {
    int year;      // 年份
    int month;     // 月份 (1-12)
    int day;       // 日期 (1-31)
    int hour;      // 小时 (0-23)
    int minute;    // 分钟 (0-59)
    int second;    // 秒 (0-59)
    int ms;        // 毫秒 (0-999)
    int us;        // 微秒 (0-999)
} FullTime;
```

### FullTimeNs 结构体（纳秒精度）

```c
typedef struct {
    int year;
    int month;
    int day;
    int hour;
    int minute;
    int second;
    int ns;        // 纳秒 (0-999,999,999)
} FullTimeNs;
```

### 错误码定义

| 错误码 | 名称 | 说明 |
|--------|------|------|
| 0 | `Success` | 操作成功 |
| 1 | `InvalidParam` | 无效参数 |
| 2 | `NtpTimeout` | NTP 请求超时 |
| 3 | `NoNtpServer` | 无可用 NTP 服务器 |
| 4 | `Timeout` | 操作超时 |
| 5 | `NotSynced` | NTP 尚未同步 |
| 6 | `FileNotFound` | 资源文件未找到 |
| 7 | `ParseError` | 解析错误 |
| 8 | `CountryNotFound` | 国家代码不在时区数据库中 |
| 9 | `DstNotAvailable` | DST 规则不可用 |
| 10 | `InternalPanic` | 内部 panic（已捕获） |
| 11 | `UnknownError` | 未知错误 |

---

## ⚠️ 注意事项

- **内存释放**：所有返回 `const char*` 的 API（`api_GetFormattedTime`、`api_GetVersionString`、`api_GetErrorString`）均返回动态分配的内存，**必须**调用 `api_FreeString` 释放。推荐使用 `api_GetFormattedTimeBuf` 避免手动内存管理。
- **网络时间零值**：`api_GetNetworkTime()` 返回全 0 表示 NTP 时间不可用，请先检查 `api_GetNTPStatus()` 或 `api_IsNetworkTimeAvailable()`。
- **后线程退出**：在卸载 DLL 前建议调用 `api_Shutdown()` 通知后台线程退出。
- **年份范围**：支持公元前 4713 年至公元后 9999 年（公历）。
- **线程安全**：所有 API 函数都是线程安全的，但全局状态设置（如时区）建议在单线程初始化阶段完成。
- **配置文件**：`resources/` 下所有文件均支持 `#` 开头的注释行。

---

## 📥 下载

### 最新版本：v0.2.8

| 文件 | 架构 | 大小 |
|------|------|------|
| [time_module_normal_x64.dll](https://github.com/setinne/time-module-rs/releases/latest) | 64位 | ~1,045 KB |
| [time_module_normal_x86.dll](https://github.com/setinne/time-module-rs/releases/latest) | 32位 | ~1,047 KB |
| [time_module_upx_x64.dll](https://github.com/setinne/time-module-rs/releases/latest) | 64位 (UPX) | ~366 KB |
| [time_module_upx_x86.dll](https://github.com/setinne/time-module-rs/releases/latest) | 32位 (UPX) | ~395 KB |


> 推荐使用 UPX 压缩版本。Windows Vista 32 位用户若压缩版不可用，请使用原始版本。

---

## 📄 许可证

LGPL-2.1-only - 详见 [LICENSE](LICENSE) 文件

---

## 🙏 技术支持

作者: Setinne

如有问题或建议，请在 GitHub 提交 [Issue](https://github.com/setinne/time-module-rs/issues)。
