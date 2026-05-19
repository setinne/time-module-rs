/**
 * time_module.dll Java 调用示例 (JNA)
 * 最低支持: Java 8 + JNA 5.0
 * 编译: javac -cp jna.jar TestTime.java
 * 运行: java -cp .;jna.jar -Djava.library.path=. TestTime
 * 
 * v0.2.18 更新: 添加错误码处理和错误字符串演示
 */

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import java.util.Arrays;
import java.util.List;

public class TestTime {
    // 定义结构体
    public static class FullTime extends Structure {
        public int year, month, day, hour, minute, second, ms, us;
        @Override
        protected List<String> getFieldOrder() {
            return Arrays.asList("year","month","day","hour","minute","second","ms","us");
        }
    }

    // 定义 DLL 接口
    public interface TimeModule extends Library {
        TimeModule INSTANCE = Native.load("time_module", TimeModule.class);
        int api_GetVersion();
        Pointer api_GetVersionString();
        void api_FreeString(Pointer ptr);
        int api_SetTimezoneOffset(int seconds);
        FullTime api_GetLocalTime();
        int api_GetFormattedTimeBuf(byte[] buf, int bufSize);
        int api_GetWeekday(int year, int month, int day);
        long api_GetUnixTimestamp();
        int api_IsLeapYearEx(int year);
        Pointer api_GetErrorString(int code);
        void api_Shutdown();
    }

    static void printError(String funcName, int errorCode, TimeModule tm) {
        if (errorCode != 0) {
            Pointer p = tm.api_GetErrorString(errorCode);
            String err = p.getString(0);
            System.err.printf("  [错误] %s 失败: 错误码 %d - %s%n", funcName, errorCode, err);
            tm.api_FreeString(p);
        }
    }

    public static void main(String[] args) {
        TimeModule tm = TimeModule.INSTANCE;

        System.out.println("========== time_module.dll 示例 (v0.2.18) ==========\n");

        // 1. 版本
        int ver = tm.api_GetVersion();
        System.out.println("[1] 版本信息");
        System.out.printf("    DLL 版本: %d.%d.%d%n", ver>>16, (ver>>8)&0xFF, ver&0xFF);
        Pointer p = tm.api_GetVersionString();
        String verStr = p.getString(0);
        System.out.println("    版本字符串: " + verStr);
        tm.api_FreeString(p);
        System.out.println();

        // 2. 设置时区
        System.out.println("[2] 时区设置");
        int ret = tm.api_SetTimezoneOffset(28800);
        if (ret != 0) {
            printError("SetTimezoneOffset", ret, tm);
        } else {
            System.out.println("    设置时区 UTC+8 成功");
        }

        // 3. 演示无效时区偏移
        System.out.println("\n[3] 无效时区偏移测试");
        ret = tm.api_SetTimezoneOffset(50400);
        if (ret == 18) {
            System.out.println("    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)");
        }
        tm.api_SetTimezoneOffset(28800);
        System.out.println();

        // 4. 本地时间
        System.out.println("[4] 本地时间");
        FullTime ft = tm.api_GetLocalTime();
        System.out.printf("    本地时间: %04d-%02d-%02d %02d:%02d:%02d.%03d%n",
            ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms);
        System.out.println();

        // 5. 格式化时间
        System.out.println("[5] 格式化时间");
        byte[] buf = new byte[64];
        int len = tm.api_GetFormattedTimeBuf(buf, buf.length);
        if (len > 0) {
            System.out.println("    格式化时间: " + new String(buf, 0, len));
        }
        System.out.println();

        // 6. 星期
        System.out.println("[6] 星期信息");
        int wd = tm.api_GetWeekday(ft.year, ft.month, ft.day);
        String[] weekdays = {"星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"};
        System.out.printf("    星期: %s (%d)%n", weekdays[wd], wd);
        System.out.println();

        // 7. Unix 时间戳
        System.out.println("[7] Unix 时间戳");
        long ts = tm.api_GetUnixTimestamp();
        System.out.println("    Unix时间戳: " + ts + " 秒");
        System.out.println();

        // 8. 闰年
        System.out.println("[8] 闰年判断");
        int leap = tm.api_IsLeapYearEx(2000);
        System.out.println("    2000年是闰年: " + (leap != 0 ? "是" : "否"));
        System.out.println();

        // 9. 错误字符串演示
        System.out.println("[9] 错误字符串演示");
        Pointer errPtr = tm.api_GetErrorString(18);
        System.out.println("    错误码 18: " + errPtr.getString(0));
        tm.api_FreeString(errPtr);
        errPtr = tm.api_GetErrorString(19);
        System.out.println("    错误码 19: " + errPtr.getString(0));
        tm.api_FreeString(errPtr);
        System.out.println();

        // 10. 关闭
        System.out.println("[10] 关闭 DLL");
        tm.api_Shutdown();
        System.out.println("    完成");
    }
}