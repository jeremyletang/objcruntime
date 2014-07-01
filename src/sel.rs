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

use ffi;

pub fn get_name(selector: &ffi::SEL) -> &'static str {
    let c_name = unsafe { ffi::sel_getName(*selector) };
    unsafe { ::std::str::raw::c_str_to_static_slice(c_name) }
}

pub fn get_uid(string: &str) -> ffi::SEL {
    string.with_c_str(|c_str| {
        unsafe { ffi::sel_getUid(c_str) }
    })
}

pub fn is_equal(lhs: &ffi::SEL, rhs: &ffi::SEL) -> bool {
    let b = unsafe { ffi::sel_isEqual(*lhs, *rhs) };
    ffi::to_bool(b)
}

pub fn register(string: &str) -> ffi::SEL {
    string.with_c_str(|c_str| {
        unsafe { ffi::sel_registerName(c_str) }
    })
}
