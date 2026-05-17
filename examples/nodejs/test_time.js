/**
 * time_module.dll Node.js 调用示例 (ffi-napi)
 * 最低支持: Node.js 12 + ffi-napi
 * 安装: npm install ffi-napi ref-napi
 * 运行: node test_time.js
 */

const ffi = require('ffi-napi');
const ref = require('ref-napi');

// 定义结构体
const FullTime = ref.types.struct({
    year: ref.types.int32,
    month: ref.types.int32,
    day: ref.types.int32,
    hour: ref.types.int32,
    minute: ref.types.int32,
    second: ref.types.int32,
    ms: ref.types.int32,
    us: ref.types.int32
});

// 加载 DLL
const lib = ffi.Library('time_module.dll', {
    'api_GetVersion': ['int', []],
    'api_GetVersionString': ['pointer', []],
    'api_FreeString': ['void', ['pointer']],
    'api_SetTimezoneOffset': ['int', ['int']],
    'api_GetLocalTime': [FullTime, []],
    'api_GetFormattedTime': ['pointer', []],
    'api_GetFormattedTimeBuf': ['int', ['pointer', 'int']],
    'api_GetWeekday': ['int', ['int', 'int', 'int']],
    'api_GetUnixTimestamp': ['int64', []],
    'api_IsLeapYearEx': ['int', ['int']],
    'api_Shutdown': ['void', []]
});

// 1. 版本
const ver = lib.api_GetVersion();
console.log(`DLL 版本: ${ver >> 16}.${(ver >> 8) & 0xFF}.${ver & 0xFF}`);

let ptr = lib.api_GetVersionString();
let verStr = ptr.readCString();
console.log(`版本字符串: ${verStr}`);
lib.api_FreeString(ptr);

// 2. 设置时区
lib.api_SetTimezoneOffset(28800);

// 3. 本地时间
const ft = lib.api_GetLocalTime();
console.log(`本地时间: ${ft.year.toString().padStart(4,'0')}-${ft.month.toString().padStart(2,'0')}-${ft.day.toString().padStart(2,'0')} ${ft.hour.toString().padStart(2,'0')}:${ft.minute.toString().padStart(2,'0')}:${ft.second.toString().padStart(2,'0')}.${ft.ms.toString().padStart(3,'0')}`);

// 4. 格式化时间（缓冲区）
const buf = Buffer.alloc(64);
const len = lib.api_GetFormattedTimeBuf(buf, buf.length);
if (len > 0) {
    console.log(`格式化时间: ${buf.toString('utf8', 0, len)}`);
}

// 5. 星期
const wd = lib.api_GetWeekday(ft.year, ft.month, ft.day);
console.log(`星期: ${wd} (0=星期日)`);

// 6. Unix 时间戳
const ts = lib.api_GetUnixTimestamp();
console.log(`Unix时间戳: ${ts} 秒`);

// 7. 闰年
const leap = lib.api_IsLeapYearEx(2000);
console.log(`2000年是闰年: ${leap ? '是' : '否'}`);

// 8. 关闭
lib.api_Shutdown();