/**
 * time_module.dll Node.js 调用示例 (ffi-napi)
 * 最低支持: Node.js 12 + ffi-napi
 * 安装: npm install ffi-napi ref-napi
 * 运行: node test_time.js
 * 
 * v0.2.18 更新: 添加错误码处理和错误字符串演示
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
    'api_GetFormattedTimeBuf': ['int', ['pointer', 'int']],
    'api_GetWeekday': ['int', ['int', 'int', 'int']],
    'api_GetUnixTimestamp': ['int64', []],
    'api_IsLeapYearEx': ['int', ['int']],
    'api_GetErrorString': ['pointer', ['int']],
    'api_Shutdown': ['void', []]
});

function printError(funcName, errorCode) {
    if (errorCode !== 0) {
        let ptr = lib.api_GetErrorString(errorCode);
        let errStr = ptr.readCString();
        console.error(`  [错误] ${funcName} 失败: 错误码 ${errorCode} - ${errStr}`);
        lib.api_FreeString(ptr);
    }
}

console.log('========== time_module.dll 示例 (v0.2.18) ==========\n');

// 1. 版本
const ver = lib.api_GetVersion();
console.log('[1] 版本信息');
console.log(`    DLL 版本: ${ver >> 16}.${(ver >> 8) & 0xFF}.${ver & 0xFF}`);

let ptr = lib.api_GetVersionString();
let verStr = ptr.readCString();
console.log(`    版本字符串: ${verStr}`);
lib.api_FreeString(ptr);
console.log();

// 2. 设置时区
console.log('[2] 时区设置');
let ret = lib.api_SetTimezoneOffset(28800);
if (ret !== 0) {
    printError('SetTimezoneOffset', ret);
} else {
    console.log('    设置时区 UTC+8 成功');
}

// 3. 演示无效时区偏移
console.log('\n[3] 无效时区偏移测试');
ret = lib.api_SetTimezoneOffset(50400);
if (ret === 18) {
    console.log('    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)');
}
lib.api_SetTimezoneOffset(28800);
console.log();

// 4. 本地时间
console.log('[4] 本地时间');
const ft = lib.api_GetLocalTime();
console.log(`    本地时间: ${ft.year.toString().padStart(4, '0')}-${ft.month.toString().padStart(2, '0')}-${ft.day.toString().padStart(2, '0')} ${ft.hour.toString().padStart(2, '0')}:${ft.minute.toString().padStart(2, '0')}:${ft.second.toString().padStart(2, '0')}.${ft.ms.toString().padStart(3, '0')}`);
console.log();

// 5. 格式化时间
console.log('[5] 格式化时间');
const buf = Buffer.alloc(64);
const len = lib.api_GetFormattedTimeBuf(buf, buf.length);
if (len > 0) {
    console.log(`    格式化时间: ${buf.toString('utf8', 0, len)}`);
}
console.log();

// 6. 星期
console.log('[6] 星期信息');
const wd = lib.api_GetWeekday(ft.year, ft.month, ft.day);
const weekdays = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'];
console.log(`    星期: ${weekdays[wd]} (${wd})`);
console.log();

// 7. Unix 时间戳
console.log('[7] Unix 时间戳');
const ts = lib.api_GetUnixTimestamp();
console.log(`    Unix时间戳: ${ts} 秒`);
console.log();

// 8. 闰年
console.log('[8] 闰年判断');
const leap = lib.api_IsLeapYearEx(2000);
console.log(`    2000年是闰年: ${leap ? '是' : '否'}`);
console.log();

// 9. 错误字符串演示
console.log('[9] 错误字符串演示');
ptr = lib.api_GetErrorString(18);
console.log(`    错误码 18: ${ptr.readCString()}`);
lib.api_FreeString(ptr);
ptr = lib.api_GetErrorString(19);
console.log(`    错误码 19: ${ptr.readCString()}`);
lib.api_FreeString(ptr);
console.log();

// 10. 关闭
console.log('[10] 关闭 DLL');
lib.api_Shutdown();
console.log('    完成');