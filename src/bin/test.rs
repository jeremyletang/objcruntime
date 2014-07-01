
#![feature(phase)]
#![allow(unused_unsafe)]

#[phase(plugin)]
extern crate objcruntime;
extern crate objcruntime;
extern crate libc;

// use objcruntime::{object};
// use objcruntime::ffi::Wrapper;

fn main() {
    let pool = m![m![cls!(NSAutoreleasePool) alloc] init];
    // let class: objcruntime::Class = object::get_class(&id).unwrap();

    // println!("class name / version: {} / {}", class::get_name(&class), class::get_version(&class));

    let nstr: objcruntime::id = m![m![cls!(NSString) alloc] initWithUTF8String: cstr!("hello world")];
    let len: objcruntime::id = m![nstr length];
    let utf8 = m![nstr UTF8String];
    // let len_class = object::get_class(&nstr).unwrap();
    // println!("class name / version: {} / {}", class::get_name(&len_class), class::get_version(&len_class));
    println!("len: {}", unsafe { ::std::mem::transmute::<objcruntime::id, int>(len) } );
    println!("STRING: {}", unsafe {  ::std::str::raw::from_c_str(::std::mem::transmute::<objcruntime::id, *const libc::c_char>(utf8)) })
    m![nstr release];
    m![pool drain];
}