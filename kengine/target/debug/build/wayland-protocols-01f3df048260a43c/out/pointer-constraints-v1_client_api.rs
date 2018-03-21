//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2014      Jonas Ådahl
    Copyright © 2015      Red Hat Inc.

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

pub mod zwp_pointer_constraints_v1 {
    //! constrain the movement of a pointer
    //!
    //! The global interface exposing pointer constraining functionality. It
    //! exposes two requests: lock_pointer for locking the pointer to its
    //! position, and confine_pointer for locking the pointer to a region.
    //! 
    //! The lock_pointer and confine_pointer requests create the objects
    //! wp_locked_pointer and wp_confined_pointer respectively, and the client can
    //! use these objects to interact with the lock.
    //! 
    //! For any surface, only one lock or confinement may be active across all
    //! wl_pointer objects of the same seat. If a lock or confinement is requested
    //! when another lock or confinement is active or requested on the same surface
    //! and with any of the wl_pointer objects of the same seat, an
    //! 'already_constrained' error will be raised.
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

    pub struct ZwpPointerConstraintsV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpPointerConstraintsV1 {}
    unsafe impl Sync for ZwpPointerConstraintsV1 {}
    unsafe impl Proxy for ZwpPointerConstraintsV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpPointerConstraintsV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpPointerConstraintsV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpPointerConstraintsV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpPointerConstraintsV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpPointerConstraintsV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_pointer_constraints_v1_interface } }
        fn interface_name() -> &'static str { "zwp_pointer_constraints_v1"  }
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

        fn equals(&self, other: &ZwpPointerConstraintsV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpPointerConstraintsV1 {
            ZwpPointerConstraintsV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    /// wp_pointer_constraints error values
    ///
    /// These errors can be emitted in response to wp_pointer_constraints
    /// requests.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// pointer constraint already requested on that surface
        AlreadyConstrained = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                1 => Some(Error::AlreadyConstrained),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    /// constraint lifetime
    ///
    /// These values represent different lifetime semantics. They are passed
    /// as arguments to the factory requests to specify how the constraint
    /// lifetimes should be managed.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Lifetime {
        /// the pointer constraint is defunct once deactivated
        ///
        /// A oneshot pointer constraint will never reactivate once it has been
        /// deactivated. See the corresponding deactivation event
        /// (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
        /// details.
        Oneshot = 1,
        /// the pointer constraint may reactivate
        ///
        /// A persistent pointer constraint may again reactivate once it has
        /// been deactivated. See the corresponding deactivation event
        /// (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
        /// details.
        Persistent = 2,
    }
    impl Lifetime {
        pub fn from_raw(n: u32) -> Option<Lifetime> {
            match n {
                1 => Some(Lifetime::Oneshot),
                2 => Some(Lifetime::Persistent),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    const ZWP_POINTER_CONSTRAINTS_V1_DESTROY: u32 = 0;
    const ZWP_POINTER_CONSTRAINTS_V1_LOCK_POINTER: u32 = 1;
    const ZWP_POINTER_CONSTRAINTS_V1_CONFINE_POINTER: u32 = 2;
    impl ZwpPointerConstraintsV1 {
        /// destroy the pointer constraints manager object
        ///
        /// Used by the client to notify the server that it will no longer use this
        /// pointer constraints object.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_POINTER_CONSTRAINTS_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// lock pointer to a position
        ///
        /// The lock_pointer request lets the client request to disable movements of
        /// the virtual pointer (i.e. the cursor), effectively locking the pointer
        /// to a position. This request may not take effect immediately; in the
        /// future, when the compositor deems implementation-specific constraints
        /// are satisfied, the pointer lock will be activated and the compositor
        /// sends a locked event.
        /// 
        /// The protocol provides no guarantee that the constraints are ever
        /// satisfied, and does not require the compositor to send an error if the
        /// constraints cannot ever be satisfied. It is thus possible to request a
        /// lock that will never activate.
        /// 
        /// There may not be another pointer constraint of any kind requested or
        /// active on the surface for any of the wl_pointer objects of the seat of
        /// the passed pointer when requesting a lock. If there is, an error will be
        /// raised. See general pointer lock documentation for more details.
        /// 
        /// The intersection of the region passed with this request and the input
        /// region of the surface is used to determine where the pointer must be
        /// in order for the lock to activate. It is up to the compositor whether to
        /// warp the pointer or require some kind of user interaction for the lock
        /// to activate. If the region is null the surface input region is used.
        /// 
        /// A surface may receive pointer focus without the lock being activated.
        /// 
        /// The request creates a new object wp_locked_pointer which is used to
        /// interact with the lock as well as receive updates about its state. See
        /// the the description of wp_locked_pointer for further information.
        /// 
        /// Note that while a pointer is locked, the wl_pointer objects of the
        /// corresponding seat will not emit any wl_pointer.motion events, but
        /// relative motion events will still be emitted via wp_relative_pointer
        /// objects of the same seat. wl_pointer.axis and wl_pointer.button events
        /// are unaffected.
        pub fn lock_pointer(&self, surface: &super::wl_surface::WlSurface, pointer: &super::wl_pointer::WlPointer, region: Option<&super::wl_region::WlRegion>, lifetime: u32) ->RequestResult<super::zwp_locked_pointer_v1::ZwpLockedPointerV1> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_POINTER_CONSTRAINTS_V1_LOCK_POINTER, &zwp_locked_pointer_v1_interface, ptr::null_mut::<wl_proxy>(), surface.ptr(), pointer.ptr(), region.map(Proxy::ptr).unwrap_or(ptr::null_mut()), lifetime) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// confine pointer to a region
        ///
        /// The confine_pointer request lets the client request to confine the
        /// pointer cursor to a given region. This request may not take effect
        /// immediately; in the future, when the compositor deems implementation-
        /// specific constraints are satisfied, the pointer confinement will be
        /// activated and the compositor sends a confined event.
        /// 
        /// The intersection of the region passed with this request and the input
        /// region of the surface is used to determine where the pointer must be
        /// in order for the confinement to activate. It is up to the compositor
        /// whether to warp the pointer or require some kind of user interaction for
        /// the confinement to activate. If the region is null the surface input
        /// region is used.
        /// 
        /// The request will create a new object wp_confined_pointer which is used
        /// to interact with the confinement as well as receive updates about its
        /// state. See the the description of wp_confined_pointer for further
        /// information.
        pub fn confine_pointer(&self, surface: &super::wl_surface::WlSurface, pointer: &super::wl_pointer::WlPointer, region: Option<&super::wl_region::WlRegion>, lifetime: u32) ->RequestResult<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_POINTER_CONSTRAINTS_V1_CONFINE_POINTER, &zwp_confined_pointer_v1_interface, ptr::null_mut::<wl_proxy>(), surface.ptr(), pointer.ptr(), region.map(Proxy::ptr).unwrap_or(ptr::null_mut()), lifetime) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod zwp_locked_pointer_v1 {
    //! receive relative pointer motion events
    //!
    //! The wp_locked_pointer interface represents a locked pointer state.
    //! 
    //! While the lock of this object is active, the wl_pointer objects of the
    //! associated seat will not emit any wl_pointer.motion events.
    //! 
    //! This object will send the event 'locked' when the lock is activated.
    //! Whenever the lock is activated, it is guaranteed that the locked surface
    //! will already have received pointer focus and that the pointer will be
    //! within the region passed to the request creating this object.
    //! 
    //! To unlock the pointer, send the destroy request. This will also destroy
    //! the wp_locked_pointer object.
    //! 
    //! If the compositor decides to unlock the pointer the unlocked event is
    //! sent. See wp_locked_pointer.unlock for details.
    //! 
    //! When unlocking, the compositor may warp the cursor position to the set
    //! cursor position hint. If it does, it will not result in any relative
    //! motion events emitted via wp_relative_pointer.
    //! 
    //! If the surface the lock was requested on is destroyed and the lock is not
    //! yet activated, the wp_locked_pointer object is now defunct and must be
    //! destroyed.
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

    pub struct ZwpLockedPointerV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpLockedPointerV1 {}
    unsafe impl Sync for ZwpLockedPointerV1 {}
    unsafe impl Proxy for ZwpLockedPointerV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpLockedPointerV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpLockedPointerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpLockedPointerV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpLockedPointerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpLockedPointerV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_locked_pointer_v1_interface } }
        fn interface_name() -> &'static str { "zwp_locked_pointer_v1"  }
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

        fn equals(&self, other: &ZwpLockedPointerV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpLockedPointerV1 {
            ZwpLockedPointerV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpLockedPointerV1 {
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
                    (implementation.locked)(evq, idata,  self);
                },
                1 => {
                    (implementation.unlocked)(evq, idata,  self);
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
        /// lock activation event
        ///
        /// Notification that the pointer lock of the seat's pointer is activated.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_locked_pointer_v1
        pub locked: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_locked_pointer_v1: &ZwpLockedPointerV1),
        /// lock deactivation event
        ///
        /// Notification that the pointer lock of the seat's pointer is no longer
        /// active. If this is a oneshot pointer lock (see
        /// wp_pointer_constraints.lifetime) this object is now defunct and should
        /// be destroyed. If this is a persistent pointer lock (see
        /// wp_pointer_constraints.lifetime) this pointer lock may again
        /// reactivate in the future.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_locked_pointer_v1
        pub unlocked: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_locked_pointer_v1: &ZwpLockedPointerV1),
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
            && (self.locked as usize == other.locked as usize)
            && (self.unlocked as usize == other.unlocked as usize)

        }
    }

    const ZWP_LOCKED_POINTER_V1_DESTROY: u32 = 0;
    const ZWP_LOCKED_POINTER_V1_SET_CURSOR_POSITION_HINT: u32 = 1;
    const ZWP_LOCKED_POINTER_V1_SET_REGION: u32 = 2;
    impl ZwpLockedPointerV1 {
        /// destroy the locked pointer object
        ///
        /// Destroy the locked pointer object. If applicable, the compositor will
        /// unlock the pointer.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_LOCKED_POINTER_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// set the pointer cursor position hint
        ///
        /// Set the cursor position hint relative to the top left corner of the
        /// surface.
        /// 
        /// If the client is drawing its own cursor, it should update the position
        /// hint to the position of its own cursor. A compositor may use this
        /// information to warp the pointer upon unlock in order to avoid pointer
        /// jumps.
        /// 
        /// The cursor position hint is double buffered. The new hint will only take
        /// effect when the associated surface gets it pending state applied. See
        /// wl_surface.commit for details.
        pub fn set_cursor_position_hint(&self, surface_x: f64, surface_y: f64) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let surface_x = wl_fixed_from_double(surface_x);
            let surface_y = wl_fixed_from_double(surface_y);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_LOCKED_POINTER_V1_SET_CURSOR_POSITION_HINT, surface_x, surface_y) };
            RequestResult::Sent(())
        }
        /// set a new lock region
        ///
        /// Set a new region used to lock the pointer.
        /// 
        /// The new lock region is double-buffered. The new lock region will
        /// only take effect when the associated surface gets its pending state
        /// applied. See wl_surface.commit for details.
        /// 
        /// For details about the lock region, see wp_locked_pointer.
        pub fn set_region(&self, region: Option<&super::wl_region::WlRegion>) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_LOCKED_POINTER_V1_SET_REGION, region.map(Proxy::ptr).unwrap_or(ptr::null_mut())) };
            RequestResult::Sent(())
        }
    }
}
pub mod zwp_confined_pointer_v1 {
    //! confined pointer object
    //!
    //! The wp_confined_pointer interface represents a confined pointer state.
    //! 
    //! This object will send the event 'confined' when the confinement is
    //! activated. Whenever the confinement is activated, it is guaranteed that
    //! the surface the pointer is confined to will already have received pointer
    //! focus and that the pointer will be within the region passed to the request
    //! creating this object. It is up to the compositor to decide whether this
    //! requires some user interaction and if the pointer will warp to within the
    //! passed region if outside.
    //! 
    //! To unconfine the pointer, send the destroy request. This will also destroy
    //! the wp_confined_pointer object.
    //! 
    //! If the compositor decides to unconfine the pointer the unconfined event is
    //! sent. The wp_confined_pointer object is at this point defunct and should
    //! be destroyed.
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

    pub struct ZwpConfinedPointerV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpConfinedPointerV1 {}
    unsafe impl Sync for ZwpConfinedPointerV1 {}
    unsafe impl Proxy for ZwpConfinedPointerV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpConfinedPointerV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpConfinedPointerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpConfinedPointerV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpConfinedPointerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpConfinedPointerV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_confined_pointer_v1_interface } }
        fn interface_name() -> &'static str { "zwp_confined_pointer_v1"  }
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

        fn equals(&self, other: &ZwpConfinedPointerV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpConfinedPointerV1 {
            ZwpConfinedPointerV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpConfinedPointerV1 {
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
                    (implementation.confined)(evq, idata,  self);
                },
                1 => {
                    (implementation.unconfined)(evq, idata,  self);
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
        /// pointer confined
        ///
        /// Notification that the pointer confinement of the seat's pointer is
        /// activated.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_confined_pointer_v1
        pub confined: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_confined_pointer_v1: &ZwpConfinedPointerV1),
        /// pointer unconfined
        ///
        /// Notification that the pointer confinement of the seat's pointer is no
        /// longer active. If this is a oneshot pointer confinement (see
        /// wp_pointer_constraints.lifetime) this object is now defunct and should
        /// be destroyed. If this is a persistent pointer confinement (see
        /// wp_pointer_constraints.lifetime) this pointer confinement may again
        /// reactivate in the future.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_confined_pointer_v1
        pub unconfined: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_confined_pointer_v1: &ZwpConfinedPointerV1),
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
            && (self.confined as usize == other.confined as usize)
            && (self.unconfined as usize == other.unconfined as usize)

        }
    }

    const ZWP_CONFINED_POINTER_V1_DESTROY: u32 = 0;
    const ZWP_CONFINED_POINTER_V1_SET_REGION: u32 = 1;
    impl ZwpConfinedPointerV1 {
        /// destroy the confined pointer object
        ///
        /// Destroy the confined pointer object. If applicable, the compositor will
        /// unconfine the pointer.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_CONFINED_POINTER_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// set a new confine region
        ///
        /// Set a new region used to confine the pointer.
        /// 
        /// The new confine region is double-buffered. The new confine region will
        /// only take effect when the associated surface gets its pending state
        /// applied. See wl_surface.commit for details.
        /// 
        /// If the confinement is active when the new confinement region is applied
        /// and the pointer ends up outside of newly applied region, the pointer may
        /// warped to a position within the new confinement region. If warped, a
        /// wl_pointer.motion event will be emitted, but no
        /// wp_relative_pointer.relative_motion event.
        /// 
        /// The compositor may also, instead of using the new region, unconfine the
        /// pointer.
        /// 
        /// For details about the confine region, see wp_confined_pointer.
        pub fn set_region(&self, region: Option<&super::wl_region::WlRegion>) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_CONFINED_POINTER_V1_SET_REGION, region.map(Proxy::ptr).unwrap_or(ptr::null_mut())) };
            RequestResult::Sent(())
        }
    }
}
