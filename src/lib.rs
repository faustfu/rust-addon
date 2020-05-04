use nodejs_sys::{
    napi_callback_info, napi_create_double, napi_create_function, napi_create_string_utf8,
    napi_env, napi_get_cb_info, napi_get_undefined, napi_get_value_double,
    napi_get_value_string_utf8, napi_set_named_property, napi_value,
};
use std::ffi::CString;

pub unsafe extern "C" fn say_hello(env: napi_env, _info: napi_callback_info) -> napi_value {
    // creating  a javastring string
    let mut local: napi_value = std::mem::zeroed();
    let p = CString::new("Hello from rust").expect("CString::new    failed");
    napi_create_string_utf8(env, p.as_ptr(), 15, &mut local);
    // returning the javascript string
    local
}

pub unsafe extern "C" fn add(env: napi_env, info: napi_callback_info) -> napi_value {
    // creating a buffer where napi_value of argument be written
    let mut buffer: [napi_value; 2] = std::mem::MaybeUninit::zeroed().assume_init();
    // max number of arguments
    let mut argc = 2 as usize;
    // getting arguments and value of this
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    // converting napi to f64
    let mut x = 0 as f64;
    let mut y = 0 as f64;
    napi_get_value_double(env, buffer[0], &mut x);
    napi_get_value_double(env, buffer[1], &mut y);
    // creating the return value
    let mut local: napi_value = std::mem::zeroed();
    napi_create_double(env, x + y, &mut local);
    // returning the result
    local
}

pub unsafe extern "C" fn print(env: napi_env, info: napi_callback_info) -> napi_value {
    // creating a buffer of arguments
    let mut buffer: [napi_value; 1] = std::mem::MaybeUninit::zeroed().assume_init();
    let mut argc = 1 as usize;
    // getting arguments
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    let mut len = 0;
    // getting length by passing null buffer
    napi_get_value_string_utf8(env, buffer[0], std::ptr::null_mut(), 0, &mut len);
    let size = len as usize;
    // creating a buffer where string can be placed
    let mut ve: Vec<u8> = Vec::with_capacity(size + 1);
    let raw = ve.as_mut_ptr();
    // telling rust not manage the vector
    std::mem::forget(ve);
    let mut cap = 0;
    // getting the string value from napi_value
    let _s = napi_get_value_string_utf8(env, buffer[0], raw as *mut i8, size + 1, &mut cap);
    let s = String::from_raw_parts(raw, cap as usize, size);
    // printing the string
    println!("{}", s);
    // creating an undefined
    let mut und: napi_value = std::mem::zeroed();
    napi_get_undefined(env, &mut und);
    // returning undefined
    und
}

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(env: napi_env, exports: napi_value) -> napi_value {
    // creating a C string
    let key = CString::new("hello").expect("CString::new failed");
    // creating a memory location where the pointer to napi_value will be saved
    let mut local: napi_value = std::mem::zeroed();
    // creating a C string
    let value = CString::new("world!").expect("CString::new failed");
    // creating napi_value for the string
    napi_create_string_utf8(env, value.as_ptr(), 6, &mut local);
    // setting the string on the exports object
    napi_set_named_property(env, exports, key.as_ptr(), local);

    // creating a C String
    let p = CString::new("myFunc1").expect("CString::new failed");
    // creating a location where pointer to napi_value be written
    let mut local: napi_value = std::mem::zeroed();
    napi_create_function(
        env,
        // pointer to function name
        p.as_ptr(),
        // length of function name
        7,
        // rust function
        Some(say_hello),
        // context which can be accessed by the rust function
        std::ptr::null_mut(),
        // output napi_value
        &mut local,
    );
    // set function as property
    napi_set_named_property(env, exports, p.as_ptr(), local);

    // creating a C String
    let p = CString::new("myFunc2").expect("CString::new failed");
    // creating a location where pointer to napi_value be written
    let mut local: napi_value = std::mem::zeroed();
    napi_create_function(
        env,
        // pointer to function name
        p.as_ptr(),
        // length of function name
        7,
        // rust function
        Some(add),
        // context which can be accessed by the rust function
        std::ptr::null_mut(),
        // output napi_value
        &mut local,
    );
    // set function as property
    napi_set_named_property(env, exports, p.as_ptr(), local);

    // creating a C String
    let p = CString::new("myFunc3").expect("CString::new failed");
    // creating a location where pointer to napi_value be written
    let mut local: napi_value = std::mem::zeroed();
    napi_create_function(
        env,
        // pointer to function name
        p.as_ptr(),
        // length of function name
        7,
        // rust function
        Some(print),
        // context which can be accessed by the rust function
        std::ptr::null_mut(),
        // output napi_value
        &mut local,
    );
    // set function as property
    napi_set_named_property(env, exports, p.as_ptr(), local);

    // returning the object
    exports
}
