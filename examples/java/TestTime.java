/**
 * time_module.dll Java 调用示例 (JNA)
 * 最低支持: Java 8 + JNA 5.0
 * 编译: javac -cp jna.jar TestTime.java
 * 运行: java -cp .;jna.jar -Djava.library.path=. TestTime
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
        Pointer api_GetFormattedTime();
        int api_GetFormattedTimeBuf(byte[] buf, int bufSize);
        int api_GetWeekday(int year, int month, int day);
        long api_GetUnixTimestamp();
        int api_IsLeapYearEx(int year);
        void api_Shutdown();
    }

    public static void main(String[] args) {
        TimeModule tm = TimeModule.INSTANCE;

        // 版本
        int ver = tm.api_GetVersion();
        System.out.printf("DLL 版本: %d.%d.%d%n", ver>>16, (ver>>8)&0xFF, ver&0xFF);
        Pointer p = tm.api_GetVersionString();
        String verStr = p.getString(0);
        System.out.println("版本字符串: " + verStr);
        tm.api_FreeString(p);

        // 设置时区
        tm.api_SetTimezoneOffset(28800);

        // 本地时间
        FullTime ft = tm.api_GetLocalTime();
        System.out.printf("本地时间: %04d-%02d-%02d %02d:%02d:%02d.%03d%n",
            ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms);

        // 格式化时间（缓冲区）
        byte[] buf = new byte[64];
        int len = tm.api_GetFormattedTimeBuf(buf, buf.length);
        if (len > 0) {
            System.out.println("格式化时间: " + new String(buf, 0, len));
        }

        // 星期
        int wd = tm.api_GetWeekday(ft.year, ft.month, ft.day);
        System.out.println("星期: " + wd + " (0=星期日)");

        // Unix 时间戳
        long ts = tm.api_GetUnixTimestamp();
        System.out.println("Unix时间戳: " + ts + " 秒");

        // 闰年
        int leap = tm.api_IsLeapYearEx(2000);
        System.out.println("2000年是闰年: " + (leap!=0 ? "是" : "否"));

        // 关闭
        tm.api_Shutdown();
    }
}