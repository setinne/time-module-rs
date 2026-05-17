/**
 * time_module.dll C++ 调用示例
 * 最低支持: C++11 (MSVC / MinGW)
 * 编译: g++ test_time.cpp -o test_time.exe
 */

#include <windows.h>
#include <iostream>
#include <memory>

struct FullTime {
    int year, month, day, hour, minute, second, ms, us;
};

// 包装类
class TimeModule {
    HMODULE dll;
public:
    TimeModule() : dll(nullptr) {}
    ~TimeModule() { if (dll) FreeLibrary(dll); }

    bool Load() {
        dll = LoadLibraryA("time_module.dll");
        return dll != nullptr;
    }

    template<typename T>
    T GetProc(const char* name) {
        return reinterpret_cast<T>(GetProcAddress(dll, name));
    }
};

int main() {
    TimeModule tm;
    if (!tm.Load()) {
        std::cerr << "加载 DLL 失败" << std::endl;
        return 1;
    }

    auto GetVersion = tm.GetProc<int(*)()>("api_GetVersion");
    auto GetVersionString = tm.GetProc<const char*(*)()>("api_GetVersionString");
    auto FreeString = tm.GetProc<void(*)(char*)>("api_FreeString");
    auto SetOffset = tm.GetProc<int(*)(int)>("api_SetTimezoneOffset");
    auto GetLocalTime = tm.GetProc<FullTime(*)()>("api_GetLocalTime");
    auto GetFormattedTime = tm.GetProc<const char*(*)()>("api_GetFormattedTime");
    auto GetFormattedTimeBuf = tm.GetProc<int(*)(char*,int)>("api_GetFormattedTimeBuf");
    auto GetWeekday = tm.GetProc<int(*)(int,int,int)>("api_GetWeekday");
    auto GetUnixTimestamp = tm.GetProc<long long(*)()>("api_GetUnixTimestamp");
    auto IsLeapYearEx = tm.GetProc<int(*)(int)>("api_IsLeapYearEx");
    auto Shutdown = tm.GetProc<void(*)()>("api_Shutdown");

    // 版本
    int ver = GetVersion();
    std::cout << "版本: " << (ver>>16) << "." << ((ver>>8)&0xFF) << "." << (ver&0xFF) << std::endl;
    const char* vs = GetVersionString();
    std::cout << "版本字符串: " << vs << std::endl;
    FreeString((char*)vs);

    SetOffset(28800);
    FullTime ft = GetLocalTime();
    std::cout << "本地时间: " << ft.year << "-" << ft.month << "-" << ft.day
                << " " << ft.hour << ":" << ft.minute << ":" << ft.second << std::endl;

    char buf[64];
    GetFormattedTimeBuf(buf, sizeof(buf));
    std::cout << "格式化时间: " << buf << std::endl;

    int wd = GetWeekday(ft.year, ft.month, ft.day);
    std::cout << "星期: " << wd << std::endl;

    std::cout << "Unix时间戳: " << GetUnixTimestamp() << std::endl;
    std::cout << "2000年是闰年: " << (IsLeapYearEx(2000) ? "是" : "否") << std::endl;

    Shutdown();
    return 0;
}