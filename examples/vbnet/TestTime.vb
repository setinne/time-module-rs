' time_module.dll VB.NET 调用示例 (P/Invoke)
' 最低支持: .NET Framework 4.0
' 编译: vbc TestTime.vb
' 运行: TestTime.exe
'
' v0.2.18 更新: 添加错误码处理和错误字符串演示

Imports System.Runtime.InteropServices
Imports System.Text

Module TestTime

    <StructLayout(LayoutKind.Sequential)>
    Public Structure FullTime
        Public year As Integer
        Public month As Integer
        Public day As Integer
        Public hour As Integer
        Public minute As Integer
        Public second As Integer
        Public ms As Integer
        Public us As Integer
    End Structure

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_GetVersion() As Integer
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_GetVersionString() As IntPtr
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Sub api_FreeString(ByVal ptr As IntPtr)
    End Sub

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_SetTimezoneOffset(ByVal seconds As Integer) As Integer
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_GetLocalTime() As FullTime
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_GetFormattedTimeBuf(ByVal buf As Byte(), ByVal bufSize As Integer) As Integer
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_GetWeekday(ByVal year As Integer, ByVal month As Integer, ByVal day As Integer) As Integer
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_GetUnixTimestamp() As Long
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_IsLeapYearEx(ByVal year As Integer) As Integer
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Function api_GetErrorString(ByVal code As Integer) As IntPtr
    End Function

    <DllImport("time_module.dll", CallingConvention:=CallingConvention.Cdecl)>
    Public Sub api_Shutdown()
    End Sub

    Sub PrintError(funcName As String, errorCode As Integer)
        If errorCode <> 0 Then
            Dim ptr As IntPtr = api_GetErrorString(errorCode)
            Dim err As String = Marshal.PtrToStringAnsi(ptr)
            Console.WriteLine($"  [错误] {funcName} 失败: 错误码 {errorCode} - {err}")
            api_FreeString(ptr)
        End If
    End Sub

    Sub Main()
        Console.WriteLine("========== time_module.dll 示例 (v0.2.18) ==========")
        Console.WriteLine()

        ' 1. 版本
        Dim ver As Integer = api_GetVersion()
        Console.WriteLine("[1] 版本信息")
        Console.WriteLine($"    DLL 版本: {ver >> 16}.{(ver >> 8) And &HFF}.{ver And &HFF}")

        Dim ptr As IntPtr = api_GetVersionString()
        Dim verStr As String = Marshal.PtrToStringAnsi(ptr)
        Console.WriteLine($"    版本字符串: {verStr}")
        api_FreeString(ptr)
        Console.WriteLine()

        ' 2. 设置时区
        Console.WriteLine("[2] 时区设置")
        Dim ret As Integer = api_SetTimezoneOffset(28800)
        If ret <> 0 Then
            PrintError("SetTimezoneOffset", ret)
        Else
            Console.WriteLine("    设置时区 UTC+8 成功")
        End If

        ' 3. 演示无效时区偏移
        Console.WriteLine()
        Console.WriteLine("[3] 无效时区偏移测试")
        ret = api_SetTimezoneOffset(50400)
        If ret = 18 Then
            Console.WriteLine("    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)")
        End If
        api_SetTimezoneOffset(28800)
        Console.WriteLine()

        ' 4. 本地时间
        Console.WriteLine("[4] 本地时间")
        Dim ft As FullTime = api_GetLocalTime()
        Console.WriteLine($"    本地时间: {ft.year:D4}-{ft.month:D2}-{ft.day:D2} {ft.hour:D2}:{ft.minute:D2}:{ft.second:D2}.{ft.ms:D3}")
        Console.WriteLine()

        ' 5. 格式化时间
        Console.WriteLine("[5] 格式化时间")
        Dim buf(63) As Byte
        Dim len As Integer = api_GetFormattedTimeBuf(buf, buf.Length)
        If len > 0 Then
            Console.WriteLine($"    格式化时间: {Encoding.UTF8.GetString(buf, 0, len)}")
        End If
        Console.WriteLine()

        ' 6. 星期
        Console.WriteLine("[6] 星期信息")
        Dim wd As Integer = api_GetWeekday(ft.year, ft.month, ft.day)
        Dim weekdays As String() = {"星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"}
        Console.WriteLine($"    星期: {weekdays(wd)} ({wd})")
        Console.WriteLine()

        ' 7. Unix 时间戳
        Console.WriteLine("[7] Unix 时间戳")
        Dim ts As Long = api_GetUnixTimestamp()
        Console.WriteLine($"    Unix时间戳: {ts} 秒")
        Console.WriteLine()

        ' 8. 闰年
        Console.WriteLine("[8] 闰年判断")
        Dim leap As Integer = api_IsLeapYearEx(2000)
        Console.WriteLine($"    2000年是闰年: {If(leap <> 0, "是", "否")}")
        Console.WriteLine()

        ' 9. 错误字符串演示
        Console.WriteLine("[9] 错误字符串演示")
        Dim errPtr As IntPtr = api_GetErrorString(18)
        Console.WriteLine($"    错误码 18: {Marshal.PtrToStringAnsi(errPtr)}")
        api_FreeString(errPtr)
        errPtr = api_GetErrorString(19)
        Console.WriteLine($"    错误码 19: {Marshal.PtrToStringAnsi(errPtr)}")
        api_FreeString(errPtr)
        Console.WriteLine()

        ' 10. 关闭
        Console.WriteLine("[10] 关闭 DLL")
        api_Shutdown()
        Console.WriteLine("    完成")
    End Sub
End Module