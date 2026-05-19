program TestTime;

{$APPTYPE CONSOLE}

(*
  time_module.dll Delphi 调用示例
  最低支持: Delphi 7 / Delphi 2007
  编译: 在 Delphi IDE 中打开并编译

  v0.2.18 更新: 添加错误码处理和错误字符串演示
*)

uses
  Windows,
  SysUtils;

type
  FullTime = packed record
    year, month, day, hour, minute, second, ms, us: Integer;
  end;

  // 函数指针类型
  TGetVersion = function: Integer; cdecl;
  TGetVersionString = function: PAnsiChar; cdecl;
  TFreeString = procedure(ptr: PAnsiChar); cdecl;
  TSetTimezoneOffset = function(sec: Integer): Integer; cdecl;
  TGetLocalTime = function: FullTime; cdecl;
  TGetFormattedTimeBuf = function(buf: PAnsiChar; bufSize: Integer): Integer; cdecl;
  TGetWeekday = function(year, month, day: Integer): Integer; cdecl;
  TGetUnixTimestamp = function: Int64; cdecl;
  TIsLeapYearEx = function(year: Integer): Integer; cdecl;
  TGetErrorString = function(code: Integer): PAnsiChar; cdecl;
  TShutdown = procedure; cdecl;

  procedure PrintError(funcName: string; errorCode: Integer; GetErrorString: TGetErrorString; FreeString: TFreeString);
  var
    errStr: PAnsiChar;
  begin
    if errorCode <> 0 then
    begin
      errStr := GetErrorString(errorCode);
      Writeln(Format('  [错误] %s 失败: 错误码 %d - %s', [funcName, errorCode, errStr]));
      FreeString(errStr);
    end;
  end;

var
  hLib: THandle;
  GetVersion: TGetVersion;
  GetVersionString: TGetVersionString;
  FreeString: TFreeString;
  SetTimezoneOffset: TSetTimezoneOffset;
  GetLocalTime: TGetLocalTime;
  GetFormattedTimeBuf: TGetFormattedTimeBuf;
  GetWeekday: TGetWeekday;
  GetUnixTimestamp: TGetUnixTimestamp;
  IsLeapYearEx: TIsLeapYearEx;
  GetErrorString: TGetErrorString;
  Shutdown: TShutdown;
  ft: FullTime;
  ver, ret, wd, leap: Integer;
  p: PAnsiChar;
  buf: array[0..63] of AnsiChar;
  len: Integer;
  ts: Int64;
  weekdays: array[0..6] of string;
  i: Integer;
begin
  weekdays[0] := '星期日';
  weekdays[1] := '星期一';
  weekdays[2] := '星期二';
  weekdays[3] := '星期三';
  weekdays[4] := '星期四';
  weekdays[5] := '星期五';
  weekdays[6] := '星期六';

  hLib := LoadLibrary('time_module.dll');
  if hLib = 0 then
  begin
    Writeln('无法加载 time_module.dll');
    Exit;
  end;

  GetVersion := GetProcAddress(hLib, 'api_GetVersion');
  GetVersionString := GetProcAddress(hLib, 'api_GetVersionString');
  FreeString := GetProcAddress(hLib, 'api_FreeString');
  SetTimezoneOffset := GetProcAddress(hLib, 'api_SetTimezoneOffset');
  GetLocalTime := GetProcAddress(hLib, 'api_GetLocalTime');
  GetFormattedTimeBuf := GetProcAddress(hLib, 'api_GetFormattedTimeBuf');
  GetWeekday := GetProcAddress(hLib, 'api_GetWeekday');
  GetUnixTimestamp := GetProcAddress(hLib, 'api_GetUnixTimestamp');
  IsLeapYearEx := GetProcAddress(hLib, 'api_IsLeapYearEx');
  GetErrorString := GetProcAddress(hLib, 'api_GetErrorString');
  Shutdown := GetProcAddress(hLib, 'api_Shutdown');

  Writeln('========== time_module.dll 示例 (v0.2.18) ==========');
  Writeln;

  // 1. 版本
  ver := GetVersion();
  Writeln('[1] 版本信息');
  Writeln(Format('    DLL 版本: %d.%d.%d', [ver shr 16, (ver shr 8) and $FF, ver and $FF]));
  p := GetVersionString();
  Writeln('    版本字符串: ', p);
  FreeString(p);
  Writeln;

  // 2. 设置时区
  Writeln('[2] 时区设置');
  ret := SetTimezoneOffset(28800);
  if ret <> 0 then
    PrintError('SetTimezoneOffset', ret, GetErrorString, FreeString)
  else
    Writeln('    设置时区 UTC+8 成功');

  // 3. 演示无效时区偏移
  Writeln;
  Writeln('[3] 无效时区偏移测试');
  ret := SetTimezoneOffset(50400);
  if ret = 18 then
    Writeln('    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)');
  SetTimezoneOffset(28800);
  Writeln;

  // 4. 本地时间
  Writeln('[4] 本地时间');
  ft := GetLocalTime();
  Writeln(Format('    本地时间: %.4d-%.2d-%.2d %.2d:%.2d:%.2d.%.3d',
    [ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms]));
  Writeln;

  // 5. 格式化时间
  Writeln('[5] 格式化时间');
  len := GetFormattedTimeBuf(buf, SizeOf(buf));
  if len > 0 then
    Writeln('    格式化时间: ', buf);
  Writeln;

  // 6. 星期
  Writeln('[6] 星期信息');
  wd := GetWeekday(ft.year, ft.month, ft.day);
  if (wd >= 0) and (wd <= 6) then
    Writeln(Format('    星期: %s (%d)', [weekdays[wd], wd]))
  else
    Writeln('    星期: 无效');
  Writeln;

  // 7. Unix 时间戳
  Writeln('[7] Unix 时间戳');
  ts := GetUnixTimestamp();
  Writeln(Format('    Unix时间戳: %d 秒', [ts]));
  Writeln;

  // 8. 闰年
  Writeln('[8] 闰年判断');
  leap := IsLeapYearEx(2000);
  Writeln(Format('    2000年是闰年: %s', [IfThen(leap <> 0, '是', '否')]));
  Writeln;

  // 9. 错误字符串演示
  Writeln('[9] 错误字符串演示');
  p := GetErrorString(18);
  Writeln(Format('    错误码 18: %s', [p]));
  FreeString(p);
  p := GetErrorString(19);
  Writeln(Format('    错误码 19: %s', [p]));
  FreeString(p);
  Writeln;

  // 10. 关闭
  Writeln('[10] 关闭 DLL');
  Shutdown();
  FreeLibrary(hLib);
  Writeln('    完成');

  Readln;
end.