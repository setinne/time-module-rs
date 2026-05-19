/**
 * time_module.dll C++ 调用示例
 * 最低支持: C++11 (MSVC / MinGW)
 * 编译: g++ test_time.cpp -o test_time.exe
 * 
 * v0.2.18 更新: 添加错误码处理和错误字符串演示
 */

#include <windows.h>
#include <iostream>
#include <memory>
#include <string>

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

    void PrintError(const char* func_name, int error_code) {
        if (error_code != 0) {
            auto GetErrorString = GetProc<const char*(*)(int)>("api_GetErrorString");
            auto FreeString = GetProc<void(*)(char*)>("api_FreeString");
            if (GetErrorString && FreeString) {
                const char* err_str = GetErrorString(error_code);
                std::cerr << "  [错误] " << func_name << " 失败: 错误码 " << error_code 
                          << " - " << err_str << std::endl;
                FreeString((char*)err_str);
            }
        }
    }
};

int main() {
    TimeModule tm;
    if (!tm.Load()) {
        std::cerr << "加载 time_module.dll 失败" << std::endl;
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
    auto GetErrorString = tm.GetProc<const char*(*)(int)>("api_GetErrorString");
    auto Shutdown = tm.GetProc<void(*)()>("api_Shutdown");

    std::cout << "========== time_module.dll 示例 (v0.2.18) ==========" << std::endl << std::endl;

    // 1. 版本
    int ver = GetVersion();
    std::cout << "[1] 版本信息" << std::endl;
    std::cout << "    DLL 版本: " << (ver>>16) << "." << ((ver>>8)&0xFF) << "." << (ver&0xFF) << std::endl;
    const char* vs = GetVersionString();
    std::cout << "    版本字符串: " << vs << std::endl;
    FreeString((char*)vs);
    std::cout << std::endl;

    // 2. 设置时区
    std::cout << "[2] 时区设置" << std::endl;
    int ret = SetOffset(28800);
    if (ret != 0) {
        const char* err_str = GetErrorString(ret);
        std::cerr << "    设置时区失败: 错误码 " << ret << " - " << err_str << std::endl;
        FreeString((char*)err_str);
    } else {
        std::cout << "    设置时区 UTC+8 成功" << std::endl;
    }

    // 3. 演示无效时区偏移
    std::cout << std::endl << "[3] 无效时区偏移测试" << std::endl;
    ret = SetOffset(50400);  // UTC+14，超出范围
    if (ret == 18) {
        std::cout << "    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)" << std::endl;
    }
    SetOffset(28800);  // 恢复
    std::cout << std::endl;

    // 4. 本地时间
    std::cout << "[4] 本地时间" << std::endl;
    FullTime ft = GetLocalTime();
    std::cout << "    本地时间: " << ft.year << "-" << ft.month << "-" << ft.day
              << " " << ft.hour << ":" << ft.minute << ":" << ft.second << std::endl;
    std::cout << std::endl;

    // 5. 格式化时间
    std::cout << "[5] 格式化时间" << std::endl;
    char buf[64];
    GetFormattedTimeBuf(buf, sizeof(buf));
    std::cout << "    格式化时间: " << buf << std::endl;
    std::cout << std::endl;

    // 6. 星期
    std::cout << "[6] 星期信息" << std::endl;
    int wd = GetWeekday(ft.year, ft.month, ft.day);
    const char* weekdays[] = {"星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"};
    std::cout << "    星期: " << weekdays[wd] << " (" << wd << ")" << std::endl;
    std::cout << std::endl;

    // 7. Unix 时间戳
    std::cout << "[7] Unix 时间戳" << std::endl;
    std::cout << "    Unix时间戳: " << GetUnixTimestamp() << " 秒" << std::endl;
    std::cout << std::endl;

    // 8. 闰年
    std::cout << "[8] 闰年判断" << std::endl;
    std::cout << "    2000年是闰年: " << (IsLeapYearEx(2000) ? "是" : "否") << std::endl;
    std::cout << std::endl;

    // 9. 错误字符串演示
    std::cout << "[9] 错误字符串演示" << std::endl;
    const char* err_str = GetErrorString(18);
    std::cout << "    错误码 18: " << err_str << std::endl;
    FreeString((char*)err_str);
    err_str = GetErrorString(19);
    std::cout << "    错误码 19: " << err_str << std::endl;
    FreeString((char*)err_str);
    std::cout << std::endl;

    // 10. 关闭
    std::cout << "[10] 关闭 DLL" << std::endl;
    Shutdown();
    std::cout << "    完成" << std::endl;

    return 0;
}