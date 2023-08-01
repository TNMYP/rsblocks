use crate::config::CONFIG;
use crate::types::ThreadsData;
use x11::xlib::{XOpenDisplay, XkbGetState, XkbStateRec, XGetAtomName, XCloseDisplay, XkbGetKeyboard};
use std::{ptr, mem};
use std::ffi::CStr;
pub async fn get_language() -> ThreadsData{
    let name:String;
    unsafe{
        let display = XOpenDisplay(ptr::null());
        if display == ptr::null_mut(){
           panic!("Failed to open display");
        }     

        let mut state: XkbStateRec = mem::zeroed();
        XkbGetState(display, 256 , &mut state);
        
        let keyboard = XkbGetKeyboard(display, 127, 256); // 127 XkbAllComponentsMask
                                                          // 256 XkbUseCoreKbd

        let cname = XGetAtomName(display, (*(*keyboard).names).groups[state.group as usize]);
        let cstr = CStr::from_ptr(cname);
        name = String::from_utf8_lossy(cstr.to_bytes()).to_string();

        XCloseDisplay(display);
    }
    let data = format!(
        "{} {}",
        name, CONFIG.seperator
                    );
    ThreadsData::Language(data)
}
