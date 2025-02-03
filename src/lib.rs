use winapi::um::winuser::{MessageBoxA, MB_OK};
use std::ffi::CString;

pub fn msgbox(title: &str, content: &str) {
    let title_cstr = CString::new(title).unwrap();
    let content_cstr = CString::new(content).unwrap();
    unsafe {
        MessageBoxA(std::ptr::null_mut(), content_cstr.as_ptr(), title_cstr.as_ptr(), MB_OK);
    };
}