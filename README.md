# time_module - Windows 高精度时间处理库

[![License](https://img.shields.io/badge/license-LGPL--2.1--only-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.64%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20Vista%2B-brightgreen.svg)]()

time_module.dll 是一个用 Rust 编写的 Windows 动态链接库，提供高精度网络时间获取、时区转换、夏令时处理等功能。该 DLL **不修改系统时间**，仅提供时间查询和计算服务，无需管理员权限。

---

## ✨ 主要特性

- ✅ **高精度时间** - NTP 协议获取网络时间，精确到微秒；支持纳秒级本地时间
- ✅ **多实例支持** (v0.2.16) - 通过句柄模式可创建多个独立配置的时间模块
- ✅ **异步 NTP 同步** (v0.2.17) - 非阻塞强制同步，回调通知结果
- ✅ **时区处理** - 支持偏移秒数/时区名称/经纬度三种设置方式，可显式控制 DST 叠加
- ✅ **夏令时 (DST)** - 内建规则表 + Windows 系统 API 双后端
- ✅ **双历法支持** - 公历（格里高利历）与儒略历可切换
- ✅ **年份范围** - 支持公元前 4713 年至公元后 9999 年（proleptic Gregorian）
- ✅ **自动校准** - 后台线程默认每小时自动同步 NTP 时间（间隔可配置）
- ✅ **指数退避重试** - 网络失败时自动降低同步频率，保护服务器资源
- ✅ **Marzullo 算法** - 多 NTP 服务器智能过滤，剔除虚假时间源，提升精度
- ✅ **闰秒平滑** - 支持 Smear 模式，在 24 小时内线性插入闰秒，避免时间跳变
- ✅ **零依赖** - 纯 Rust + Windows API，无任何外部运行时依赖
- ✅ **旧系统兼容** - 支持 Windows Vista/7/8/10/11（32/64 位）
- ✅ **小体积** - UPX 压缩后约 380~420 KB
- ✅ **外部配置** - 支持通过 `resources/` 目录覆盖内嵌配置文件
- ✅ **Panic 安全** - 所有 FFI 函数均有 `catch_unwind` 保护
- ✅ **内存安全选择** - 提供安全的 `api_GetFormattedTimeBuf` 接口，避免手动释放
- ✅ **跨语言兼容** - 所有 `bool` 返回值均提供 `Ex` 版本（返回 `int` 0/1），消除 C ABI 平台差异
- ✅ **可观测性** - 支持增强日志回调（文件名、行号、时间戳），实时监控 NTP 状态
- ✅ **自动化测试** - GitHub Actions CI，24+ 单元测试保证质量
- ✅ **静态库支持** (v0.2.17) - 提供 MinGW 静态库 `.a` 和 MSVC 导入库 `.lib`
- ✅ **模块化架构** (v0.2.19) - 代码拆分为 20 个独立模块，便于维护

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

### C/C++ 单实例使用（兼容旧版 API）

```c
#include <windows.h>
#include <stdio.h>

int main() {
    HMODULE dll = LoadLibraryA("time_module.dll");
    if (!dll) { printf("加载 DLL 失败\n"); return 1; }

    // 获取函数指针
    int (*pfn_SetTimezoneOffset)(int) = (void*)GetProcAddress(dll, "api_SetTimezoneOffset");
    int (*pfn_GetFormattedTimeBuf)(char*, int) = (void*)GetProcAddress(dll, "api_GetFormattedTimeBuf");
    void (*pfn_Shutdown)(void) = (void*)GetProcAddress(dll, "api_Shutdown");

    // 设置时区为 UTC+8
    pfn_SetTimezoneOffset(28800);

    // 使用安全缓冲区版本（无内存泄漏风险）
    char buf[64];
    if (pfn_GetFormattedTimeBuf(buf, sizeof(buf)) > 0) {
        printf("当前时间: %s\n", buf);
    }

    // 关闭后台线程
    pfn_Shutdown();
    FreeLibrary(dll);
    return 0;
}
```

### 多实例使用 (v0.2.16+)

```c
void* mod1 = api_CreateModule();
void* mod2 = api_CreateModule();

api_SetTimezoneOffsetWithModule(mod1, 28800);   // UTC+8
api_SetTimezoneOffsetWithModule(mod2, -18000);  // UTC-5

FullTime t1 = api_GetLocalTimeWithModule(mod1);
FullTime t2 = api_GetLocalTimeWithModule(mod2);

api_DestroyModule(mod1);
api_DestroyModule(mod2);
```

### 异步 NTP 同步 (v0.2.17+)

```c
void on_ntp_sync(int success, long long offset_ms, void* user_data) {
    if (success) {
        printf("NTP 同步成功，偏移 %lld 毫秒\n", offset_ms);
    } else {
        printf("NTP 同步失败\n");
    }
}

int main() {
    // 启动异步同步（非阻塞）
    api_ForceResyncAsync(NULL, on_ntp_sync, NULL);
    
    // 继续做其他工作...
    Sleep(3000);
    
    api_Shutdown();
    return 0;
}
```

### 静态链接 (MinGW)

```bash
gcc myapp.c -L. -ltime_module_x64 -o myapp.exe
```

### 静态链接 (MSVC 使用导入库)

1. 在项目中添加 `time_module_x64.lib` 作为附加依赖项
2. 将 `time_module_normal_x64.dll` 放在可执行文件目录

---

## 📚 API 函数列表

### 模块生命周期管理 (v0.2.16+)

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_CreateModule()` | `void*` | 创建新的时间模块句柄 |
| `api_DestroyModule(handle)` | `void` | 销毁句柄，释放资源 |
| 所有原有 API 的 `*WithModule` 版本 | 同原 API | 使用指定句柄进行操作 |

> 旧 API（无 `WithModule`）继续可用，内部使用默认全局句柄，保证向后兼容。

### 时间获取函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetLocalTime()` | `FullTime` | 获取经校准的本地时间（微秒精度，闰秒平滑） |
| `api_GetLocalTimeNs()` | `FullTimeNs` | 获取经校准的本地时间（纳秒精度，闰秒平滑） |
| `api_GetNetworkTime()` | `FullTime` | 获取 NTP 网络时间（微秒精度，Marzullo 过滤） |
| `api_GetFormattedTime()` | `const char*` | 获取格式化时间字符串（**必须**调用 `api_FreeString` 释放） |
| `api_GetFormattedTimeBuf()` | `int` | **安全版本**：写入调用者提供的缓冲区。缓冲区不足时返回 -1，错误码 `BufferTooSmall` |
| `api_FreeString()` | `void` | 释放由 `api_GetFormattedTime`、`api_GetVersionString`、`api_GetErrorString`、`api_GetWeekdayName`、`api_GetWeekdayNameZh` 返回的字符串 |

### 时区与 DST 函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetTimezoneOffset()` | `int` | 获取当前时区偏移（秒） |
| `api_SetTimezoneOffset()` | `int` | 设置时区偏移（秒），有效范围：-43200 到 50400（**UTC-12 到 UTC+14**） |
| `api_SetTimezoneByName()` | `int` | 通过名称设置时区（如 "UTC+8"） |
| `api_SetTimezoneByLocation()` | `int` | 通过经纬度设置时区（默认自动应用 DST） |
| `api_SetTimezoneByLocationEx()` | `int` | **推荐** 通过经纬度设置时区，显式指定是否应用 DST |
| `api_GetBaseOffsetByLocation()` | `int` | 获取基础时区偏移（不含 DST），失败返回 -1 并设置错误码 |
| `api_IsDSTEx()` | `int` | 判断指定国家是否处于夏令时（返回 1/0） |
| `api_GetDSTOffset()` | `int` | 获取指定国家的 DST 偏移（秒） |
| `api_SetAutoDST()` | `void` | 启用/禁用自动 DST |
| `api_SetDSTBackend()` | `void` | 设置 DST 后端（0=规则表，1=系统API） |
| `api_GetDSTBackend()` | `int` | 获取当前 DST 后端 |
| `api_GetSystemTimezoneOffset()` | `int` | 获取系统完整时区偏移（含 DST） |
| `api_IsSystemDSTEx()` | `int` | 判断系统当前是否处于 DST（返回 1/0） |

### 历法函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_SetCalendarType()` | `void` | 设置历法类型（0=公历，1=儒略历） |
| `api_GetCalendarType()` | `int` | 获取当前历法类型 |

### 星期函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetWeekday()` | `int` | 获取星期几（0=星期日, 1=星期一, ..., 6=星期六），无效日期返回 -1 |
| `api_GetWeekdayISO()` | `int` | 获取星期几（1=星期一, ..., 7=星期日），无效日期返回 -1 |
| `api_GetWeekdayName()` | `const char*` | 获取英文星期名称（**需调用 `api_FreeString` 释放**），无效日期返回 NULL |
| `api_GetWeekdayNameZh()` | `const char*` | 获取中文星期名称（**需调用 `api_FreeString` 释放**），无效日期返回 NULL |
| `api_GetWeekdayNameBuf()` | `int` | **安全版本**：英文星期名称写入调用者缓冲区，返回字节数 |
| `api_GetWeekdayNameZhBuf()` | `int` | **安全版本**：中文星期名称写入调用者缓冲区，返回字节数 |

### Unix 时间戳函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetUnixTimestamp()` | `int64_t` | 获取当前 Unix 时间戳（秒） |
| `api_GetUnixTimestampMs()` | `int64_t` | 获取当前 Unix 时间戳（毫秒） |
| `api_GetUnixTimestampUs()` | `int64_t` | 获取当前 Unix 时间戳（微秒） |
| `api_GetUnixTimestampNs()` | `int64_t` | 获取当前 Unix 时间戳（纳秒） |

### 日期工具函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_IsLeapYearEx()` | `int` | 判断是否为闰年（返回 1/0） |
| `api_DayOfYear()` | `int` | 获取指定日期在一年中的第几天（1-366），无效日期返回 -1 |

### NTP 同步控制函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_ForceResyncEx()` | `int` | 强制同步 NTP（同步阻塞） |
| `api_ForceResyncAsync()` | `int` | **v0.2.17** 异步强制同步 NTP，非阻塞，通过回调返回结果 |
| `api_SetAutoSyncEnabled()` | `void` | 启用/禁用自动 NTP 同步 |
| `api_SetSyncInterval()` | `void` | 设置自动同步间隔（秒，最小 10 秒，默认 3600） |
| `api_GetSyncInterval()` | `int` | 获取当前自动同步间隔（秒） |
| `api_GetNTPStatus()` | `int` | 获取 NTP 同步状态（0=未启动, 1=同步中, 2=已同步, 3=偏移过大） |
| `api_IsNTPSyncedEx()` | `int` | 检查 NTP 是否已同步（返回 1/0） |
| `api_IsNetworkTimeAvailableEx()` | `int` | 检查 NTP 网络时间是否可用（返回 1/0） |

### 日志与闰秒控制函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_SetLogCallback()` | `void` | 注册日志回调函数（回调签名包含文件名、行号、时间戳） |
| `api_SetLogLevel()` | `void` | 设置日志最低输出级别（0=Debug,1=Info,2=Warning,3=Error） |
| `api_SetLeapSecondMode()` | `void` | 设置闰秒处理模式（0=忽略, 1=平滑, 2=拒绝） |

### 工具函数

| 函数名 | 返回值 | 说明 |
|--------|--------|------|
| `api_GetVersion()` | `int` | 获取 DLL 版本号（如 0x000213 表示 v0.2.19） |
| `api_GetVersionString()` | `const char*` | 获取版本号字符串（**必须**调用 `api_FreeString` 释放） |
| `api_GetErrorString()` | `const char*` | 获取错误码描述文字（**必须**调用 `api_FreeString` 释放） |
| `api_GetLastError()` | `int` | 获取最后发生的错误码 |
| `api_IsDSTAvailableEx()` | `int` | 检查指定国家是否有 DST 规则（返回 1/0） |
| `api_IsValidTimezoneOffsetEx()` | `int` | 检查时区偏移是否有效（返回 1/0） |
| `api_Shutdown()` | `void` | 关闭 DLL，停止所有后台线程和异步任务（**必须**在卸载 DLL 前调用） |

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
| 12 | `NotInitialized` | 组件未初始化 |
| 13 | `InvalidDate` | 无效日期 |
| 14 | `BufferTooSmall` | 缓冲区太小 |
| 15 | `NtpServerUnreachable` | NTP 服务器不可达 |
| 16 | `NtpResponseInvalid` | NTP 响应无效 |
| 17 | `LogCallbackNotSet` | 日志回调未设置 |
| **18** | `TimezoneOffsetOutOfRange` | **v0.2.18** 时区偏移超出有效范围（-43200..50400，UTC-12 到 UTC+14） |
| **19** | `TimezoneNameNotFound` | **v0.2.18** 时区名称不存在 |
| **20** | `DstRuleNotFound` | **v0.2.18** DST 规则未找到 |
| **21** | `AsyncTaskFailed` | **v0.2.18** 异步任务启动失败 |

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

## ⚠️ 注意事项

- **内存释放**：所有返回 `const char*` 的 API 必须调用 `api_FreeString` 释放。推荐使用缓冲区版本（如 `api_GetFormattedTimeBuf`）避免手动内存管理。
- **跨语言 `bool`**：强烈建议使用返回 `int` 的 `Ex` 系列函数，避免 C ABI 中 `bool` 类型在不同语言的映射差异。
- **时区偏移范围**：有效范围为 -43200 到 50400 秒（**UTC-12 到 UTC+14**）。支持所有标准时区，包括基里巴斯（UTC+14）、托克劳（UTC+13）等政治选择时区。
- **DST 显式控制**：`api_SetTimezoneByLocation` 会自动叠加 DST。如需精确控制，使用 `api_SetTimezoneByLocationEx` 或先用 `api_GetBaseOffsetByLocation` 获取基础偏移。
- **NTP 零值检查**：`api_GetNetworkTime()` 返回全 0 表示 NTP 时间不可用，请先检查 `api_GetNTPStatus()` 或 `api_IsNetworkTimeAvailableEx()`。
- **必须调用 `api_Shutdown()`**：在卸载 DLL 前**必须**调用此函数以通知后台线程和异步任务退出，否则可能导致进程崩溃。
- **异步任务**：`api_ForceResyncAsync` 启动的异步任务会被 `api_Shutdown` 自动等待（最多 5 秒），确保安全退出。
- **NTP 指数退避**：网络不可达时，后台同步间隔会动态调整（初始 10 秒，每次失败加倍，最大 1 小时），成功后恢复用户设定间隔。
- **闰秒模式**：默认 `Ignore`（丢弃闰秒标志的响应）；`Smear` 模式在 24 小时内线性平滑插入；`Reject` 返回错误码。
- **日志回调**：可通过 `api_SetLogCallback` 注册回调，回调函数接收文件名、行号、时间戳和日志消息。
- **年份范围**：支持公元前 4713 年至公元后 9999 年（公历 proleptic Gregorian），星期和闰年函数支持负数年份。
- **线程安全**：所有 API 函数都是线程安全的。多实例模式下各句柄独立，互不干扰。
- **配置文件**：`resources/` 下所有文件均支持 `#` 开头的注释行。
- **日期验证**：星期函数和 `api_DayOfYear` 会对无效日期进行校验，失败时返回 -1（或 NULL）并设置错误码 `InvalidDate`。
- **缓冲区安全**：缓冲区版本函数（`*Buf`）会对缓冲区大小进行检查，不足时返回 -1 并设置 `BufferTooSmall`。

---

## 📥 下载

### 最新版本：v0.2.20

| 文件 | 架构 | 大小（约） |
|------|------|------------|
| [time_module_normal_x64.dll](https://github.com/setinne/time-module-rs/releases/latest) | 64位 | 1,086 KB |
| [time_module_normal_x86.dll](https://github.com/setinne/time-module-rs/releases/latest) | 32位 | 1,135 KB |
| [time_module_upx_x64.dll](https://github.com/setinne/time-module-rs/releases/latest) | 64位 (UPX) | 381 KB |
| [time_module_upx_x86.dll](https://github.com/setinne/time-module-rs/releases/latest) | 32位 (UPX) | 422 KB |
| [libtime_module_x64.a](https://github.com/setinne/time-module-rs/releases/latest) | 64位静态库 (MinGW) | 15.9 MB |
| [libtime_module_x86.a](https://github.com/setinne/time-module-rs/releases/latest) | 32位静态库 (MinGW) | 15.9 MB |
| [time_module_x64.lib](https://github.com/setinne/time-module-rs/releases/latest) | 64位导入库 (MSVC) | 3.4 KB |
| [time_module_x86.lib](https://github.com/setinne/time-module-rs/releases/latest) | 32位导入库 (MSVC) | 3.4 KB |

> 推荐使用 UPX 压缩版本。Windows Vista 32 位用户若压缩版不可用，请使用原始版本。

---

## 📄 许可证

LGPL-2.1-only - 详见 [LICENSE](LICENSE) 文件

---
