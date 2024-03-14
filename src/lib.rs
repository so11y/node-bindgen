mod ffi;

use ffi::*;
use std::{ffi::CString, ptr};

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

unsafe extern "C" fn fibonacci_rust(env: NapiEnv, info: NapiCallbackInfo) -> NapiValue {
    let mut this = ptr::null_mut();
    let mut argv: [NapiValue; 2] = [ptr::null_mut(), ptr::null_mut()];

    assert!(
        napi_get_cb_info(
            env,
            info,
            &mut argv.len(),
            argv.as_mut_ptr(),
            &mut this,
            ptr::null_mut(),
        ) == 0
    );

    let mut result = std::ptr::null_mut();

    let mut arg1 = 0;
    let mut arg2 = 0;

    assert!(napi_get_value_int32(env, argv[0], &mut arg1) == 0);
    assert!(napi_get_value_int32(env, argv[1], &mut arg2) == 0);

    assert!(napi_create_int32(env, fibonacci(arg1) + fibonacci(arg2), &mut result) == 0);

    return result;
}

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(env: NapiEnv, exports: NapiValue) -> NapiValue {
    let key = CString::new("hello").unwrap();
    let value = CString::new("world").unwrap();

    let mut str = ptr::null_mut();

    let mut js_function = ptr::null_mut();

    assert!(napi_create_string_utf8(env, value.as_ptr(), 5, &mut str) == 0);
    assert!(napi_set_named_property(env, exports, key.as_ptr(), str) == 0);

    let fibonacci_str = "fibonacci";
    let fibonacci_name = CString::new(fibonacci_str).unwrap();

    assert!(
        napi_create_function(
            env,
            fibonacci_str.as_ptr() as *const i8,
            fibonacci_str.len(),
            Some(fibonacci_rust),
            ptr::null_mut(),
            &mut js_function,
        ) == 0
    );

    assert!(napi_set_named_property(env, exports, fibonacci_name.as_ptr(), js_function) == 0);

    exports
}

// #[link(name = "user32")]
// extern "system" {
//     fn SetCursorPos(x: c_int, y: c_int) -> c_int;
// }
