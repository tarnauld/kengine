//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2017 Red Hat Inc.

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice (including the next
    paragraph) shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
    THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
*/

pub mod zxdg_output_manager_v1 {
    //! manage xdg_output objects
    //!
    //! A global factory interface for xdg_output objects.
    use super::EventQueueHandle;
    use super::Proxy;
    use super::RequestResult;

    use super::{Liveness, Implementable};
    use super::interfaces::*;
    use wayland_sys::common::*;
    use std::any::Any;
    use std::ffi::{CString,CStr};
    use std::os::raw::c_void;
    use std::ptr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
    use wayland_sys::RUST_MANAGED;
    use wayland_sys::client::*;
    type UserData = (*mut EventQueueHandle, Option<Box<Any>>, Arc<(AtomicBool, AtomicPtr<()>)>);

    pub struct ZxdgOutputManagerV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgOutputManagerV1 {}
    unsafe impl Sync for ZxdgOutputManagerV1 {}
    unsafe impl Proxy for ZxdgOutputManagerV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgOutputManagerV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgOutputManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgOutputManagerV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgOutputManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgOutputManagerV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_output_manager_v1_interface } }
        fn interface_name() -> &'static str { "zxdg_output_manager_v1"  }
        fn supported_version() -> u32 { 1 }
        fn version(&self) -> u32 { unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_version, self.ptr()) } }

        fn status(&self) -> Liveness {
            if let Some(ref data) = self.data {
                if data.0.load(Ordering::SeqCst) {
                    Liveness::Alive
                } else {
                    Liveness::Dead
                }
            } else {
                Liveness::Unmanaged
            }
        }

        fn equals(&self, other: &ZxdgOutputManagerV1) -> bool {
            self.status() != Liveness::Dead && other.status() != Liveness::Dead && self.ptr == other.ptr
        }

        fn set_user_data(&self, ptr: *mut ()) {
            if let Some(ref data) = self.data {
                data.1.store(ptr, Ordering::SeqCst);
            }
        }
        fn get_user_data(&self) -> *mut () {
            if let Some(ref data) = self.data {
                data.1.load(Ordering::SeqCst)
            } else {
                ::std::ptr::null_mut()
            }
        }

        unsafe fn clone_unchecked(&self) -> ZxdgOutputManagerV1 {
            ZxdgOutputManagerV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZXDG_OUTPUT_MANAGER_V1_DESTROY: u32 = 0;
    const ZXDG_OUTPUT_MANAGER_V1_GET_XDG_OUTPUT: u32 = 1;
    impl ZxdgOutputManagerV1 {
        /// destroy the xdg_output_manager object
        ///
        /// Using this request a client can tell the server that it is not
        /// going to use the xdg_output_manager object anymore.
        /// 
        /// Any objects already created through this instance are not affected.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_OUTPUT_MANAGER_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// create an xdg output from a wl_output
        ///
        /// This creates a new xdg_output object for the given wl_output.
        pub fn get_xdg_output(&self, output: &super::wl_output::WlOutput) ->RequestResult<super::zxdg_output_v1::ZxdgOutputV1> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZXDG_OUTPUT_MANAGER_V1_GET_XDG_OUTPUT, &zxdg_output_v1_interface, ptr::null_mut::<wl_proxy>(), output.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod zxdg_output_v1 {
    //! compositor logical output region
    //!
    //! An xdg_output describes part of the compositor geometry.
    //! 
    //! This typically corresponds to a monitor that displays part of the
    //! compositor space.
    use super::EventQueueHandle;
    use super::Proxy;
    use super::RequestResult;

    use super::{Liveness, Implementable};
    use super::interfaces::*;
    use wayland_sys::common::*;
    use std::any::Any;
    use std::ffi::{CString,CStr};
    use std::os::raw::c_void;
    use std::ptr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
    use wayland_sys::RUST_MANAGED;
    use wayland_sys::client::*;
    type UserData = (*mut EventQueueHandle, Option<Box<Any>>, Arc<(AtomicBool, AtomicPtr<()>)>);

    pub struct ZxdgOutputV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgOutputV1 {}
    unsafe impl Sync for ZxdgOutputV1 {}
    unsafe impl Proxy for ZxdgOutputV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgOutputV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgOutputV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgOutputV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgOutputV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgOutputV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_output_v1_interface } }
        fn interface_name() -> &'static str { "zxdg_output_v1"  }
        fn supported_version() -> u32 { 1 }
        fn version(&self) -> u32 { unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_version, self.ptr()) } }

        fn status(&self) -> Liveness {
            if let Some(ref data) = self.data {
                if data.0.load(Ordering::SeqCst) {
                    Liveness::Alive
                } else {
                    Liveness::Dead
                }
            } else {
                Liveness::Unmanaged
            }
        }

        fn equals(&self, other: &ZxdgOutputV1) -> bool {
            self.status() != Liveness::Dead && other.status() != Liveness::Dead && self.ptr == other.ptr
        }

        fn set_user_data(&self, ptr: *mut ()) {
            if let Some(ref data) = self.data {
                data.1.store(ptr, Ordering::SeqCst);
            }
        }
        fn get_user_data(&self) -> *mut () {
            if let Some(ref data) = self.data {
                data.1.load(Ordering::SeqCst)
            } else {
                ::std::ptr::null_mut()
            }
        }

        unsafe fn clone_unchecked(&self) -> ZxdgOutputV1 {
            ZxdgOutputV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZxdgOutputV1 {
        type Implementation = Implementation<ID>;
        #[allow(unused_mut,unused_assignments)]
        unsafe fn __dispatch_msg(&self,  opcode: u32, args: *const wl_argument) -> Result<(),()> {

        let data = &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData);
        let evq = &mut *(data.0);
        let mut kill = false;
        {
            let &mut (ref implementation, ref mut idata) = data.1.as_mut().unwrap().downcast_mut::<(Implementation<ID>, ID)>().unwrap();
            match opcode {
                0 => {
                    let x = {*(args.offset(0) as *const i32)};
                    let y = {*(args.offset(1) as *const i32)};
                    (implementation.logical_position)(evq, idata,  self, x, y);
                },
                1 => {
                    let width = {*(args.offset(0) as *const i32)};
                    let height = {*(args.offset(1) as *const i32)};
                    (implementation.logical_size)(evq, idata,  self, width, height);
                },
                2 => {
                    (implementation.done)(evq, idata,  self);
                },
                _ => return Err(())
            }
        }

        if kill {
            let _impl = data.1.take();
            ::std::mem::drop(_impl);
        }
            Ok(())
        }
    }
    pub struct Implementation<ID> {
        /// position of the output within the global compositor space
        ///
        /// The position event describes the location of the wl_output within
        /// the global compositor space.
        /// 
        /// The logical_position event is sent after creating an xdg_output
        /// (see xdg_output_manager.get_xdg_output) and whenever the location
        /// of the output changes within the global compositor space.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_output_v1, x, y
        pub logical_position: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_output_v1: &ZxdgOutputV1, x: i32, y: i32),
        /// size of the output in the global compositor space
        ///
        /// The logical_size event describes the size of the output in the
        /// global compositor space.
        /// 
        /// For example, a surface without any buffer scale, transformation
        /// nor rotation set, with the size matching the logical_size will
        /// have the same size as the corresponding output when displayed.
        /// 
        /// Most regular Wayland clients should not pay attention to the
        /// logical size and would rather rely on xdg_shell interfaces.
        /// 
        /// Some clients such as Xwayland, however, need this to configure
        /// their surfaces in the global compositor space as the compositor
        /// may apply a different scale from what is advertised by the output
        /// scaling property (to achieve fractional scaling, for example).
        /// 
        /// For example, for a wl_output mode 3840×2160 and a scale factor 2:
        /// 
        /// - A compositor not scaling the surface buffers will advertise a
        /// logical size of 3840×2160,
        /// 
        /// - A compositor automatically scaling the surface buffers will
        /// advertise a logical size of 1920×1080,
        /// 
        /// - A compositor using a fractional scale of 1.5 will advertise a
        /// logical size to 2560×1620.
        /// 
        /// The logical_size event is sent after creating an xdg_output
        /// (see xdg_output_manager.get_xdg_output) and whenever the logical
        /// size of the output changes, either as a result of a change in the
        /// applied scale or because of a change in the corresponding output
        /// mode(see wl_output.mode) or transform (see wl_output.transform).
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_output_v1, width, height
        pub logical_size: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_output_v1: &ZxdgOutputV1, width: i32, height: i32),
        /// all information about the output have been sent
        ///
        /// This event is sent after all other properties of an xdg_output
        /// have been sent.
        /// 
        /// This allows changes to the xdg_output properties to be seen as
        /// atomic, even if they happen via multiple events.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_output_v1
        pub done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_output_v1: &ZxdgOutputV1),
    }

    impl<ID> Copy for Implementation<ID> {}
    impl<ID> Clone for Implementation<ID> {
        fn clone(&self) -> Implementation<ID> {
            *self
        }
    }

    impl<ID> PartialEq for Implementation<ID> {
        fn eq(&self, other: &Implementation<ID>) -> bool {
            true
            && (self.logical_position as usize == other.logical_position as usize)
            && (self.logical_size as usize == other.logical_size as usize)
            && (self.done as usize == other.done as usize)

        }
    }

    const ZXDG_OUTPUT_V1_DESTROY: u32 = 0;
    impl ZxdgOutputV1 {
        /// destroy the xdg_output object
        ///
        /// Using this request a client can tell the server that it is not
        /// going to use the xdg_output object anymore.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_OUTPUT_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
    }
}
