// tests/error_code_tests.rs
// 集成测试：验证 v0.2.18 新增错误码
// 注意：此测试需要 time_module.dll 存在于可执行路径

#[cfg(test)]
mod error_code_tests {
    use std::ffi::CString;
    use std::ptr;

    // Windows DLL 加载
    #[cfg(windows)]
    type LibHandle = *mut std::ffi::c_void;

    #[cfg(windows)]
    fn load_library() -> LibHandle {
        use std::ffi::CString;
        let name = CString::new("time_module.dll").unwrap();
        unsafe { LoadLibraryA(name.as_ptr()) }
    }

    #[cfg(windows)]
    fn get_function<T>(handle: LibHandle, name: &str) -> Option<T> {
        use std::ffi::CString;
        let cname = CString::new(name).unwrap();
        unsafe {
            let addr = GetProcAddress(handle, cname.as_ptr());
            if addr.is_null() {
                None
            } else {
                Some(std::mem::transmute_copy(&addr))
            }
        }
    }

    #[cfg(windows)]
    extern "system" {
        fn LoadLibraryA(lpLibFileName: *const i8) -> LibHandle;
        fn GetProcAddress(hModule: LibHandle, lpProcName: *const i8) -> *const std::ffi::c_void;
        fn FreeLibrary(hModule: LibHandle) -> i32;
    }

    // 跳过集成测试（默认），因为需要 DLL 存在
    // 运行方式：cargo test -- --ignored
    #[test]
    #[ignore]
    fn test_timezone_offset_out_of_range() {
        let dll = load_library();
        assert!(!dll.is_null());

        type SetTimezoneOffsetFn = extern "C" fn(i32) -> i32;
        let set_offset: SetTimezoneOffsetFn = get_function(dll, "api_SetTimezoneOffset").unwrap();

        // 测试超出范围的正偏移
        let ret = set_offset(50400);
        assert_eq!(ret, 18); // TimezoneOffsetOutOfRange

        // 测试超出范围的负偏移
        let ret = set_offset(-50400);
        assert_eq!(ret, 18);

        unsafe { FreeLibrary(dll); }
    }

    #[test]
    #[ignore]
    fn test_timezone_name_not_found() {
        let dll = load_library();
        assert!(!dll.is_null());

        type SetTimezoneByNameFn = extern "C" fn(*const i8) -> i32;
        let set_by_name: SetTimezoneByNameFn = get_function(dll, "api_SetTimezoneByName").unwrap();

        let invalid_name = CString::new("INVALID_TIMEZONE").unwrap();
        let ret = set_by_name(invalid_name.as_ptr());
        assert_eq!(ret, 19); // TimezoneNameNotFound

        unsafe { FreeLibrary(dll); }
    }

    #[test]
    #[ignore]
    fn test_async_task_invalid_callback() {
        let dll = load_library();
        assert!(!dll.is_null());

        type ForceResyncAsyncFn = extern "C" fn(*mut std::ffi::c_void, Option<extern "C" fn(i32, i64, *mut std::ffi::c_void)>, *mut std::ffi::c_void) -> i32;
        let force_async: ForceResyncAsyncFn = get_function(dll, "api_ForceResyncAsync").unwrap();

        // 传递空回调应该返回 InvalidParam
        let ret = force_async(ptr::null_mut(), None, ptr::null_mut());
        assert_eq!(ret, 1); // InvalidParam

        unsafe { FreeLibrary(dll); }
    }
}