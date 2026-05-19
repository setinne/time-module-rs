/**
 * time_module.dll C# 调用示例 (P/Invoke)
 * 最低支持: .NET Framework 4.0 / .NET 6
 * 编译: csc TestTime.cs
 * 运行: TestTime.exe
 * 
 * v0.2.18 更新: 添加错误码处理和错误字符串演示
 */

using System;
using System.Runtime.InteropServices;
using System.Text;

class TimeModule
{
    // 结构体定义
    [StructLayout(LayoutKind.Sequential)]
    public struct FullTime
    {
        public int year, month, day, hour, minute, second, ms, us;
    }

    // DLL 导入
    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_GetVersion();

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr api_GetVersionString();

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void api_FreeString(IntPtr ptr);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_SetTimezoneOffset(int seconds);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern FullTime api_GetLocalTime();

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_GetFormattedTimeBuf(byte[] buf, int bufSize);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_GetWeekday(int year, int month, int day);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern long api_GetUnixTimestamp();

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_IsLeapYearEx(int year);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr api_GetErrorString(int code);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void api_Shutdown();

    static void PrintError(string funcName, int errorCode)
    {
        if (errorCode != 0)
        {
            IntPtr ptr = api_GetErrorString(errorCode);
            string err = Marshal.PtrToStringAnsi(ptr);
            Console.WriteLine($"  [错误] {funcName} 失败: 错误码 {errorCode} - {err}");
            api_FreeString(ptr);
        }
    }

    static void Main()
    {
        Console.WriteLine("========== time_module.dll 示例 (v0.2.18) ==========\n");

        // 1. 版本
        int ver = api_GetVersion();
        Console.WriteLine("[1] 版本信息");
        Console.WriteLine($"    DLL 版本: {ver >> 16}.{(ver >> 8) & 0xFF}.{ver & 0xFF}");

        IntPtr ptr = api_GetVersionString();
        string verStr = Marshal.PtrToStringAnsi(ptr);
        Console.WriteLine($"    版本字符串: {verStr}");
        api_FreeString(ptr);
        Console.WriteLine();

        // 2. 设置时区
        Console.WriteLine("[2] 时区设置");
        int ret = api_SetTimezoneOffset(28800);
        if (ret != 0)
        {
            PrintError("SetTimezoneOffset", ret);
        }
        else
        {
            Console.WriteLine("    设置时区 UTC+8 成功");
        }

        // 3. 演示无效时区偏移
        Console.WriteLine("\n[3] 无效时区偏移测试");
        ret = api_SetTimezoneOffset(50400);  // UTC+14，超出范围
        if (ret == 18)
        {
            Console.WriteLine("    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)");
        }
        api_SetTimezoneOffset(28800);  // 恢复
        Console.WriteLine();

        // 4. 本地时间
        Console.WriteLine("[4] 本地时间");
        FullTime ft = api_GetLocalTime();
        Console.WriteLine($"    本地时间: {ft.year:D4}-{ft.month:D2}-{ft.day:D2} {ft.hour:D2}:{ft.minute:D2}:{ft.second:D2}.{ft.ms:D3}");
        Console.WriteLine();

        // 5. 格式化时间
        Console.WriteLine("[5] 格式化时间");
        byte[] buf = new byte[64];
        int len = api_GetFormattedTimeBuf(buf, buf.Length);
        if (len > 0)
            Console.WriteLine($"    格式化时间: {Encoding.UTF8.GetString(buf, 0, len)}");
        Console.WriteLine();

        // 6. 星期
        Console.WriteLine("[6] 星期信息");
        int wd = api_GetWeekday(ft.year, ft.month, ft.day);
        string[] weekdays = { "星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六" };
        Console.WriteLine($"    星期: {weekdays[wd]} ({wd})");
        Console.WriteLine();

        // 7. Unix 时间戳
        Console.WriteLine("[7] Unix 时间戳");
        long ts = api_GetUnixTimestamp();
        Console.WriteLine($"    Unix时间戳: {ts} 秒");
        Console.WriteLine();

        // 8. 闰年
        Console.WriteLine("[8] 闰年判断");
        int leap = api_IsLeapYearEx(2000);
        Console.WriteLine($"    2000年是闰年: {(leap != 0 ? "是" : "否")}");
        Console.WriteLine();

        // 9. 错误字符串演示
        Console.WriteLine("[9] 错误字符串演示");
        IntPtr errPtr = api_GetErrorString(18);
        string errStr = Marshal.PtrToStringAnsi(errPtr);
        Console.WriteLine($"    错误码 18: {errStr}");
        api_FreeString(errPtr);
        errPtr = api_GetErrorString(19);
        errStr = Marshal.PtrToStringAnsi(errPtr);
        Console.WriteLine($"    错误码 19: {errStr}");
        api_FreeString(errPtr);
        Console.WriteLine();

        // 10. 关闭
        Console.WriteLine("[10] 关闭 DLL");
        api_Shutdown();
        Console.WriteLine("    完成");
    }
}