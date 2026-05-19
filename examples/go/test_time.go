/**
 * time_module.dll Go 调用示例 (cgo)
 * 最低支持: Go 1.11
 * 编译: go build -o test_time.exe
 *
 * v0.2.18 更新: 添加错误码处理和错误字符串演示
 */

package main

/*
#include <windows.h>
#include <stdio.h>

typedef struct {
    int year, month, day, hour, minute, second, ms, us;
} FullTime;

typedef int (*pfn_GetVersion)(void);
typedef const char* (*pfn_GetVersionString)(void);
typedef void (*pfn_FreeString)(char*);
typedef int (*pfn_SetTimezoneOffset)(int);
typedef FullTime (*pfn_GetLocalTime)(void);
typedef int (*pfn_GetFormattedTimeBuf)(char*, int);
typedef int (*pfn_GetWeekday)(int,int,int);
typedef long long (*pfn_GetUnixTimestamp)(void);
typedef int (*pfn_IsLeapYearEx)(int);
typedef const char* (*pfn_GetErrorString)(int);
typedef void (*pfn_Shutdown)(void);

static pfn_GetVersion _GetVersion;
static pfn_GetVersionString _GetVersionString;
static pfn_FreeString _FreeString;
static pfn_SetTimezoneOffset _SetTimezoneOffset;
static pfn_GetLocalTime _GetLocalTime;
static pfn_GetFormattedTimeBuf _GetFormattedTimeBuf;
static pfn_GetWeekday _GetWeekday;
static pfn_GetUnixTimestamp _GetUnixTimestamp;
static pfn_IsLeapYearEx _IsLeapYearEx;
static pfn_GetErrorString _GetErrorString;
static pfn_Shutdown _Shutdown;

int go_GetVersion() { return _GetVersion(); }
const char* go_GetVersionString() { return _GetVersionString(); }
void go_FreeString(char* s) { _FreeString(s); }
int go_SetTimezoneOffset(int sec) { return _SetTimezoneOffset(sec); }
FullTime go_GetLocalTime() { return _GetLocalTime(); }
int go_GetFormattedTimeBuf(char* buf, int sz) { return _GetFormattedTimeBuf(buf, sz); }
int go_GetWeekday(int y,int m,int d) { return _GetWeekday(y,m,d); }
long long go_GetUnixTimestamp() { return _GetUnixTimestamp(); }
int go_IsLeapYearEx(int y) { return _IsLeapYearEx(y); }
const char* go_GetErrorString(int code) { return _GetErrorString(code); }
void go_Shutdown() { _Shutdown(); }
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func printError(funcName string, errorCode int) {
	if errorCode != 0 {
		errStr := C.go_GetErrorString(C.int(errorCode))
		fmt.Printf("  [错误] %s 失败: 错误码 %d - %s\n", funcName, errorCode, C.GoString(errStr))
		C.go_FreeString((*C.char)(unsafe.Pointer(errStr)))
	}
}

func main() {
	dll := C.LoadLibraryA(C.CString("time_module.dll"))
	if dll == nil {
		fmt.Println("加载 time_module.dll 失败")
		return
	}
	defer C.FreeLibrary(dll)

	C._GetVersion = (C.pfn_GetVersion)(C.GetProcAddress(dll, C.CString("api_GetVersion")))
	C._GetVersionString = (C.pfn_GetVersionString)(C.GetProcAddress(dll, C.CString("api_GetVersionString")))
	C._FreeString = (C.pfn_FreeString)(C.GetProcAddress(dll, C.CString("api_FreeString")))
	C._SetTimezoneOffset = (C.pfn_SetTimezoneOffset)(C.GetProcAddress(dll, C.CString("api_SetTimezoneOffset")))
	C._GetLocalTime = (C.pfn_GetLocalTime)(C.GetProcAddress(dll, C.CString("api_GetLocalTime")))
	C._GetFormattedTimeBuf = (C.pfn_GetFormattedTimeBuf)(C.GetProcAddress(dll, C.CString("api_GetFormattedTimeBuf")))
	C._GetWeekday = (C.pfn_GetWeekday)(C.GetProcAddress(dll, C.CString("api_GetWeekday")))
	C._GetUnixTimestamp = (C.pfn_GetUnixTimestamp)(C.GetProcAddress(dll, C.CString("api_GetUnixTimestamp")))
	C._IsLeapYearEx = (C.pfn_IsLeapYearEx)(C.GetProcAddress(dll, C.CString("api_IsLeapYearEx")))
	C._GetErrorString = (C.pfn_GetErrorString)(C.GetProcAddress(dll, C.CString("api_GetErrorString")))
	C._Shutdown = (C.pfn_Shutdown)(C.GetProcAddress(dll, C.CString("api_Shutdown")))

	fmt.Println("========== time_module.dll 示例 (v0.2.18) ==========\n")

	// 1. 版本
	ver := int(C.go_GetVersion())
	fmt.Printf("[1] 版本信息\n")
	fmt.Printf("    DLL 版本: %d.%d.%d\n", ver>>16, (ver>>8)&0xFF, ver&0xFF)
	vs := C.go_GetVersionString()
	fmt.Printf("    版本字符串: %s\n", C.GoString(vs))
	C.go_FreeString((*C.char)(unsafe.Pointer(vs)))
	fmt.Println()

	// 2. 设置时区
	fmt.Printf("[2] 时区设置\n")
	ret := int(C.go_SetTimezoneOffset(28800))
	if ret != 0 {
		printError("SetTimezoneOffset", ret)
	} else {
		fmt.Println("    设置时区 UTC+8 成功")
	}

	// 3. 演示无效时区偏移
	fmt.Printf("\n[3] 无效时区偏移测试\n")
	ret = int(C.go_SetTimezoneOffset(50400))
	if ret == 18 {
		fmt.Println("    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)")
	}
	C.go_SetTimezoneOffset(28800)
	fmt.Println()

	// 4. 本地时间
	fmt.Printf("[4] 本地时间\n")
	ft := C.go_GetLocalTime()
	fmt.Printf("    本地时间: %04d-%02d-%02d %02d:%02d:%02d.%03d\n",
		ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms)
	fmt.Println()

	// 5. 格式化时间
	fmt.Printf("[5] 格式化时间\n")
	buf := make([]byte, 64)
	retLen := int(C.go_GetFormattedTimeBuf((*C.char)(unsafe.Pointer(&buf[0])), C.int(len(buf))))
	if retLen > 0 {
		fmt.Printf("    格式化时间: %s\n", string(buf[:retLen]))
	}
	fmt.Println()

	// 6. 星期
	fmt.Printf("[6] 星期信息\n")
	wd := int(C.go_GetWeekday(ft.year, ft.month, ft.day))
	weekdays := []string{"星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"}
	fmt.Printf("    星期: %s (%d)\n", weekdays[wd], wd)
	fmt.Println()

	// 7. Unix 时间戳
	fmt.Printf("[7] Unix 时间戳\n")
	ts := int64(C.go_GetUnixTimestamp())
	fmt.Printf("    Unix时间戳: %d 秒\n", ts)
	fmt.Println()

	// 8. 闰年
	fmt.Printf("[8] 闰年判断\n")
	leap := int(C.go_IsLeapYearEx(2000))
	fmt.Printf("    2000年是闰年: %t\n", leap != 0)
	fmt.Println()

	// 9. 错误字符串演示
	fmt.Printf("[9] 错误字符串演示\n")
	errStr := C.go_GetErrorString(18)
	fmt.Printf("    错误码 18: %s\n", C.GoString(errStr))
	C.go_FreeString((*C.char)(unsafe.Pointer(errStr)))
	errStr = C.go_GetErrorString(19)
	fmt.Printf("    错误码 19: %s\n", C.GoString(errStr))
	C.go_FreeString((*C.char)(unsafe.Pointer(errStr)))
	fmt.Println()

	// 10. 关闭
	fmt.Printf("[10] 关闭 DLL\n")
	C.go_Shutdown()
	fmt.Println("    完成")
}
