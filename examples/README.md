
# time_module.dll 跨语言调用示例

本目录提供了 **10 种主流编程语言** 的完整示例，演示如何动态加载 `time_module.dll` 并调用其全部公开 API。

所有示例均使用 **动态加载** 方式（`LoadLibrary` / `dlopen` / `ffi` 等），无需静态链接，充分体现 FFI 通用性。

## 目录结构

```
examples/
├── README.md
├── c/
│   └── test_time.c
├── cpp/
│   └── test_time.cpp
├── python/
│   └── test_time.py
├── csharp/
│   └── TestTime.cs
├── go/
│   └── test_time.go
├── rust/
│   └── test_time.rs
├── java/
│   └── TestTime.java
├── delphi/
│   └── TestTime.dpr
├── vbnet/
│   └── TestTime.vb
└── nodejs/
    └── test_time.js
```

## 通用要求

- 操作系统：Windows (DLL 目标平台)
- 文件 `time_module.dll` 必须位于与可执行文件相同的目录或系统 PATH 中。
- 所有示例均调用以下 API（顺序可能略有不同）：
  - `api_GetVersion`
  - `api_GetVersionString`
  - `api_FreeString`
  - `api_SetTimezoneOffset`
  - `api_GetLocalTime`
  - `api_GetFormattedTime`
  - `api_GetFormattedTimeBuf`（安全缓冲区版本）
  - `api_GetWeekday`
  - `api_GetUnixTimestamp`
  - `api_IsLeapYearEx`
  - `api_Shutdown`

## 各语言示例说明

### 1. C 语言

| 项目 | 说明 |
|------|------|
| 文件 | `c/test_time.c` |
| 最低版本 | C89 (MSVC / MinGW) |
| 编译 | `gcc test_time.c -o test_time.exe` |
| 运行 | `test_time.exe` |

使用 `windows.h` 和 `LoadLibrary`，函数指针调用。

### 2. C++

| 项目 | 说明 |
|------|------|
| 文件 | `cpp/test_time.cpp` |
| 最低版本 | C++11 (MSVC / MinGW) |
| 编译 | `g++ test_time.cpp -o test_time.exe` |
| 运行 | `test_time.exe` |

使用 RAII 包装类管理 DLL 句柄。

### 3. Python

| 项目 | 说明 |
|------|------|
| 文件 | `python/test_time.py` |
| 最低版本 | Python 3.6 |
| 运行 | `python test_time.py` |

使用标准库 `ctypes`，定义结构体 `FullTime`。

### 4. C#

| 项目 | 说明 |
|------|------|
| 文件 | `csharp/TestTime.cs` |
| 最低版本 | .NET Framework 4.0 / .NET 6 |
| 编译 | `csc TestTime.cs` 或 `dotnet build` |
| 运行 | `TestTime.exe` |

使用 P/Invoke 和 `DllImport`。

### 5. Go

| 项目 | 说明 |
|------|------|
| 文件 | `go/test_time.go` |
| 最低版本 | Go 1.11 (支持 cgo) |
| 编译 | `go build -o test_time.exe` |
| 运行 | `test_time.exe` |

使用 cgo 调用 Windows API 和函数指针。

### 6. Rust

| 项目 | 说明 |
|------|------|
| 文件 | `rust/test_time.rs` |
| 最低版本 | Rust 1.64 |
| 编译 | `rustc test_time.rs` 或 `cargo build` |
| 运行 | `test_time.exe` |

使用 `libloading` crate 动态加载 DLL。若使用 Cargo，需在 `Cargo.toml` 中添加：
```toml
[dependencies]
libloading = "0.8"
```

### 7. Java

| 项目 | 说明 |
|------|------|
| 文件 | `java/TestTime.java` |
| 最低版本 | Java 8 + JNA 5.0 |
| 编译 | `javac -cp jna.jar TestTime.java` |
| 运行 | `java -cp .;jna.jar -Djava.library.path=. TestTime` |

使用 JNA (Java Native Access)，无需 JNI 手写胶水代码。需下载 `jna.jar` 并置于 classpath。

### 8. Delphi

| 项目 | 说明 |
|------|------|
| 文件 | `delphi/TestTime.dpr` |
| 最低版本 | Delphi 7 / Delphi 2007 |
| 编译 | 在 Delphi IDE 中打开并编译 |
| 运行 | 编译后的 EXE |

使用 `LoadLibrary` 和函数指针，输出到控制台。

### 9. VB.NET

| 项目 | 说明 |
|------|------|
| 文件 | `vbnet/TestTime.vb` |
| 最低版本 | .NET Framework 4.0 |
| 编译 | `vbc TestTime.vb` |
| 运行 | `TestTime.exe` |

与 C# 类似，使用 P/Invoke 和 `DllImport`。

### 10. Node.js

| 项目 | 说明 |
|------|------|
| 文件 | `nodejs/test_time.js` |
| 最低版本 | Node.js 12 + ffi-napi |
| 安装 | `npm install ffi-napi ref-napi` |
| 运行 | `node test_time.js` |

使用 `ffi-napi` 库调用 DLL 函数，`ref-napi` 处理结构体。

## 常见问题

**Q: 运行示例时提示“无法加载 time_module.dll”**  
A: 请确保 `time_module.dll` 与可执行文件在同一目录，或将其所在目录添加到系统 PATH 环境变量。

**Q: 某些语言示例需要额外依赖（如 JNA、ffi-napi）**  
A: 这些依赖均为常见库，安装说明已在对应小节中给出。

**Q: 时区偏移量的单位是什么？**  
A: 秒。示例中 `28800` 代表 UTC+8（8 × 3600 秒）。

**Q: `api_GetFormattedTimeBuf` 缓冲区需要多大？**  
A: 建议至少 64 字节，示例中均分配了 64 字节。

## 更多信息

- 主项目 README 请见项目根目录。
- 遇到问题请提交 Issue 或联系维护者。

---
