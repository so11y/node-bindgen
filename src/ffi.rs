mod ffi {
    use std::ffi::{c_char, c_void};

    #[repr(C)]
    pub struct NapiCallbackInfo__ {
        _unused: [u8; 0],
    }
    #[repr(C)]
    pub struct NapiValue__ {
        _unused: [u8; 0],
    }
    #[repr(C)]
    pub struct NapiEnv__ {
        _unused: [u8; 0],
    }

    pub type NapiCallbackInfo = *mut NapiCallbackInfo__;
    pub type NapiCallback =
        Option<unsafe extern "C" fn(env: NapiEnv, info: NapiCallbackInfo) -> NapiValue>;

    pub type NapiEnv = *mut NapiEnv__;

    pub type NapiValue = *mut NapiValue__;

    //https://nodejs.org/dist/v18.16.1/win-x64/ 找到自己对应的node版本
    //然后下载dll文件,添加到对应目录
    //可以改为宏去自动下载对应版本，当前为硬编码
    #[link(name = "C:\\Program Files\\nodejs\\node-v18.16.1", kind = "dylib")]
    extern "C" {

        pub fn napi_set_named_property(
            env: NapiEnv,
            object: NapiValue,
            utf8name: *const c_char,
            value: NapiValue,
        ) -> u8;

        pub fn napi_create_string_utf8(
            env: NapiEnv,
            str_: *const c_char,
            length: usize,
            result: *mut NapiValue,
        ) -> u8;

        pub fn napi_create_function(
            env: NapiEnv,
            utf8name: *const c_char,
            length: usize,
            cb: NapiCallback,
            data: *mut c_void,
            result: *mut NapiValue,
        ) -> u8;

        pub fn napi_get_cb_info(
            env: NapiEnv,
            cbinfo: NapiCallbackInfo,
            argc: *mut usize,
            argv: *mut NapiValue,
            this_arg: *mut NapiValue,
            data: *mut *mut c_void,
        ) -> u8;

        pub fn napi_get_value_int32(env: NapiEnv, value: NapiValue, result: *mut i32) -> u8;

        pub fn napi_create_int32(env: NapiEnv, value: i32, result: *mut NapiValue) -> u8;
    }
}

pub use ffi::*;
