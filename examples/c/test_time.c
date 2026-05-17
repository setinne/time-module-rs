/**
 * time_module.dll C 语言调用示例
 * 最低支持: C89 (MSVC / MinGW)
 * 编译: gcc test_time.c -o test_time.exe
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
typedef void (*pfn_Shutdown)(void);

int main() {
    HMODULE dll = LoadLibraryA("time_module.dll");
    if (!dll) {
        printf("错误: 无法加载 time_module.dll (错误码: %d)\n", GetLastError());
        return 1;
    }

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
    pfn_Shutdown Shutdown = (pfn_Shutdown)GetProcAddress(dll, "api_Shutdown");

    if (!GetVersion || !GetVersionString || !FreeString || !SetTimezoneOffset ||
        !GetLocalTime || !GetFormattedTime || !GetFormattedTimeBuf || !GetWeekday ||
        !GetUnixTimestamp || !IsLeapYearEx || !Shutdown) {
        printf("错误: 获取函数地址失败\n");
        FreeLibrary(dll);
        return 1;
    }

    // 1. 版本信息
    int ver = GetVersion();
    printf("DLL 版本: %d.%d.%d (0x%06X)\n", ver>>16, (ver>>8)&0xFF, ver&0xFF, ver);
    const char* verStr = GetVersionString();
    printf("版本字符串: %s\n", verStr);
    FreeString((char*)verStr);

    // 2. 设置时区为 UTC+8
    SetTimezoneOffset(28800);

    // 3. 获取本地时间
    FullTime ft = GetLocalTime();
    printf("本地时间: %04d-%02d-%02d %02d:%02d:%02d.%03d\n",
            ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms);

    // 4. 格式化时间（动态分配版本）
    const char* fmt = GetFormattedTime();
    printf("格式化时间(动态): %s\n", fmt);
    FreeString((char*)fmt);

    // 5. 格式化时间（缓冲区版本，更安全）
    char buf[64];
    int len = GetFormattedTimeBuf(buf, sizeof(buf));
    if (len > 0) {
        printf("格式化时间(缓冲): %s\n", buf);
    } else {
        printf("缓冲区版本失败\n");
    }

    // 6. 星期
    int wday = GetWeekday(ft.year, ft.month, ft.day);
    printf("星期: %d (0=星期日)\n", wday);

    // 7. Unix 时间戳
    long long ts = GetUnixTimestamp();
    printf("Unix时间戳: %lld 秒\n", ts);

    // 8. 闰年判断
    int leap = IsLeapYearEx(2000);
    printf("2000年是闰年: %s\n", leap ? "是" : "否");

    // 9. 关闭 DLL（释放后台线程）
    Shutdown();
    FreeLibrary(dll);
    return 0;
}