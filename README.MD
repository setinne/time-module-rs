
# TIME_MODULE.DLL 技术文档

版本: 0.1.0
日期: 2026\-05\-03
作者: Setinne
> *仍然在**开发阶段**，所有版本均为**测试版***

## 一、概述

time_module.dll 是一个用 Rust 编写的时间处理动态库，提供高精度网络时间获取、时区转换、本地时间校准等功能。该 DLL 不修改系统时间，仅提供时间查询和计算服务。

**主要特性：**

- 本地时间读取与校准

- NTP 网络时间同步（精确到微秒）

- 时区偏移设置（秒 / 名称 / 经纬度）

- 自动后台同步（每小时）

- 时区数据库内嵌（单文件分发）

- 支持外部配置文件覆盖

- 无外部依赖，Windows Vista 及以上兼容

## 二、文件说明

### 必需文件

- `time_module.dll` 主动态库文件（约 320 KB）

### 可选文件（高级用户自定义配置）

- `resources\countries_tz.txt` 国家代码到时区偏移的映射表

- `resources\ntp_servers.txt` NTP 服务器列表

- `resources\tz_offsets.txt` 时区名称到偏移秒的映射表

### 目录结构示例

```Plain Text
your_app_folder/
├── your_program.exe
├── time_module.dll
└── resources/                （可选）
    ├── countries_tz.txt
    ├── ntp_servers.txt
    └── tz_offsets.txt
```

> 注意：如果不存在 resources 文件夹，DLL 将使用内嵌的默认数据。
> 
> 

## 三、API 函数列表

### 3\.1 时间获取函数

|函数名|返回值|说明|
|---|---|---|
|api\_GetLocalTime\(\)|FullTime 结构体|获取本地时间（经偏差校准）|
|api\_GetNetworkTime\(\)|FullTime 结构体|获取 NTP 网络时间|
|api\_GetFormattedTime\(\)|const char\*|获取格式化的时间字符串|
|api\_FreeString\(\)|void|释放格式化的字符串内存|
|api\_IsNTPSynced\(\)|bool|检查 NTP 是否已同步|

### 3\.2 时区设置函数

|函数名|返回值|说明|
|---|---|---|
|api\_GetTimezoneOffset\(\)|int|获取当前时区偏移（秒）|
|api\_SetTimezoneOffset\(\)|int|设置时区偏移（秒）|
|api\_SetTimezoneByName\(\)|int|通过名称设置时区|
|api\_SetTimezoneByLocation\(\)|int|通过经纬度设置时区|

### 3\.3 NTP 同步函数

|函数名|返回值|说明|
|---|---|---|
|api\_ForceResync\(\)|bool|强制同步 NTP（旧版）|
|api\_ForceResyncEx\(\)|int|强制同步 NTP（返回错误码）|
|api\_SetAutoSyncEnabled\(\)|void|启用 / 禁用自动同步|

### 3\.4 错误处理函数

|函数名|返回值|说明|
|---|---|---|
|api\_GetErrorString\(\)|const char\*|获取错误码对应的描述文字|
|api\_GetLastError\(\)|int|获取最后发生的错误码|
|api\_SetLastError\(\)|void|设置错误码|

## 四、数据结构

### 4\.1 FullTime 结构体（时间表示）

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

### 4\.2 错误码定义

|错误码|名称|说明|
|---|---|---|
|0|Success|成功|
|1|InvalidParam|无效参数|
|2|NtpTimeout|NTP 请求超时|
|3|NoNtpServer|没有可用的 NTP 服务器|
|4|Timeout|操作超时|
|5|NotSynced|NTP 未同步|
|6|FileNotFound|找不到资源文件|
|7|ParseError|解析错误|

## 五、配置文件格式

### 5\.1 countries\_tz\.txt（国家时区映射表）

**格式**：国家代码，偏移秒 1, 偏移秒 2,\.\.\.
**示例**：

```Plain Text
CN,28800
US,-28800,-25200,-21600,-18000,-14400
AU,28800,34200,36000
```

**说明**：

- 国家代码：ISO 3166\-1 alpha\-2 标准两位字母国家代码

- 偏移秒：相对于 UTC 的时区偏移秒数（东正西负）

- 多个偏移秒：表示该国使用多个时区，系统会根据经度自动选择

### 5\.2 ntp\_servers\.txt（NTP 服务器列表）

**格式**：每行一个服务器地址 \[: 端口\]
**示例**：

```Plain Text
203.107.6.88:123
time.windows.com:123
```

**说明**：

- 服务器地址：IP 地址或域名

- 端口：可选，默认 UDP 端口 123

- 系统会按顺序尝试每个服务器，直到成功获取时间

### 5\.3 tz\_offsets\.txt（时区名称映射表）

**格式**：时区名称，偏移秒
**示例**：

```Plain Text
UTC+8,28800
UTC-5,-18000
UTC+5:30,19800
```

**说明**：

- 时区名称：如 UTC\+8、UTC\-5、UTC\+5:30 等

- 偏移秒：相对于 UTC 的秒数（东正西负）

## 六、使用示例

### 6\.1 C/C\+\+ 调用示例

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
    if (!dll) {
        printf("加载 DLL 失败\n");
        return 1;
    }

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

### 6\.2 Python 调用示例（ctypes）

```python
import ctypes
from ctypes import c_int, c_char_p, Structure

class FullTime(Structure):
    _fields_ = [
        ("year", c_int),
        ("month", c_int),
        ("day", c_int),
        ("hour", c_int),
        ("minute", c_int),
        ("second", c_int),
        ("ms", c_int),
        ("us", c_int),
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

### 6\.3 C\# 调用示例（P/Invoke）

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
        // 设置时区为 UTC+8
        api_SetTimezoneOffset(28800);

        // 获取本地时间
        FullTime ft = api_GetLocalTime();
        Console.WriteLine($"本地时间: {ft.year}-{ft.month:D2}-{ft.day:D2} {ft.hour:D2}:{ft.minute:D2}:{ft.second:D2}");

        // 获取格式化时间
        IntPtr ptr = api_GetFormattedTime();
        string timeStr = Marshal.PtrToStringAnsi(ptr);
        Console.WriteLine($"格式化时间: {timeStr}");
        api_FreeString(ptr);
    }
}
```

## 七、错误码处理

```c
int result = api_SetTimezoneOffset(50000);  // 超出范围，返回 1
if (result != 0) {
    const char* errMsg = api_GetErrorString(result);
    printf("错误: %s\n", errMsg);
}
```

## 八、常见问题解答（FAQ）

**Q1: DLL 需要管理员权限吗？**
A1: 不需要。本 DLL 只读取系统时间，不修改系统时间。

**Q2: 支持哪些 Windows 版本？**
A2: Windows Vista、Windows 7、Windows 8、Windows 10、Windows 11。

**Q3: 如何自定义 NTP 服务器？**
A3: 在与 DLL 同级的 resources 文件夹中创建 ntp\_servers\.txt，每行一个服务器地址。

**Q4: 网络时间不可用怎么办？**
A4: 检查防火墙是否允许 UDP 123 端口出站，或手动调用 api\_ForceResyncEx \(\)。

**Q5: 时区偏移超出范围会怎样？**
A5: 设置函数会返回错误码 1（InvalidParam），时区偏移保持原值不变。

**Q6: 是否支持夏令时？**
A6: 当前版本不支持自动夏令时转换，需要用户手动调整偏移或通过外部配置实现。

## 九、技术支持

作者: Setinne

如有问题或建议，请联系开发者。

---

**完**

