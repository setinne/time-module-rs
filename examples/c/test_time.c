/**
 * time_module.dll C 语言调用示例
 * 最低支持: C89 (MSVC / MinGW)
 * 编译: gcc test_time.c -o test_time.exe
 * 
 * v0.2.18 更新: 添加错误码处理和错误字符串演示
 */

#include <windows.h>
#include <stdio.h>

// 结构体定义
typedef struct {
    int year, month, day, hour, minute, second, ms, us;
} FullTime;

// 函数指针类型
typedef int (*pfn_GetVersion)(void);
typedef const char* (*pfn_GetVersionString)(void);
typedef void (*pfn_FreeString)(char*);
typedef int (*pfn_SetTimezoneOffset)(int);
typedef FullTime (*pfn_GetLocalTime)(void);
typedef const char* (*pfn_GetFormattedTime)(void);
typedef int (*pfn_GetFormattedTimeBuf)(char*, int);
typedef int (*pfn_GetWeekday)(int,int,int);
typedef long long (*pfn_GetUnixTimestamp)(void);
typedef int (*pfn_IsLeapYearEx)(int);
typedef const char* (*pfn_GetErrorString)(int);
typedef void (*pfn_Shutdown)(void);

void print_error(const char* func_name, int error_code, pfn_GetErrorString GetErrorString, pfn_FreeString FreeString) {
    if (error_code != 0) {
        const char* err_str = GetErrorString(error_code);
        printf("  [错误] %s 失败: 错误码 %d - %s\n", func_name, error_code, err_str);
        FreeString((char*)err_str);
    }
}

int main() {
    HMODULE dll = LoadLibraryA("time_module.dll");
    if (!dll) {
        printf("错误: 无法加载 time_module.dll (错误码: %d)\n", GetLastError());
        return 1;
    }

    // 获取函数指针
    pfn_GetVersion GetVersion = (pfn_GetVersion)GetProcAddress(dll, "api_GetVersion");
    pfn_GetVersionString GetVersionString = (pfn_GetVersionString)GetProcAddress(dll, "api_GetVersionString");
    pfn_FreeString FreeString = (pfn_FreeString)GetProcAddress(dll, "api_FreeString");
    pfn_SetTimezoneOffset SetTimezoneOffset = (pfn_SetTimezoneOffset)GetProcAddress(dll, "api_SetTimezoneOffset");
    pfn_GetLocalTime GetLocalTime = (pfn_GetLocalTime)GetProcAddress(dll, "api_GetLocalTime");
    pfn_GetFormattedTime GetFormattedTime = (pfn_GetFormattedTime)GetProcAddress(dll, "api_GetFormattedTime");
    pfn_GetFormattedTimeBuf GetFormattedTimeBuf = (pfn_GetFormattedTimeBuf)GetProcAddress(dll, "api_GetFormattedTimeBuf");
    pfn_GetWeekday GetWeekday = (pfn_GetWeekday)GetProcAddress(dll, "api_GetWeekday");
    pfn_GetUnixTimestamp GetUnixTimestamp = (pfn_GetUnixTimestamp)GetProcAddress(dll, "api_GetUnixTimestamp");
    pfn_IsLeapYearEx IsLeapYearEx = (pfn_IsLeapYearEx)GetProcAddress(dll, "api_IsLeapYearEx");
    pfn_GetErrorString GetErrorString = (pfn_GetErrorString)GetProcAddress(dll, "api_GetErrorString");
    pfn_Shutdown Shutdown = (pfn_Shutdown)GetProcAddress(dll, "api_Shutdown");

    if (!GetVersion || !GetVersionString || !FreeString || !SetTimezoneOffset ||
        !GetLocalTime || !GetFormattedTime || !GetFormattedTimeBuf || !GetWeekday ||
        !GetUnixTimestamp || !IsLeapYearEx || !GetErrorString || !Shutdown) {
        printf("错误: 获取函数地址失败\n");
        FreeLibrary(dll);
        return 1;
    }

    printf("========== time_module.dll 示例 (v0.2.18) ==========\n\n");

    // 1. 版本信息
    int ver = GetVersion();
    printf("[1] 版本信息\n");
    printf("    DLL 版本: %d.%d.%d (0x%06X)\n", ver>>16, (ver>>8)&0xFF, ver&0xFF, ver);
    const char* verStr = GetVersionString();
    printf("    版本字符串: %s\n", verStr);
    FreeString((char*)verStr);
    printf("\n");

    // 2. 设置时区为 UTC+8 (有效范围内)
    printf("[2] 时区设置\n");
    int ret = SetTimezoneOffset(28800);
    if (ret != 0) {
        print_error("SetTimezoneOffset", ret, GetErrorString, FreeString);
    } else {
        printf("    设置时区 UTC+8 成功\n");
    }
    
    // 3. 演示无效时区偏移（返回错误码 18: TimezoneOffsetOutOfRange）
    printf("\n[3] 无效时区偏移测试\n");
    ret = SetTimezoneOffset(50400);  // UTC+14，超出有效范围
    if (ret == 18) {
        printf("    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)\n");
    } else {
        print_error("SetTimezoneOffset(50400)", ret, GetErrorString, FreeString);
    }
    // 恢复有效偏移
    SetTimezoneOffset(28800);
    printf("\n");

    // 4. 获取本地时间
    printf("[4] 本地时间\n");
    FullTime ft = GetLocalTime();
    printf("    本地时间: %04d-%02d-%02d %02d:%02d:%02d.%03d\n",
           ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms);
    printf("\n");

    // 5. 格式化时间（动态分配版本）
    printf("[5] 格式化时间\n");
    const char* fmt = GetFormattedTime();
    printf("    动态分配: %s\n", fmt);
    FreeString((char*)fmt);

    // 6. 格式化时间（缓冲区版本，更安全）
    char buf[64];
    int len = GetFormattedTimeBuf(buf, sizeof(buf));
    if (len > 0) {
        printf("    缓冲版本: %s\n", buf);
    } else {
        printf("    缓冲区版本失败\n");
    }
    printf("\n");

    // 7. 星期
    printf("[6] 星期信息\n");
    int wday = GetWeekday(ft.year, ft.month, ft.day);
    const char* weekdays[] = {"星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"};
    printf("    星期: %s (%d)\n", weekdays[wday], wday);
    printf("\n");

    // 8. Unix 时间戳
    printf("[7] Unix 时间戳\n");
    long long ts = GetUnixTimestamp();
    printf("    Unix时间戳: %lld 秒\n", ts);
    printf("\n");

    // 9. 闰年判断
    printf("[8] 闰年判断\n");
    int leap = IsLeapYearEx(2000);
    printf("    2000年是闰年: %s\n", leap ? "是" : "否");
    printf("\n");

    // 10. 错误字符串演示
    printf("[9] 错误字符串演示\n");
    const char* err_str = GetErrorString(18);
    printf("    错误码 18: %s\n", err_str);
    FreeString((char*)err_str);
    err_str = GetErrorString(19);
    printf("    错误码 19: %s\n", err_str);
    FreeString((char*)err_str);
    printf("\n");

    // 11. 关闭 DLL
    printf("[10] 关闭 DLL\n");
    Shutdown();
    FreeLibrary(dll);
    printf("    完成\n");

    return 0;
}