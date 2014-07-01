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

#![allow(raw_pointer_deriving)]

use libc::{c_char, c_int, size_t};

pub struct objc_class;
pub struct objc_method;
pub struct objc_ivar;
pub struct objc_category;
pub struct objc_property;
pub struct objc_selector;

// wrappers

pub trait Wrapper<T> {
    fn wrap(ptr: T) -> Self;
    fn unwrap(&self) -> T;
}

pub trait Id {
    fn get_id(&self) -> id;
    fn from_id(id: id) -> Self;
}

#[deriving(Clone, PartialEq, Show)]
pub struct Class {
    ptr: *mut objc_class
}

impl_wrapper!(Class, objc_class)

#[deriving(Clone, PartialEq, Show)]
pub struct Method {
    ptr: *mut objc_method
}

impl_wrapper!(Method, objc_method)

#[deriving(Clone, PartialEq, Show)]
pub struct Ivar {
    ptr: *mut objc_ivar
}

impl_wrapper!(Ivar, objc_ivar)

#[deriving(Clone, PartialEq, Show)]
pub struct Category {
    ptr: *mut objc_category
}

impl_wrapper!(Category, objc_category)

#[deriving(Clone, PartialEq, Show)]
pub struct objc_property_t {
    ptr: *mut objc_property
}

impl_wrapper!(objc_property_t, objc_property)

#[deriving(Clone, PartialEq, Show)]
pub struct SEL {
    ptr: *mut objc_selector
}

impl_wrapper!(SEL, objc_selector)

#[deriving(Clone, PartialEq, Show)]
pub struct id {
    ptr: *mut objc_object
}

impl Id for id {
    fn get_id(&self) -> id {
        *self
    }

    #[allow(unused_variable)]
    fn from_id(id: id) -> id {
        id {
            ptr: ::std::ptr::mut_null()
        }
    }
}

impl_wrapper!(id, objc_object)

// ffi

pub struct objc_method_description {
   name: SEL,
   types: *const c_char
}

pub struct objc_object {
   isa: Class
}

pub struct objc_super {
    receiver: id,
    class: Class
}

pub type BOOL = c_char;
pub static YES: BOOL = 1;
pub static NO: BOOL = 0;

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ObjcAssocBehavior {
   Assign = 0,
   RetainNonAtomic = 1,
   CopyNonAtomic = 3,
   Retain = 01401,
   Copy = 01403
}

pub fn to_bool(b: BOOL) -> bool {
    match b {
        YES => true,
        NO => false,
        _ => unreachable!()
    }
}

pub fn to_objcbool(b: bool) -> BOOL {
    match b {
        true => YES,
        false => NO
    }
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    // objc
    pub fn objc_getClass(name: *const c_char) -> id;
    pub fn objc_msgSend(self_: id, op: SEL, ...) -> id;

    // object
    pub fn object_getClass(object: id) -> Class;
    pub fn object_getClassName(obj: id) -> *const c_char;

    // class
    pub fn class_getName(cls: Class) -> *const c_char;
    pub fn class_getSuperclass(cls: Class) -> Class;
    pub fn class_getVersion(cls: Class) -> c_int;
    pub fn class_isMetaClass(cls: Class) -> BOOL;
    pub fn class_createInstance(cls: Class, extraBytes: size_t) -> id;

    // sel
    pub fn sel_getName(aSelector: SEL) -> *const c_char;
    pub fn sel_getUid(string: *const c_char) -> SEL;
    pub fn sel_isEqual(lhs: SEL, rhs: SEL) -> BOOL;
    pub fn sel_registerName(string: *const c_char) -> SEL;
}
























