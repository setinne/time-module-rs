' time_module.dll VB.NET 调用示例 (P/Invoke)
' 最低支持: .NET Framework 4.0
' 编译: vbc TestTime.vb

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
    Public Function api_GetFormattedTime() As IntPtr
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
    Public Sub api_Shutdown()
    End Sub

    Sub Main()
        ' 版本
        Dim ver As Integer = api_GetVersion()
        Console.WriteLine($"DLL 版本: {ver >> 16}.{(ver >> 8) And &HFF}.{ver And &HFF}")

        Dim ptr As IntPtr = api_GetVersionString()
        Dim verStr As String = Marshal.PtrToStringAnsi(ptr)
        Console.WriteLine($"版本字符串: {verStr}")
        api_FreeString(ptr)

        ' 设置时区
        api_SetTimezoneOffset(28800)

        ' 本地时间
        Dim ft As FullTime = api_GetLocalTime()
        Console.WriteLine($"本地时间: {ft.year:D4}-{ft.month:D2}-{ft.day:D2} {ft.hour:D2}:{ft.minute:D2}:{ft.second:D2}.{ft.ms:D3}")

        ' 格式化时间（缓冲区）
        Dim buf(63) As Byte
        Dim len As Integer = api_GetFormattedTimeBuf(buf, buf.Length)
        If len > 0 Then
            Console.WriteLine($"格式化时间: {Encoding.UTF8.GetString(buf, 0, len)}")
        End If

        ' 星期
        Dim wd As Integer = api_GetWeekday(ft.year, ft.month, ft.day)
        Console.WriteLine($"星期: {wd} (0=星期日)")

        ' Unix 时间戳
        Dim ts As Long = api_GetUnixTimestamp()
        Console.WriteLine($"Unix时间戳: {ts} 秒")

        ' 闰年
        Dim leap As Integer = api_IsLeapYearEx(2000)
        Console.WriteLine($"2000年是闰年: {If(leap <> 0, "是", "否")}")

        ' 关闭
        api_Shutdown()
    End Sub
End Module