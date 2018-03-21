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

pub mod zwp_xwayland_keyboard_grab_manager_v1 {
    //! context object for keyboard grab manager
    //!
    //! A global interface used for grabbing the keyboard.
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

    pub struct ZwpXwaylandKeyboardGrabManagerV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpXwaylandKeyboardGrabManagerV1 {}
    unsafe impl Sync for ZwpXwaylandKeyboardGrabManagerV1 {}
    unsafe impl Proxy for ZwpXwaylandKeyboardGrabManagerV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpXwaylandKeyboardGrabManagerV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpXwaylandKeyboardGrabManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpXwaylandKeyboardGrabManagerV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpXwaylandKeyboardGrabManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpXwaylandKeyboardGrabManagerV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_xwayland_keyboard_grab_manager_v1_interface } }
        fn interface_name() -> &'static str { "zwp_xwayland_keyboard_grab_manager_v1"  }
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

        fn equals(&self, other: &ZwpXwaylandKeyboardGrabManagerV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpXwaylandKeyboardGrabManagerV1 {
            ZwpXwaylandKeyboardGrabManagerV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZWP_XWAYLAND_KEYBOARD_GRAB_MANAGER_V1_DESTROY: u32 = 0;
    const ZWP_XWAYLAND_KEYBOARD_GRAB_MANAGER_V1_GRAB_KEYBOARD: u32 = 1;
    impl ZwpXwaylandKeyboardGrabManagerV1 {
        /// destroy the keyboard grab manager
        ///
        /// Destroy the keyboard grab manager.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_XWAYLAND_KEYBOARD_GRAB_MANAGER_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// grab the keyboard to a surface
        ///
        /// The grab_keyboard request asks for a grab of the keyboard, forcing
        /// the keyboard focus for the given seat upon the given surface.
        /// 
        /// The protocol provides no guarantee that the grab is ever satisfied,
        /// and does not require the compositor to send an error if the grab
        /// cannot ever be satisfied. It is thus possible to request a keyboard
        /// grab that will never be effective.
        /// 
        /// The protocol:
        /// 
        /// * does not guarantee that the grab itself is applied for a surface,
        /// the grab request may be silently ignored by the compositor,
        /// * does not guarantee that any events are sent to this client even
        /// if the grab is applied to a surface,
        /// * does not guarantee that events sent to this client are exhaustive,
        /// a compositor may filter some events for its own consumption,
        /// * does not guarantee that events sent to this client are continuous,
        /// a compositor may change and reroute keyboard events while the grab
        /// is nominally active.
        pub fn grab_keyboard(&self, surface: &super::wl_surface::WlSurface, seat: &super::wl_seat::WlSeat) ->RequestResult<super::zwp_xwayland_keyboard_grab_v1::ZwpXwaylandKeyboardGrabV1> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_XWAYLAND_KEYBOARD_GRAB_MANAGER_V1_GRAB_KEYBOARD, &zwp_xwayland_keyboard_grab_v1_interface, ptr::null_mut::<wl_proxy>(), surface.ptr(), seat.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod zwp_xwayland_keyboard_grab_v1 {
    //! interface for grabbing the keyboard
    //!
    //! A global interface used for grabbing the keyboard.
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

    pub struct ZwpXwaylandKeyboardGrabV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpXwaylandKeyboardGrabV1 {}
    unsafe impl Sync for ZwpXwaylandKeyboardGrabV1 {}
    unsafe impl Proxy for ZwpXwaylandKeyboardGrabV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpXwaylandKeyboardGrabV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpXwaylandKeyboardGrabV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpXwaylandKeyboardGrabV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpXwaylandKeyboardGrabV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpXwaylandKeyboardGrabV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_xwayland_keyboard_grab_v1_interface } }
        fn interface_name() -> &'static str { "zwp_xwayland_keyboard_grab_v1"  }
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

        fn equals(&self, other: &ZwpXwaylandKeyboardGrabV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpXwaylandKeyboardGrabV1 {
            ZwpXwaylandKeyboardGrabV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZWP_XWAYLAND_KEYBOARD_GRAB_V1_DESTROY: u32 = 0;
    impl ZwpXwaylandKeyboardGrabV1 {
        /// destroy the grabbed keyboard object
        ///
        /// Destroy the grabbed keyboard object. If applicable, the compositor
        /// will ungrab the keyboard.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_XWAYLAND_KEYBOARD_GRAB_V1_DESTROY) };

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
