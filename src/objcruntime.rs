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

#![crate_name = "github.com/jeremyletang/objcruntime"]
#![desc = "Rust binding to the objective-c runtime"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(missing_doc)]
#![feature(macro_rules)]

extern crate libc;

pub use ffi::{
    Class,
    Method,
    Ivar,
    Category,
    objc_property_t,
    SEL,
    id,
    Id
};

mod macros;

#[doc(hidden)]
pub mod ffi;
pub mod objc;
pub mod object;
pub mod class;
pub mod method;
pub mod ivar;
pub mod protocol;
pub mod sel;
pub mod user_macros;
