
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(unused_variables)]


#![feature(new_uninit)]
#![feature(concat_idents)]

use std::ptr;
use std::os::raw::{c_float};

mod te;
mod ui;
mod gm;
mod image;
mod gl_wrapper;

use crate::gm::{Size};
use crate::te::Screen;
use crate::gl_wrapper::gl_wrapper::Updatable;

#[macro_use] extern crate tools;
#[macro_use] extern crate guard;

static mut SCREEN: *mut Screen = ptr::null_mut();

#[no_mangle]
pub extern fn create_screen() {
    unsafe {
        let mut screen = Screen::new();
        screen.init();
        SCREEN = Box::into_raw(Box::new(screen));
    }
}

#[no_mangle]
pub extern fn set_screen_size(width: c_float, height: c_float) {
    unsafe {
        SCREEN.as_mut().unwrap().set_size(Size { width, height });
    }
}

#[no_mangle]
pub extern fn update_screen() {
    unsafe {
        SCREEN.as_mut().unwrap().update();
    }
}
