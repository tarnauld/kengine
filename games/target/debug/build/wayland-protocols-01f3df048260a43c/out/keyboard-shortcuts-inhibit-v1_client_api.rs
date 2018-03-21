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

pub mod zwp_keyboard_shortcuts_inhibit_manager_v1 {
    //! context object for keyboard grab_manager
    //!
    //! A global interface used for inhibiting the compositor keyboard shortcuts.
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

    pub struct ZwpKeyboardShortcutsInhibitManagerV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpKeyboardShortcutsInhibitManagerV1 {}
    unsafe impl Sync for ZwpKeyboardShortcutsInhibitManagerV1 {}
    unsafe impl Proxy for ZwpKeyboardShortcutsInhibitManagerV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpKeyboardShortcutsInhibitManagerV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpKeyboardShortcutsInhibitManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpKeyboardShortcutsInhibitManagerV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpKeyboardShortcutsInhibitManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpKeyboardShortcutsInhibitManagerV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_keyboard_shortcuts_inhibit_manager_v1_interface } }
        fn interface_name() -> &'static str { "zwp_keyboard_shortcuts_inhibit_manager_v1"  }
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

        fn equals(&self, other: &ZwpKeyboardShortcutsInhibitManagerV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpKeyboardShortcutsInhibitManagerV1 {
            ZwpKeyboardShortcutsInhibitManagerV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// the shortcuts are already inhibited for this surface
        AlreadyInhibited = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyInhibited),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    const ZWP_KEYBOARD_SHORTCUTS_INHIBIT_MANAGER_V1_DESTROY: u32 = 0;
    const ZWP_KEYBOARD_SHORTCUTS_INHIBIT_MANAGER_V1_INHIBIT_SHORTCUTS: u32 = 1;
    impl ZwpKeyboardShortcutsInhibitManagerV1 {
        /// destroy the keyboard shortcuts inhibitor object
        ///
        /// Destroy the keyboard shortcuts inhibitor manager.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_KEYBOARD_SHORTCUTS_INHIBIT_MANAGER_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// create a new keyboard shortcuts inhibitor object
        ///
        /// Create a new keyboard shortcuts inhibitor object associated with
        /// the given surface for the given seat.
        /// 
        /// If shortcuts are already inhibited for the specified seat and surface,
        /// a protocol error "already_inhibited" is raised by the compositor.
        pub fn inhibit_shortcuts(&self, surface: &super::wl_surface::WlSurface, seat: &super::wl_seat::WlSeat) ->RequestResult<super::zwp_keyboard_shortcuts_inhibitor_v1::ZwpKeyboardShortcutsInhibitorV1> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_KEYBOARD_SHORTCUTS_INHIBIT_MANAGER_V1_INHIBIT_SHORTCUTS, &zwp_keyboard_shortcuts_inhibitor_v1_interface, ptr::null_mut::<wl_proxy>(), surface.ptr(), seat.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod zwp_keyboard_shortcuts_inhibitor_v1 {
    //! context object for keyboard shortcuts inhibitor
    //!
    //! A keyboard shortcuts inhibitor instructs the compositor to ignore
    //! its own keyboard shortcuts when the associated surface has keyboard
    //! focus. As a result, when the surface has keyboard focus on the given
    //! seat, it will receive all key events originating from the specified
    //! seat, even those which would normally be caught by the compositor for
    //! its own shortcuts.
    //! 
    //! The Wayland compositor is however under no obligation to disable
    //! all of its shortcuts, and may keep some special key combo for its own
    //! use, including but not limited to one allowing the user to forcibly
    //! restore normal keyboard events routing in the case of an unwilling
    //! client. The compositor may also use the same key combo to reactivate
    //! an existing shortcut inhibitor that was previously deactivated on
    //! user request.
    //! 
    //! When the compositor restores its own keyboard shortcuts, an
    //! "inactive" event is emitted to notify the client that the keyboard
    //! shortcuts inhibitor is not effectively active for the surface and
    //! seat any more, and the client should not expect to receive all
    //! keyboard events.
    //! 
    //! When the keyboard shortcuts inhibitor is inactive, the client has
    //! no way to forcibly reactivate the keyboard shortcuts inhibitor.
    //! 
    //! The user can chose to re-enable a previously deactivated keyboard
    //! shortcuts inhibitor using any mechanism the compositor may offer,
    //! in which case the compositor will send an "active" event to notify
    //! the client.
    //! 
    //! If the surface is destroyed, unmapped, or loses the seat's keyboard
    //! focus, the keyboard shortcuts inhibitor becomes irrelevant and the
    //! compositor will restore its own keyboard shortcuts but no "inactive"
    //! event is emitted in this case.
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

    pub struct ZwpKeyboardShortcutsInhibitorV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpKeyboardShortcutsInhibitorV1 {}
    unsafe impl Sync for ZwpKeyboardShortcutsInhibitorV1 {}
    unsafe impl Proxy for ZwpKeyboardShortcutsInhibitorV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpKeyboardShortcutsInhibitorV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpKeyboardShortcutsInhibitorV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpKeyboardShortcutsInhibitorV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpKeyboardShortcutsInhibitorV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpKeyboardShortcutsInhibitorV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_keyboard_shortcuts_inhibitor_v1_interface } }
        fn interface_name() -> &'static str { "zwp_keyboard_shortcuts_inhibitor_v1"  }
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

        fn equals(&self, other: &ZwpKeyboardShortcutsInhibitorV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpKeyboardShortcutsInhibitorV1 {
            ZwpKeyboardShortcutsInhibitorV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpKeyboardShortcutsInhibitorV1 {
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
                    (implementation.active)(evq, idata,  self);
                },
                1 => {
                    (implementation.inactive)(evq, idata,  self);
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
        /// shortcuts are inhibited
        ///
        /// This event indicates that the shortcut inhibitor is active.
        /// 
        /// The compositor sends this event every time compositor shortcuts
        /// are inhibited on behalf of the surface. When active, the client
        /// may receive input events normally reserved by the compositor
        /// (see zwp_keyboard_shortcuts_inhibitor_v1).
        /// 
        /// This occurs typically when the initial request "inhibit_shortcuts"
        /// first becomes active or when the user instructs the compositor to
        /// re-enable and existing shortcuts inhibitor using any mechanism
        /// offered by the compositor.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_keyboard_shortcuts_inhibitor_v1
        pub active: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_keyboard_shortcuts_inhibitor_v1: &ZwpKeyboardShortcutsInhibitorV1),
        /// shortcuts are restored
        ///
        /// This event indicates that the shortcuts inhibitor is inactive,
        /// normal shortcuts processing is restored by the compositor.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_keyboard_shortcuts_inhibitor_v1
        pub inactive: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_keyboard_shortcuts_inhibitor_v1: &ZwpKeyboardShortcutsInhibitorV1),
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
            && (self.active as usize == other.active as usize)
            && (self.inactive as usize == other.inactive as usize)

        }
    }

    const ZWP_KEYBOARD_SHORTCUTS_INHIBITOR_V1_DESTROY: u32 = 0;
    impl ZwpKeyboardShortcutsInhibitorV1 {
        /// destroy the keyboard shortcuts inhibitor object
        ///
        /// Remove the keyboard shortcuts inhibitor from the associated wl_surface.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_KEYBOARD_SHORTCUTS_INHIBITOR_V1_DESTROY) };

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
