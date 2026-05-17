/**
 * time_module.dll C# 调用示例 (P/Invoke)
 * 最低支持: .NET Framework 4.0 / .NET 6
 * 编译: csc TestTime.cs
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
    public static extern IntPtr api_GetFormattedTime();

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_GetFormattedTimeBuf(byte[] buf, int bufSize);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_GetWeekday(int year, int month, int day);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern long api_GetUnixTimestamp();

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int api_IsLeapYearEx(int year);

    [DllImport("time_module.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void api_Shutdown();

    static void Main()
    {
        // 版本
        int ver = api_GetVersion();
        Console.WriteLine($"DLL 版本: {ver >> 16}.{(ver >> 8) & 0xFF}.{ver & 0xFF}");

        IntPtr ptr = api_GetVersionString();
        string verStr = Marshal.PtrToStringAnsi(ptr);
        Console.WriteLine($"版本字符串: {verStr}");
        api_FreeString(ptr);

        // 设置时区
        api_SetTimezoneOffset(28800);

        // 本地时间
        FullTime ft = api_GetLocalTime();
        Console.WriteLine($"本地时间: {ft.year:D4}-{ft.month:D2}-{ft.day:D2} {ft.hour:D2}:{ft.minute:D2}:{ft.second:D2}.{ft.ms:D3}");

        // 格式化时间（缓冲区）
        byte[] buf = new byte[64];
        int len = api_GetFormattedTimeBuf(buf, buf.Length);
        if (len > 0)
            Console.WriteLine($"格式化时间: {Encoding.UTF8.GetString(buf, 0, len)}");

        // 星期
        int wd = api_GetWeekday(ft.year, ft.month, ft.day);
        Console.WriteLine($"星期: {wd} (0=星期日)");

        // Unix 时间戳
        long ts = api_GetUnixTimestamp();
        Console.WriteLine($"Unix时间戳: {ts} 秒");

        // 闰年
        int leap = api_IsLeapYearEx(2000);
        Console.WriteLine($"2000年是闰年: {(leap != 0 ? "是" : "否")}");

        // 关闭
        api_Shutdown();
    }
}