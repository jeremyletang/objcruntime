// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![macro_escape]

#[macro_export]
macro_rules! m(
    ($id:expr $msg:ident) => ({
        use objcruntime::{sel, ffi};

        unsafe {
            let op = sel::register(stringify!($msg));
            ffi::objc_msgSend($id, op)
        }
    });

    ($id:expr $($msg:ident: $arg:expr)+) => ({
        use objcruntime::{sel, ffi};

        let op = sel::register(concat!($(stringify!($msg), ":"), +));
        unsafe {
            ffi::objc_msgSend($id, op $(,$arg)+)
        }
    })
)

// pub fn objc_msgSend_stret(stretAddr: *mut c_void, theReceiver: id, theSelector: SEL,  ...);
#[macro_export]
macro_rules! m_struct(
    ($struc:ident, $id:expr $msg:ident) => ({
        use objcruntime::{sel, ffi};

        unsafe {
            let op = sel::register(stringify!($msg));
            ffi::objc_msgSend_stret(struc, $id, op)
        }
    });

    ($struc:ident, $id:expr $($msg:ident: $arg:expr)+) => ({
        use objcruntime::{sel, ffi};

        let op = sel::register(concat!($(stringify!($msg), ":"), +));
        unsafe {
            ffi::objc_msgSend_stret(struc, $id, op $(,$arg)+)
        }
    })
)

#[macro_export]
macro_rules! cstr(
    ($string:expr) => (
        $string.to_c_str().unwrap()
    )
)

#[macro_export]
macro_rules! cls(
    ($class:ident) => ({
        use objcruntime::objc;

        objc::get_class(stringify!($class)).unwrap()
    })
)