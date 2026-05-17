program TestTime;

{$APPTYPE CONSOLE}

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
  TGetFormattedTime = function: PAnsiChar; cdecl;
  TGetFormattedTimeBuf = function(buf: PAnsiChar; bufSize: Integer): Integer; cdecl;
  TGetWeekday = function(year, month, day: Integer): Integer; cdecl;
  TGetUnixTimestamp = function: Int64; cdecl;
  TIsLeapYearEx = function(year: Integer): Integer; cdecl;
  TShutdown = procedure; cdecl;

var
  hLib: THandle;
  GetVersion: TGetVersion;
  GetVersionString: TGetVersionString;
  FreeString: TFreeString;
  SetTimezoneOffset: TSetTimezoneOffset;
  GetLocalTime: TGetLocalTime;
  GetFormattedTime: TGetFormattedTime;
  GetFormattedTimeBuf: TGetFormattedTimeBuf;
  GetWeekday: TGetWeekday;
  GetUnixTimestamp: TGetUnixTimestamp;
  IsLeapYearEx: TIsLeapYearEx;
  Shutdown: TShutdown;
  ft: FullTime;
  ver: Integer;
  p: PAnsiChar;
  buf: array[0..63] of AnsiChar;
  len: Integer;
begin
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
  GetFormattedTime := GetProcAddress(hLib, 'api_GetFormattedTime');
  GetFormattedTimeBuf := GetProcAddress(hLib, 'api_GetFormattedTimeBuf');
  GetWeekday := GetProcAddress(hLib, 'api_GetWeekday');
  GetUnixTimestamp := GetProcAddress(hLib, 'api_GetUnixTimestamp');
  IsLeapYearEx := GetProcAddress(hLib, 'api_IsLeapYearEx');
  Shutdown := GetProcAddress(hLib, 'api_Shutdown');

  // 版本
  ver := GetVersion();
  Writeln(Format('DLL 版本: %d.%d.%d', [ver shr 16, (ver shr 8) and $FF, ver and $FF]));
  p := GetVersionString();
  Writeln('版本字符串: ', p);
  FreeString(p);

  // 设置时区
  SetTimezoneOffset(28800);

  // 本地时间
  ft := GetLocalTime();
  Writeln(Format('本地时间: %.4d-%.2d-%.2d %.2d:%.2d:%.2d.%.3d',
    [ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms]));

  // 格式化时间（缓冲区）
  len := GetFormattedTimeBuf(buf, SizeOf(buf));
  if len > 0 then
    Writeln('格式化时间: ', buf);

  // 星期
  Writeln(Format('星期: %d (0=星期日)', [GetWeekday(ft.year, ft.month, ft.day)]));

  // Unix 时间戳
  Writeln(Format('Unix时间戳: %d 秒', [GetUnixTimestamp()]));

  // 闰年
  Writeln(Format('2000年是闰年: %s', [IfThen(IsLeapYearEx(2000) <> 0, '是', '否')]));

  // 关闭
  Shutdown();
  FreeLibrary(hLib);
  Readln;
end.