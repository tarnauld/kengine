//
// This file was auto-generated, do not edit directly
//

/*
Copyright 2014 © Stephen "Lyude" Chandler Paul
    Copyright 2015-2016 © Red Hat, Inc.

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation files
    (the "Software"), to deal in the Software without restriction,
    including without limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of the Software,
    and to permit persons to whom the Software is furnished to do so,
    subject to the following conditions:

    The above copyright notice and this permission notice (including the
    next paragraph) shall be included in all copies or substantial
    portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
    BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
    ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
    CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

pub mod zwp_tablet_manager_v2 {
    //! controller object for graphic tablet devices
    //!
    //! An object that provides access to the graphics tablets available on this
    //! system. All tablets are associated with a seat, to get access to the
    //! actual tablets, use wp_tablet_manager.get_tablet_seat.
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

    pub struct ZwpTabletManagerV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletManagerV2 {}
    unsafe impl Sync for ZwpTabletManagerV2 {}
    unsafe impl Proxy for ZwpTabletManagerV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletManagerV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletManagerV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletManagerV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletManagerV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletManagerV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_manager_v2_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_manager_v2"  }
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

        fn equals(&self, other: &ZwpTabletManagerV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletManagerV2 {
            ZwpTabletManagerV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZWP_TABLET_MANAGER_V2_GET_TABLET_SEAT: u32 = 0;
    const ZWP_TABLET_MANAGER_V2_DESTROY: u32 = 1;
    impl ZwpTabletManagerV2 {
        /// get the tablet seat
        ///
        /// Get the wp_tablet_seat object for the given seat. This object
        /// provides access to all graphics tablets in this seat.
        pub fn get_tablet_seat(&self, seat: &super::wl_seat::WlSeat) ->RequestResult<super::zwp_tablet_seat_v2::ZwpTabletSeatV2> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_TABLET_MANAGER_V2_GET_TABLET_SEAT, &zwp_tablet_seat_v2_interface, ptr::null_mut::<wl_proxy>(), seat.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// release the memory for the tablet manager object
        ///
        /// Destroy the wp_tablet_manager object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_MANAGER_V2_DESTROY) };

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
pub mod zwp_tablet_seat_v2 {
    //! controller object for graphic tablet devices of a seat
    //!
    //! An object that provides access to the graphics tablets available on this
    //! seat. After binding to this interface, the compositor sends a set of
    //! wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events.
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

    pub struct ZwpTabletSeatV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletSeatV2 {}
    unsafe impl Sync for ZwpTabletSeatV2 {}
    unsafe impl Proxy for ZwpTabletSeatV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletSeatV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletSeatV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletSeatV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletSeatV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletSeatV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_seat_v2_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_seat_v2"  }
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

        fn equals(&self, other: &ZwpTabletSeatV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletSeatV2 {
            ZwpTabletSeatV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletSeatV2 {
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
                    let id = {Proxy::from_ptr_new(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.tablet_added)(evq, idata,  self, id);
                },
                1 => {
                    let id = {Proxy::from_ptr_new(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.tool_added)(evq, idata,  self, id);
                },
                2 => {
                    let id = {Proxy::from_ptr_new(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.pad_added)(evq, idata,  self, id);
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
        /// new device notification
        ///
        /// This event is sent whenever a new tablet becomes available on this
        /// seat. This event only provides the object id of the tablet, any
        /// static information about the tablet (device name, vid/pid, etc.) is
        /// sent through the wp_tablet interface.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_seat_v2, id
        pub tablet_added: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_seat_v2: &ZwpTabletSeatV2, id: super::zwp_tablet_v2::ZwpTabletV2),
        /// a new tool has been used with a tablet
        ///
        /// This event is sent whenever a tool that has not previously been used
        /// with a tablet comes into use. This event only provides the object id
        /// of the tool; any static information about the tool (capabilities,
        /// type, etc.) is sent through the wp_tablet_tool interface.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_seat_v2, id
        pub tool_added: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_seat_v2: &ZwpTabletSeatV2, id: super::zwp_tablet_tool_v2::ZwpTabletToolV2),
        /// new pad notification
        ///
        /// This event is sent whenever a new pad is known to the system. Typically,
        /// pads are physically attached to tablets and a pad_added event is
        /// sent immediately after the wp_tablet_seat.tablet_added.
        /// However, some standalone pad devices logically attach to tablets at
        /// runtime, and the client must wait for wp_tablet_pad.enter to know
        /// the tablet a pad is attached to.
        /// 
        /// This event only provides the object id of the pad. All further
        /// features (buttons, strips, rings) are sent through the wp_tablet_pad
        /// interface.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_seat_v2, id
        pub pad_added: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_seat_v2: &ZwpTabletSeatV2, id: super::zwp_tablet_pad_v2::ZwpTabletPadV2),
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
            && (self.tablet_added as usize == other.tablet_added as usize)
            && (self.tool_added as usize == other.tool_added as usize)
            && (self.pad_added as usize == other.pad_added as usize)

        }
    }

    const ZWP_TABLET_SEAT_V2_DESTROY: u32 = 0;
    impl ZwpTabletSeatV2 {
        /// release the memory for the tablet seat object
        ///
        /// Destroy the wp_tablet_seat object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_SEAT_V2_DESTROY) };

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
pub mod zwp_tablet_tool_v2 {
    //! a physical tablet tool
    //!
    //! An object that represents a physical tool that has been, or is
    //! currently in use with a tablet in this seat. Each wp_tablet_tool
    //! object stays valid until the client destroys it; the compositor
    //! reuses the wp_tablet_tool object to indicate that the object's
    //! respective physical tool has come into proximity of a tablet again.
    //! 
    //! A wp_tablet_tool object's relation to a physical tool depends on the
    //! tablet's ability to report serial numbers. If the tablet supports
    //! this capability, then the object represents a specific physical tool
    //! and can be identified even when used on multiple tablets.
    //! 
    //! A tablet tool has a number of static characteristics, e.g. tool type,
    //! hardware_serial and capabilities. These capabilities are sent in an
    //! event sequence after the wp_tablet_seat.tool_added event before any
    //! actual events from this tool. This initial event sequence is
    //! terminated by a wp_tablet_tool.done event.
    //! 
    //! Tablet tool events are grouped by wp_tablet_tool.frame events.
    //! Any events received before a wp_tablet_tool.frame event should be
    //! considered part of the same hardware state change.
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

    pub struct ZwpTabletToolV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletToolV2 {}
    unsafe impl Sync for ZwpTabletToolV2 {}
    unsafe impl Proxy for ZwpTabletToolV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletToolV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletToolV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletToolV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletToolV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletToolV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_tool_v2_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_tool_v2"  }
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

        fn equals(&self, other: &ZwpTabletToolV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletToolV2 {
            ZwpTabletToolV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletToolV2 {
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
                    let tool_type = {match Type::from_raw(*(args.offset(0) as *const u32)) { Some(v) => v, Option::None => return Err(()) }};
                    (implementation.type_)(evq, idata,  self, tool_type);
                },
                1 => {
                    let hardware_serial_hi = {*(args.offset(0) as *const u32)};
                    let hardware_serial_lo = {*(args.offset(1) as *const u32)};
                    (implementation.hardware_serial)(evq, idata,  self, hardware_serial_hi, hardware_serial_lo);
                },
                2 => {
                    let hardware_id_hi = {*(args.offset(0) as *const u32)};
                    let hardware_id_lo = {*(args.offset(1) as *const u32)};
                    (implementation.hardware_id_wacom)(evq, idata,  self, hardware_id_hi, hardware_id_lo);
                },
                3 => {
                    let capability = {match Capability::from_raw(*(args.offset(0) as *const u32)) { Some(v) => v, Option::None => return Err(()) }};
                    (implementation.capability)(evq, idata,  self, capability);
                },
                4 => {
                    (implementation.done)(evq, idata,  self);
                },
                5 => {
                    (implementation.removed)(evq, idata,  self);
                },
                6 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let tablet = {Proxy::from_ptr_initialized(*(args.offset(1) as *const *mut wl_proxy))};
                    let surface = {Proxy::from_ptr_initialized(*(args.offset(2) as *const *mut wl_proxy))};
                    (implementation.proximity_in)(evq, idata,  self, serial, &tablet, &surface);
                },
                7 => {
                    (implementation.proximity_out)(evq, idata,  self);
                },
                8 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    (implementation.down)(evq, idata,  self, serial);
                },
                9 => {
                    (implementation.up)(evq, idata,  self);
                },
                10 => {
                    let x = {wl_fixed_to_double(*(args.offset(0) as *const i32))};
                    let y = {wl_fixed_to_double(*(args.offset(1) as *const i32))};
                    (implementation.motion)(evq, idata,  self, x, y);
                },
                11 => {
                    let pressure = {*(args.offset(0) as *const u32)};
                    (implementation.pressure)(evq, idata,  self, pressure);
                },
                12 => {
                    let distance = {*(args.offset(0) as *const u32)};
                    (implementation.distance)(evq, idata,  self, distance);
                },
                13 => {
                    let tilt_x = {wl_fixed_to_double(*(args.offset(0) as *const i32))};
                    let tilt_y = {wl_fixed_to_double(*(args.offset(1) as *const i32))};
                    (implementation.tilt)(evq, idata,  self, tilt_x, tilt_y);
                },
                14 => {
                    let degrees = {wl_fixed_to_double(*(args.offset(0) as *const i32))};
                    (implementation.rotation)(evq, idata,  self, degrees);
                },
                15 => {
                    let position = {*(args.offset(0) as *const i32)};
                    (implementation.slider)(evq, idata,  self, position);
                },
                16 => {
                    let degrees = {wl_fixed_to_double(*(args.offset(0) as *const i32))};
                    let clicks = {*(args.offset(1) as *const i32)};
                    (implementation.wheel)(evq, idata,  self, degrees, clicks);
                },
                17 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let button = {*(args.offset(1) as *const u32)};
                    let state = {match ButtonState::from_raw(*(args.offset(2) as *const u32)) { Some(v) => v, Option::None => return Err(()) }};
                    (implementation.button)(evq, idata,  self, serial, button, state);
                },
                18 => {
                    let time = {*(args.offset(0) as *const u32)};
                    (implementation.frame)(evq, idata,  self, time);
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
    /// a physical tool type
    ///
    /// Describes the physical type of a tool. The physical type of a tool
    /// generally defines its base usage.
    /// 
    /// The mouse tool represents a mouse-shaped tool that is not a relative
    /// device but bound to the tablet's surface, providing absolute
    /// coordinates.
    /// 
    /// The lens tool is a mouse-shaped tool with an attached lens to
    /// provide precision focus.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Type {
        /// Pen
        Pen = 0x140,
        /// Eraser
        Eraser = 0x141,
        /// Brush
        Brush = 0x142,
        /// Pencil
        Pencil = 0x143,
        /// Airbrush
        Airbrush = 0x144,
        /// Finger
        Finger = 0x145,
        /// Mouse
        Mouse = 0x146,
        /// Lens
        Lens = 0x147,
    }
    impl Type {
        pub fn from_raw(n: u32) -> Option<Type> {
            match n {
                0x140 => Some(Type::Pen),
                0x141 => Some(Type::Eraser),
                0x142 => Some(Type::Brush),
                0x143 => Some(Type::Pencil),
                0x144 => Some(Type::Airbrush),
                0x145 => Some(Type::Finger),
                0x146 => Some(Type::Mouse),
                0x147 => Some(Type::Lens),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    /// capability flags for a tool
    ///
    /// Describes extra capabilities on a tablet.
    /// 
    /// Any tool must provide x and y values, extra axes are
    /// device-specific.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Capability {
        /// Tilt axes
        Tilt = 1,
        /// Pressure axis
        Pressure = 2,
        /// Distance axis
        Distance = 3,
        /// Z-rotation axis
        Rotation = 4,
        /// Slider axis
        Slider = 5,
        /// Wheel axis
        Wheel = 6,
    }
    impl Capability {
        pub fn from_raw(n: u32) -> Option<Capability> {
            match n {
                1 => Some(Capability::Tilt),
                2 => Some(Capability::Pressure),
                3 => Some(Capability::Distance),
                4 => Some(Capability::Rotation),
                5 => Some(Capability::Slider),
                6 => Some(Capability::Wheel),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    /// physical button state
    ///
    /// Describes the physical state of a button that produced the button event.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ButtonState {
        /// button is not pressed
        Released = 0,
        /// button is pressed
        Pressed = 1,
    }
    impl ButtonState {
        pub fn from_raw(n: u32) -> Option<ButtonState> {
            match n {
                0 => Some(ButtonState::Released),
                1 => Some(ButtonState::Pressed),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// given wl_surface has another role
        Role = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// tool type
        ///
        /// The tool type is the high-level type of the tool and usually decides
        /// the interaction expected from this tool.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, tool_type
        pub type_: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, tool_type: Type),
        /// unique hardware serial number of the tool
        ///
        /// If the physical tool can be identified by a unique 64-bit serial
        /// number, this event notifies the client of this serial number.
        /// 
        /// If multiple tablets are available in the same seat and the tool is
        /// uniquely identifiable by the serial number, that tool may move
        /// between tablets.
        /// 
        /// Otherwise, if the tool has no serial number and this event is
        /// missing, the tool is tied to the tablet it first comes into
        /// proximity with. Even if the physical tool is used on multiple
        /// tablets, separate wp_tablet_tool objects will be created, one per
        /// tablet.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, hardware_serial_hi, hardware_serial_lo
        pub hardware_serial: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, hardware_serial_hi: u32, hardware_serial_lo: u32),
        /// hardware id notification in Wacom's format
        ///
        /// This event notifies the client of a hardware id available on this tool.
        /// 
        /// The hardware id is a device-specific 64-bit id that provides extra
        /// information about the tool in use, beyond the wl_tool.type
        /// enumeration. The format of the id is specific to tablets made by
        /// Wacom Inc. For example, the hardware id of a Wacom Grip
        /// Pen (a stylus) is 0x802.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, hardware_id_hi, hardware_id_lo
        pub hardware_id_wacom: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, hardware_id_hi: u32, hardware_id_lo: u32),
        /// tool capability notification
        ///
        /// This event notifies the client of any capabilities of this tool,
        /// beyond the main set of x/y axes and tip up/down detection.
        /// 
        /// One event is sent for each extra capability available on this tool.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, capability
        pub capability: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, capability: Capability),
        /// tool description events sequence complete
        ///
        /// This event signals the end of the initial burst of descriptive
        /// events. A client may consider the static description of the tool to
        /// be complete and finalize initialization of the tool.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2
        pub done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2),
        /// tool removed
        ///
        /// This event is sent when the tool is removed from the system and will
        /// send no further events. Should the physical tool come back into
        /// proximity later, a new wp_tablet_tool object will be created.
        /// 
        /// It is compositor-dependent when a tool is removed. A compositor may
        /// remove a tool on proximity out, tablet removal or any other reason.
        /// A compositor may also keep a tool alive until shutdown.
        /// 
        /// If the tool is currently in proximity, a proximity_out event will be
        /// sent before the removed event. See wp_tablet_tool.proximity_out for
        /// the handling of any buttons logically down.
        /// 
        /// When this event is received, the client must wp_tablet_tool.destroy
        /// the object.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2
        pub removed: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2),
        /// proximity in event
        ///
        /// Notification that this tool is focused on a certain surface.
        /// 
        /// This event can be received when the tool has moved from one surface to
        /// another, or when the tool has come back into proximity above the
        /// surface.
        /// 
        /// If any button is logically down when the tool comes into proximity,
        /// the respective button event is sent after the proximity_in event but
        /// within the same frame as the proximity_in event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, serial, tablet, surface
        pub proximity_in: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, serial: u32, tablet: &super::zwp_tablet_v2::ZwpTabletV2, surface: &super::wl_surface::WlSurface),
        /// proximity out event
        ///
        /// Notification that this tool has either left proximity, or is no
        /// longer focused on a certain surface.
        /// 
        /// When the tablet tool leaves proximity of the tablet, button release
        /// events are sent for each button that was held down at the time of
        /// leaving proximity. These events are sent before the proximity_out
        /// event but within the same wp_tablet.frame.
        /// 
        /// If the tool stays within proximity of the tablet, but the focus
        /// changes from one surface to another, a button release event may not
        /// be sent until the button is actually released or the tool leaves the
        /// proximity of the tablet.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2
        pub proximity_out: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2),
        /// tablet tool is making contact
        ///
        /// Sent whenever the tablet tool comes in contact with the surface of the
        /// tablet.
        /// 
        /// If the tool is already in contact with the tablet when entering the
        /// input region, the client owning said region will receive a
        /// wp_tablet.proximity_in event, followed by a wp_tablet.down
        /// event and a wp_tablet.frame event.
        /// 
        /// Note that this event describes logical contact, not physical
        /// contact. On some devices, a compositor may not consider a tool in
        /// logical contact until a minimum physical pressure threshold is
        /// exceeded.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, serial
        pub down: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, serial: u32),
        /// tablet tool is no longer making contact
        ///
        /// Sent whenever the tablet tool stops making contact with the surface of
        /// the tablet, or when the tablet tool moves out of the input region
        /// and the compositor grab (if any) is dismissed.
        /// 
        /// If the tablet tool moves out of the input region while in contact
        /// with the surface of the tablet and the compositor does not have an
        /// ongoing grab on the surface, the client owning said region will
        /// receive a wp_tablet.up event, followed by a wp_tablet.proximity_out
        /// event and a wp_tablet.frame event. If the compositor has an ongoing
        /// grab on this device, this event sequence is sent whenever the grab
        /// is dismissed in the future.
        /// 
        /// Note that this event describes logical contact, not physical
        /// contact. On some devices, a compositor may not consider a tool out
        /// of logical contact until physical pressure falls below a specific
        /// threshold.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2
        pub up: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2),
        /// motion event
        ///
        /// Sent whenever a tablet tool moves.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, x, y
        pub motion: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, x: f64, y: f64),
        /// pressure change event
        ///
        /// Sent whenever the pressure axis on a tool changes. The value of this
        /// event is normalized to a value between 0 and 65535.
        /// 
        /// Note that pressure may be nonzero even when a tool is not in logical
        /// contact. See the down and up events for more details.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, pressure
        pub pressure: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, pressure: u32),
        /// distance change event
        ///
        /// Sent whenever the distance axis on a tool changes. The value of this
        /// event is normalized to a value between 0 and 65535.
        /// 
        /// Note that distance may be nonzero even when a tool is not in logical
        /// contact. See the down and up events for more details.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, distance
        pub distance: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, distance: u32),
        /// tilt change event
        ///
        /// Sent whenever one or both of the tilt axes on a tool change. Each tilt
        /// value is in degrees, relative to the z-axis of the tablet.
        /// The angle is positive when the top of a tool tilts along the
        /// positive x or y axis.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, tilt_x, tilt_y
        pub tilt: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, tilt_x: f64, tilt_y: f64),
        /// z-rotation change event
        ///
        /// Sent whenever the z-rotation axis on the tool changes. The
        /// rotation value is in degrees clockwise from the tool's
        /// logical neutral position.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, degrees
        pub rotation: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, degrees: f64),
        /// Slider position change event
        ///
        /// Sent whenever the slider position on the tool changes. The
        /// value is normalized between -65535 and 65535, with 0 as the logical
        /// neutral position of the slider.
        /// 
        /// The slider is available on e.g. the Wacom Airbrush tool.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, position
        pub slider: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, position: i32),
        /// Wheel delta event
        ///
        /// Sent whenever the wheel on the tool emits an event. This event
        /// contains two values for the same axis change. The degrees value is
        /// in the same orientation as the wl_pointer.vertical_scroll axis. The
        /// clicks value is in discrete logical clicks of the mouse wheel. This
        /// value may be zero if the movement of the wheel was less
        /// than one logical click.
        /// 
        /// Clients should choose either value and avoid mixing degrees and
        /// clicks. The compositor may accumulate values smaller than a logical
        /// click and emulate click events when a certain threshold is met.
        /// Thus, wl_tablet_tool.wheel events with non-zero clicks values may
        /// have different degrees values.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, degrees, clicks
        pub wheel: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, degrees: f64, clicks: i32),
        /// button event
        ///
        /// Sent whenever a button on the tool is pressed or released.
        /// 
        /// If a button is held down when the tool moves in or out of proximity,
        /// button events are generated by the compositor. See
        /// wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for
        /// details.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, serial, button, state
        pub button: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, serial: u32, button: u32, state: ButtonState),
        /// frame event
        ///
        /// Marks the end of a series of axis and/or button updates from the
        /// tablet. The Wayland protocol requires axis updates to be sent
        /// sequentially, however all events within a frame should be considered
        /// one hardware event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v2, time
        pub frame: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v2: &ZwpTabletToolV2, time: u32),
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
            && (self.type_ as usize == other.type_ as usize)
            && (self.hardware_serial as usize == other.hardware_serial as usize)
            && (self.hardware_id_wacom as usize == other.hardware_id_wacom as usize)
            && (self.capability as usize == other.capability as usize)
            && (self.done as usize == other.done as usize)
            && (self.removed as usize == other.removed as usize)
            && (self.proximity_in as usize == other.proximity_in as usize)
            && (self.proximity_out as usize == other.proximity_out as usize)
            && (self.down as usize == other.down as usize)
            && (self.up as usize == other.up as usize)
            && (self.motion as usize == other.motion as usize)
            && (self.pressure as usize == other.pressure as usize)
            && (self.distance as usize == other.distance as usize)
            && (self.tilt as usize == other.tilt as usize)
            && (self.rotation as usize == other.rotation as usize)
            && (self.slider as usize == other.slider as usize)
            && (self.wheel as usize == other.wheel as usize)
            && (self.button as usize == other.button as usize)
            && (self.frame as usize == other.frame as usize)

        }
    }

    const ZWP_TABLET_TOOL_V2_SET_CURSOR: u32 = 0;
    const ZWP_TABLET_TOOL_V2_DESTROY: u32 = 1;
    impl ZwpTabletToolV2 {
        /// set the tablet tool's surface
        ///
        /// Sets the surface of the cursor used for this tool on the given
        /// tablet. This request only takes effect if the tool is in proximity
        /// of one of the requesting client's surfaces or the surface parameter
        /// is the current pointer surface. If there was a previous surface set
        /// with this request it is replaced. If surface is NULL, the cursor
        /// image is hidden.
        /// 
        /// The parameters hotspot_x and hotspot_y define the position of the
        /// pointer surface relative to the pointer location. Its top-left corner
        /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
        /// coordinates of the pointer location, in surface-local coordinates.
        /// 
        /// On surface.attach requests to the pointer surface, hotspot_x and
        /// hotspot_y are decremented by the x and y parameters passed to the
        /// request. Attach must be confirmed by wl_surface.commit as usual.
        /// 
        /// The hotspot can also be updated by passing the currently set pointer
        /// surface to this request with new values for hotspot_x and hotspot_y.
        /// 
        /// The current and pending input regions of the wl_surface are cleared,
        /// and wl_surface.set_input_region is ignored until the wl_surface is no
        /// longer used as the cursor. When the use as a cursor ends, the current
        /// and pending input regions become undefined, and the wl_surface is
        /// unmapped.
        /// 
        /// This request gives the surface the role of a wp_tablet_tool cursor. A
        /// surface may only ever be used as the cursor surface for one
        /// wp_tablet_tool. If the surface already has another role or has
        /// previously been used as cursor surface for a different tool, a
        /// protocol error is raised.
        pub fn set_cursor(&self, serial: u32, surface: Option<&super::wl_surface::WlSurface>, hotspot_x: i32, hotspot_y: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_TOOL_V2_SET_CURSOR, serial, surface.map(Proxy::ptr).unwrap_or(ptr::null_mut()), hotspot_x, hotspot_y) };
            RequestResult::Sent(())
        }
        /// destroy the tool object
        ///
        /// This destroys the client's resource for this tool object.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_TOOL_V2_DESTROY) };

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
pub mod zwp_tablet_v2 {
    //! graphics tablet device
    //!
    //! The wp_tablet interface represents one graphics tablet device. The
    //! tablet interface itself does not generate events; all events are
    //! generated by wp_tablet_tool objects when in proximity above a tablet.
    //! 
    //! A tablet has a number of static characteristics, e.g. device name and
    //! pid/vid. These capabilities are sent in an event sequence after the
    //! wp_tablet_seat.tablet_added event. This initial event sequence is
    //! terminated by a wp_tablet.done event.
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

    pub struct ZwpTabletV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletV2 {}
    unsafe impl Sync for ZwpTabletV2 {}
    unsafe impl Proxy for ZwpTabletV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_v2_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_v2"  }
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

        fn equals(&self, other: &ZwpTabletV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletV2 {
            ZwpTabletV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletV2 {
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
                    let name = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(0) as *const *const _)).to_bytes()).into_owned()};
                    (implementation.name)(evq, idata,  self, name);
                },
                1 => {
                    let vid = {*(args.offset(0) as *const u32)};
                    let pid = {*(args.offset(1) as *const u32)};
                    (implementation.id)(evq, idata,  self, vid, pid);
                },
                2 => {
                    let path = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(0) as *const *const _)).to_bytes()).into_owned()};
                    (implementation.path)(evq, idata,  self, path);
                },
                3 => {
                    (implementation.done)(evq, idata,  self);
                },
                4 => {
                    (implementation.removed)(evq, idata,  self);
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
        /// tablet device name
        ///
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v2, name
        pub name: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v2: &ZwpTabletV2, name: String),
        /// tablet device USB vendor/product id
        ///
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v2, vid, pid
        pub id: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v2: &ZwpTabletV2, vid: u32, pid: u32),
        /// path to the device
        ///
        /// A system-specific device path that indicates which device is behind
        /// this wp_tablet. This information may be used to gather additional
        /// information about the device, e.g. through libwacom.
        /// 
        /// A device may have more than one device path. If so, multiple
        /// wp_tablet.path events are sent. A device may be emulated and not
        /// have a device path, and in that case this event will not be sent.
        /// 
        /// The format of the path is unspecified, it may be a device node, a
        /// sysfs path, or some other identifier. It is up to the client to
        /// identify the string provided.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v2, path
        pub path: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v2: &ZwpTabletV2, path: String),
        /// tablet description events sequence complete
        ///
        /// This event is sent immediately to signal the end of the initial
        /// burst of descriptive events. A client may consider the static
        /// description of the tablet to be complete and finalize initialization
        /// of the tablet.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v2
        pub done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v2: &ZwpTabletV2),
        /// tablet removed event
        ///
        /// Sent when the tablet has been removed from the system. When a tablet
        /// is removed, some tools may be removed.
        /// 
        /// When this event is received, the client must wp_tablet.destroy
        /// the object.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v2
        pub removed: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v2: &ZwpTabletV2),
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
            && (self.name as usize == other.name as usize)
            && (self.id as usize == other.id as usize)
            && (self.path as usize == other.path as usize)
            && (self.done as usize == other.done as usize)
            && (self.removed as usize == other.removed as usize)

        }
    }

    const ZWP_TABLET_V2_DESTROY: u32 = 0;
    impl ZwpTabletV2 {
        /// destroy the tablet object
        ///
        /// This destroys the client's resource for this tablet object.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_V2_DESTROY) };

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
pub mod zwp_tablet_pad_ring_v2 {
    //! pad ring
    //!
    //! A circular interaction area, such as the touch ring on the Wacom Intuos
    //! Pro series tablets.
    //! 
    //! Events on a ring are logically grouped by the wl_tablet_pad_ring.frame
    //! event.
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

    pub struct ZwpTabletPadRingV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletPadRingV2 {}
    unsafe impl Sync for ZwpTabletPadRingV2 {}
    unsafe impl Proxy for ZwpTabletPadRingV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletPadRingV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletPadRingV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletPadRingV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletPadRingV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletPadRingV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_pad_ring_v2_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_pad_ring_v2"  }
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

        fn equals(&self, other: &ZwpTabletPadRingV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletPadRingV2 {
            ZwpTabletPadRingV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletPadRingV2 {
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
                    let source = {match Source::from_raw(*(args.offset(0) as *const u32)) { Some(v) => v, Option::None => return Err(()) }};
                    (implementation.source)(evq, idata,  self, source);
                },
                1 => {
                    let degrees = {wl_fixed_to_double(*(args.offset(0) as *const i32))};
                    (implementation.angle)(evq, idata,  self, degrees);
                },
                2 => {
                    (implementation.stop)(evq, idata,  self);
                },
                3 => {
                    let time = {*(args.offset(0) as *const u32)};
                    (implementation.frame)(evq, idata,  self, time);
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
    /// ring axis source
    ///
    /// Describes the source types for ring events. This indicates to the
    /// client how a ring event was physically generated; a client may
    /// adjust the user interface accordingly. For example, events
    /// from a "finger" source may trigger kinetic scrolling.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Source {
        /// finger
        Finger = 1,
    }
    impl Source {
        pub fn from_raw(n: u32) -> Option<Source> {
            match n {
                1 => Some(Source::Finger),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// ring event source
        ///
        /// Source information for ring events.
        /// 
        /// This event does not occur on its own. It is sent before a
        /// wp_tablet_pad_ring.frame event and carries the source information
        /// for all events within that frame.
        /// 
        /// The source specifies how this event was generated. If the source is
        /// wp_tablet_pad_ring.source.finger, a wp_tablet_pad_ring.stop event
        /// will be sent when the user lifts the finger off the device.
        /// 
        /// This event is optional. If the source is unknown for an interaction,
        /// no event is sent.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_ring_v2, source
        pub source: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_ring_v2: &ZwpTabletPadRingV2, source: Source),
        /// angle changed
        ///
        /// Sent whenever the angle on a ring changes.
        /// 
        /// The angle is provided in degrees clockwise from the logical
        /// north of the ring in the pad's current rotation.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_ring_v2, degrees
        pub angle: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_ring_v2: &ZwpTabletPadRingV2, degrees: f64),
        /// interaction stopped
        ///
        /// Stop notification for ring events.
        /// 
        /// For some wp_tablet_pad_ring.source types, a wp_tablet_pad_ring.stop
        /// event is sent to notify a client that the interaction with the ring
        /// has terminated. This enables the client to implement kinetic scrolling.
        /// See the wp_tablet_pad_ring.source documentation for information on
        /// when this event may be generated.
        /// 
        /// Any wp_tablet_pad_ring.angle events with the same source after this
        /// event should be considered as the start of a new interaction.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_ring_v2
        pub stop: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_ring_v2: &ZwpTabletPadRingV2),
        /// end of a ring event sequence
        ///
        /// Indicates the end of a set of ring events that logically belong
        /// together. A client is expected to accumulate the data in all events
        /// within the frame before proceeding.
        /// 
        /// All wp_tablet_pad_ring events before a wp_tablet_pad_ring.frame event belong
        /// logically together. For example, on termination of a finger interaction
        /// on a ring the compositor will send a wp_tablet_pad_ring.source event,
        /// a wp_tablet_pad_ring.stop event and a wp_tablet_pad_ring.frame event.
        /// 
        /// A wp_tablet_pad_ring.frame event is sent for every logical event
        /// group, even if the group only contains a single wp_tablet_pad_ring
        /// event. Specifically, a client may get a sequence: angle, frame,
        /// angle, frame, etc.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_ring_v2, time
        pub frame: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_ring_v2: &ZwpTabletPadRingV2, time: u32),
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
            && (self.source as usize == other.source as usize)
            && (self.angle as usize == other.angle as usize)
            && (self.stop as usize == other.stop as usize)
            && (self.frame as usize == other.frame as usize)

        }
    }

    const ZWP_TABLET_PAD_RING_V2_SET_FEEDBACK: u32 = 0;
    const ZWP_TABLET_PAD_RING_V2_DESTROY: u32 = 1;
    impl ZwpTabletPadRingV2 {
        /// set compositor feedback
        ///
        /// Request that the compositor use the provided feedback string
        /// associated with this ring. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever the ring is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with the ring; compositors may use this
        /// information to offer visual feedback about the button layout
        /// (eg. on-screen displays).
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// ring. Requests providing other serials than the most recent one will be
        /// ignored.
        pub fn set_feedback(&self, description: String, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let description = CString::new(description).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_tablet_pad_ring_v2.set_feedback:description"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_PAD_RING_V2_SET_FEEDBACK, description.as_ptr(), serial) };
            RequestResult::Sent(())
        }
        /// destroy the ring object
        ///
        /// This destroys the client's resource for this ring object.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_PAD_RING_V2_DESTROY) };

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
pub mod zwp_tablet_pad_strip_v2 {
    //! pad strip
    //!
    //! A linear interaction area, such as the strips found in Wacom Cintiq
    //! models.
    //! 
    //! Events on a strip are logically grouped by the wl_tablet_pad_strip.frame
    //! event.
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

    pub struct ZwpTabletPadStripV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletPadStripV2 {}
    unsafe impl Sync for ZwpTabletPadStripV2 {}
    unsafe impl Proxy for ZwpTabletPadStripV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletPadStripV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletPadStripV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletPadStripV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletPadStripV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletPadStripV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_pad_strip_v2_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_pad_strip_v2"  }
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

        fn equals(&self, other: &ZwpTabletPadStripV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletPadStripV2 {
            ZwpTabletPadStripV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletPadStripV2 {
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
                    let source = {match Source::from_raw(*(args.offset(0) as *const u32)) { Some(v) => v, Option::None => return Err(()) }};
                    (implementation.source)(evq, idata,  self, source);
                },
                1 => {
                    let position = {*(args.offset(0) as *const u32)};
                    (implementation.position)(evq, idata,  self, position);
                },
                2 => {
                    (implementation.stop)(evq, idata,  self);
                },
                3 => {
                    let time = {*(args.offset(0) as *const u32)};
                    (implementation.frame)(evq, idata,  self, time);
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
    /// strip axis source
    ///
    /// Describes the source types for strip events. This indicates to the
    /// client how a strip event was physically generated; a client may
    /// adjust the user interface accordingly. For example, events
    /// from a "finger" source may trigger kinetic scrolling.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Source {
        /// finger
        Finger = 1,
    }
    impl Source {
        pub fn from_raw(n: u32) -> Option<Source> {
            match n {
                1 => Some(Source::Finger),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// strip event source
        ///
        /// Source information for strip events.
        /// 
        /// This event does not occur on its own. It is sent before a
        /// wp_tablet_pad_strip.frame event and carries the source information
        /// for all events within that frame.
        /// 
        /// The source specifies how this event was generated. If the source is
        /// wp_tablet_pad_strip.source.finger, a wp_tablet_pad_strip.stop event
        /// will be sent when the user lifts their finger off the device.
        /// 
        /// This event is optional. If the source is unknown for an interaction,
        /// no event is sent.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_strip_v2, source
        pub source: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_strip_v2: &ZwpTabletPadStripV2, source: Source),
        /// position changed
        ///
        /// Sent whenever the position on a strip changes.
        /// 
        /// The position is normalized to a range of [0, 65535], the 0-value
        /// represents the top-most and/or left-most position of the strip in
        /// the pad's current rotation.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_strip_v2, position
        pub position: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_strip_v2: &ZwpTabletPadStripV2, position: u32),
        /// interaction stopped
        ///
        /// Stop notification for strip events.
        /// 
        /// For some wp_tablet_pad_strip.source types, a wp_tablet_pad_strip.stop
        /// event is sent to notify a client that the interaction with the strip
        /// has terminated. This enables the client to implement kinetic
        /// scrolling. See the wp_tablet_pad_strip.source documentation for
        /// information on when this event may be generated.
        /// 
        /// Any wp_tablet_pad_strip.position events with the same source after this
        /// event should be considered as the start of a new interaction.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_strip_v2
        pub stop: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_strip_v2: &ZwpTabletPadStripV2),
        /// end of a strip event sequence
        ///
        /// Indicates the end of a set of events that represent one logical
        /// hardware strip event. A client is expected to accumulate the data
        /// in all events within the frame before proceeding.
        /// 
        /// All wp_tablet_pad_strip events before a wp_tablet_pad_strip.frame event belong
        /// logically together. For example, on termination of a finger interaction
        /// on a strip the compositor will send a wp_tablet_pad_strip.source event,
        /// a wp_tablet_pad_strip.stop event and a wp_tablet_pad_strip.frame
        /// event.
        /// 
        /// A wp_tablet_pad_strip.frame event is sent for every logical event
        /// group, even if the group only contains a single wp_tablet_pad_strip
        /// event. Specifically, a client may get a sequence: position, frame,
        /// position, frame, etc.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_strip_v2, time
        pub frame: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_strip_v2: &ZwpTabletPadStripV2, time: u32),
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
            && (self.source as usize == other.source as usize)
            && (self.position as usize == other.position as usize)
            && (self.stop as usize == other.stop as usize)
            && (self.frame as usize == other.frame as usize)

        }
    }

    const ZWP_TABLET_PAD_STRIP_V2_SET_FEEDBACK: u32 = 0;
    const ZWP_TABLET_PAD_STRIP_V2_DESTROY: u32 = 1;
    impl ZwpTabletPadStripV2 {
        /// set compositor feedback
        ///
        /// Requests the compositor to use the provided feedback string
        /// associated with this strip. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever the strip is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with the strip, and compositors may use this
        /// information to offer visual feedback about the button layout
        /// (eg. on-screen displays).
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// strip. Requests providing other serials than the most recent one will be
        /// ignored.
        pub fn set_feedback(&self, description: String, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let description = CString::new(description).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_tablet_pad_strip_v2.set_feedback:description"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_PAD_STRIP_V2_SET_FEEDBACK, description.as_ptr(), serial) };
            RequestResult::Sent(())
        }
        /// destroy the strip object
        ///
        /// This destroys the client's resource for this strip object.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_PAD_STRIP_V2_DESTROY) };

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
pub mod zwp_tablet_pad_group_v2 {
    //! a set of buttons, rings and strips
    //!
    //! A pad group describes a distinct (sub)set of buttons, rings and strips
    //! present in the tablet. The criteria of this grouping is usually positional,
    //! eg. if a tablet has buttons on the left and right side, 2 groups will be
    //! presented. The physical arrangement of groups is undisclosed and may
    //! change on the fly.
    //! 
    //! Pad groups will announce their features during pad initialization. Between
    //! the corresponding wp_tablet_pad.group event and wp_tablet_pad_group.done, the
    //! pad group will announce the buttons, rings and strips contained in it,
    //! plus the number of supported modes.
    //! 
    //! Modes are a mechanism to allow multiple groups of actions for every element
    //! in the pad group. The number of groups and available modes in each is
    //! persistent across device plugs. The current mode is user-switchable, it
    //! will be announced through the wp_tablet_pad_group.mode_switch event both
    //! whenever it is switched, and after wp_tablet_pad.enter.
    //! 
    //! The current mode logically applies to all elements in the pad group,
    //! although it is at clients' discretion whether to actually perform different
    //! actions, and/or issue the respective .set_feedback requests to notify the
    //! compositor. See the wp_tablet_pad_group.mode_switch event for more details.
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

    pub struct ZwpTabletPadGroupV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletPadGroupV2 {}
    unsafe impl Sync for ZwpTabletPadGroupV2 {}
    unsafe impl Proxy for ZwpTabletPadGroupV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletPadGroupV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletPadGroupV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletPadGroupV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletPadGroupV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletPadGroupV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_pad_group_v2_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_pad_group_v2"  }
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

        fn equals(&self, other: &ZwpTabletPadGroupV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletPadGroupV2 {
            ZwpTabletPadGroupV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletPadGroupV2 {
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
                    let buttons = {let array = *(args.offset(0) as *const *mut wl_array); ::std::slice::from_raw_parts((*array).data as *const u8, (*array).size as usize).to_owned()};
                    (implementation.buttons)(evq, idata,  self, buttons);
                },
                1 => {
                    let ring = {Proxy::from_ptr_new(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.ring)(evq, idata,  self, ring);
                },
                2 => {
                    let strip = {Proxy::from_ptr_new(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.strip)(evq, idata,  self, strip);
                },
                3 => {
                    let modes = {*(args.offset(0) as *const u32)};
                    (implementation.modes)(evq, idata,  self, modes);
                },
                4 => {
                    (implementation.done)(evq, idata,  self);
                },
                5 => {
                    let time = {*(args.offset(0) as *const u32)};
                    let serial = {*(args.offset(1) as *const u32)};
                    let mode = {*(args.offset(2) as *const u32)};
                    (implementation.mode_switch)(evq, idata,  self, time, serial, mode);
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
        /// buttons announced
        ///
        /// Sent on wp_tablet_pad_group initialization to announce the available
        /// buttons in the group. Button indices start at 0, a button may only be
        /// in one group at a time.
        /// 
        /// This event is first sent in the initial burst of events before the
        /// wp_tablet_pad_group.done event.
        /// 
        /// Some buttons are reserved by the compositor. These buttons may not be
        /// assigned to any wp_tablet_pad_group. Compositors may broadcast this
        /// event in the case of changes to the mapping of these reserved buttons.
        /// If the compositor happens to reserve all buttons in a group, this event
        /// will be sent with an empty array.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_group_v2, buttons
        pub buttons: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_group_v2: &ZwpTabletPadGroupV2, buttons: Vec<u8>),
        /// ring announced
        ///
        /// Sent on wp_tablet_pad_group initialization to announce available rings.
        /// One event is sent for each ring available on this pad group.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad_group.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_group_v2, ring
        pub ring: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_group_v2: &ZwpTabletPadGroupV2, ring: super::zwp_tablet_pad_ring_v2::ZwpTabletPadRingV2),
        /// strip announced
        ///
        /// Sent on wp_tablet_pad initialization to announce available strips.
        /// One event is sent for each strip available on this pad group.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad_group.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_group_v2, strip
        pub strip: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_group_v2: &ZwpTabletPadGroupV2, strip: super::zwp_tablet_pad_strip_v2::ZwpTabletPadStripV2),
        /// mode-switch ability announced
        ///
        /// Sent on wp_tablet_pad_group initialization to announce that the pad
        /// group may switch between modes. A client may use a mode to store a
        /// specific configuration for buttons, rings and strips and use the
        /// wl_tablet_pad_group.mode_switch event to toggle between these
        /// configurations. Mode indices start at 0.
        /// 
        /// Switching modes is compositor-dependent. See the
        /// wp_tablet_pad_group.mode_switch event for more details.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad_group.done event. This event is only sent when more than
        /// more than one mode is available.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_group_v2, modes
        pub modes: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_group_v2: &ZwpTabletPadGroupV2, modes: u32),
        /// tablet group description events sequence complete
        ///
        /// This event is sent immediately to signal the end of the initial
        /// burst of descriptive events. A client may consider the static
        /// description of the tablet to be complete and finalize initialization
        /// of the tablet group.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_group_v2
        pub done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_group_v2: &ZwpTabletPadGroupV2),
        /// mode switch event
        ///
        /// Notification that the mode was switched.
        /// 
        /// A mode applies to all buttons, rings and strips in a group
        /// simultaneously, but a client is not required to assign different actions
        /// for each mode. For example, a client may have mode-specific button
        /// mappings but map the ring to vertical scrolling in all modes. Mode
        /// indices start at 0.
        /// 
        /// Switching modes is compositor-dependent. The compositor may provide
        /// visual cues to the client about the mode, e.g. by toggling LEDs on
        /// the tablet device. Mode-switching may be software-controlled or
        /// controlled by one or more physical buttons. For example, on a Wacom
        /// Intuos Pro, the button inside the ring may be assigned to switch
        /// between modes.
        /// 
        /// The compositor will also send this event after wp_tablet_pad.enter on
        /// each group in order to notify of the current mode. Groups that only
        /// feature one mode will use mode=0 when emitting this event.
        /// 
        /// If a button action in the new mode differs from the action in the
        /// previous mode, the client should immediately issue a
        /// wp_tablet_pad.set_feedback request for each changed button.
        /// 
        /// If a ring or strip action in the new mode differs from the action
        /// in the previous mode, the client should immediately issue a
        /// wp_tablet_ring.set_feedback or wp_tablet_strip.set_feedback request
        /// for each changed ring or strip.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_group_v2, time, serial, mode
        pub mode_switch: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_group_v2: &ZwpTabletPadGroupV2, time: u32, serial: u32, mode: u32),
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
            && (self.buttons as usize == other.buttons as usize)
            && (self.ring as usize == other.ring as usize)
            && (self.strip as usize == other.strip as usize)
            && (self.modes as usize == other.modes as usize)
            && (self.done as usize == other.done as usize)
            && (self.mode_switch as usize == other.mode_switch as usize)

        }
    }

    const ZWP_TABLET_PAD_GROUP_V2_DESTROY: u32 = 0;
    impl ZwpTabletPadGroupV2 {
        /// destroy the pad object
        ///
        /// Destroy the wp_tablet_pad_group object. Objects created from this object
        /// are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_PAD_GROUP_V2_DESTROY) };

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
pub mod zwp_tablet_pad_v2 {
    //! a set of buttons, rings and strips
    //!
    //! A pad device is a set of buttons, rings and strips
    //! usually physically present on the tablet device itself. Some
    //! exceptions exist where the pad device is physically detached, e.g. the
    //! Wacom ExpressKey Remote.
    //! 
    //! Pad devices have no axes that control the cursor and are generally
    //! auxiliary devices to the tool devices used on the tablet surface.
    //! 
    //! A pad device has a number of static characteristics, e.g. the number
    //! of rings. These capabilities are sent in an event sequence after the
    //! wp_tablet_seat.pad_added event before any actual events from this pad.
    //! This initial event sequence is terminated by a wp_tablet_pad.done
    //! event.
    //! 
    //! All pad features (buttons, rings and strips) are logically divided into
    //! groups and all pads have at least one group. The available groups are
    //! notified through the wp_tablet_pad.group event; the compositor will
    //! emit one event per group before emitting wp_tablet_pad.done.
    //! 
    //! Groups may have multiple modes. Modes allow clients to map multiple
    //! actions to a single pad feature. Only one mode can be active per group,
    //! although different groups may have different active modes.
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

    pub struct ZwpTabletPadV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletPadV2 {}
    unsafe impl Sync for ZwpTabletPadV2 {}
    unsafe impl Proxy for ZwpTabletPadV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletPadV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletPadV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletPadV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletPadV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletPadV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_pad_v2_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_pad_v2"  }
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

        fn equals(&self, other: &ZwpTabletPadV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletPadV2 {
            ZwpTabletPadV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletPadV2 {
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
                    let pad_group = {Proxy::from_ptr_new(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.group)(evq, idata,  self, pad_group);
                },
                1 => {
                    let path = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(0) as *const *const _)).to_bytes()).into_owned()};
                    (implementation.path)(evq, idata,  self, path);
                },
                2 => {
                    let buttons = {*(args.offset(0) as *const u32)};
                    (implementation.buttons)(evq, idata,  self, buttons);
                },
                3 => {
                    (implementation.done)(evq, idata,  self);
                },
                4 => {
                    let time = {*(args.offset(0) as *const u32)};
                    let button = {*(args.offset(1) as *const u32)};
                    let state = {match ButtonState::from_raw(*(args.offset(2) as *const u32)) { Some(v) => v, Option::None => return Err(()) }};
                    (implementation.button)(evq, idata,  self, time, button, state);
                },
                5 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let tablet = {Proxy::from_ptr_initialized(*(args.offset(1) as *const *mut wl_proxy))};
                    let surface = {Proxy::from_ptr_initialized(*(args.offset(2) as *const *mut wl_proxy))};
                    (implementation.enter)(evq, idata,  self, serial, &tablet, &surface);
                },
                6 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let surface = {Proxy::from_ptr_initialized(*(args.offset(1) as *const *mut wl_proxy))};
                    (implementation.leave)(evq, idata,  self, serial, &surface);
                },
                7 => {
                    (implementation.removed)(evq, idata,  self);
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
    /// physical button state
    ///
    /// Describes the physical state of a button that caused the button
    /// event.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ButtonState {
        /// the button is not pressed
        Released = 0,
        /// the button is pressed
        Pressed = 1,
    }
    impl ButtonState {
        pub fn from_raw(n: u32) -> Option<ButtonState> {
            match n {
                0 => Some(ButtonState::Released),
                1 => Some(ButtonState::Pressed),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// group announced
        ///
        /// Sent on wp_tablet_pad initialization to announce available groups.
        /// One event is sent for each pad group available.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad.done event. At least one group will be announced.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_v2, pad_group
        pub group: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_v2: &ZwpTabletPadV2, pad_group: super::zwp_tablet_pad_group_v2::ZwpTabletPadGroupV2),
        /// path to the device
        ///
        /// A system-specific device path that indicates which device is behind
        /// this wp_tablet_pad. This information may be used to gather additional
        /// information about the device, e.g. through libwacom.
        /// 
        /// The format of the path is unspecified, it may be a device node, a
        /// sysfs path, or some other identifier. It is up to the client to
        /// identify the string provided.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_v2, path
        pub path: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_v2: &ZwpTabletPadV2, path: String),
        /// buttons announced
        ///
        /// Sent on wp_tablet_pad initialization to announce the available
        /// buttons.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_pad.done event. This event is only sent when at least one
        /// button is available.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_v2, buttons
        pub buttons: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_v2: &ZwpTabletPadV2, buttons: u32),
        /// pad description event sequence complete
        ///
        /// This event signals the end of the initial burst of descriptive
        /// events. A client may consider the static description of the pad to
        /// be complete and finalize initialization of the pad.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_v2
        pub done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_v2: &ZwpTabletPadV2),
        /// physical button state
        ///
        /// Sent whenever the physical state of a button changes.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_v2, time, button, state
        pub button: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_v2: &ZwpTabletPadV2, time: u32, button: u32, state: ButtonState),
        /// enter event
        ///
        /// Notification that this pad is focused on the specified surface.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_v2, serial, tablet, surface
        pub enter: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_v2: &ZwpTabletPadV2, serial: u32, tablet: &super::zwp_tablet_v2::ZwpTabletV2, surface: &super::wl_surface::WlSurface),
        /// enter event
        ///
        /// Notification that this pad is no longer focused on the specified
        /// surface.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_v2, serial, surface
        pub leave: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_v2: &ZwpTabletPadV2, serial: u32, surface: &super::wl_surface::WlSurface),
        /// pad removed event
        ///
        /// Sent when the pad has been removed from the system. When a tablet
        /// is removed its pad(s) will be removed too.
        /// 
        /// When this event is received, the client must destroy all rings, strips
        /// and groups that were offered by this pad, and issue wp_tablet_pad.destroy
        /// the pad itself.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_pad_v2
        pub removed: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_pad_v2: &ZwpTabletPadV2),
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
            && (self.group as usize == other.group as usize)
            && (self.path as usize == other.path as usize)
            && (self.buttons as usize == other.buttons as usize)
            && (self.done as usize == other.done as usize)
            && (self.button as usize == other.button as usize)
            && (self.enter as usize == other.enter as usize)
            && (self.leave as usize == other.leave as usize)
            && (self.removed as usize == other.removed as usize)

        }
    }

    const ZWP_TABLET_PAD_V2_SET_FEEDBACK: u32 = 0;
    const ZWP_TABLET_PAD_V2_DESTROY: u32 = 1;
    impl ZwpTabletPadV2 {
        /// set compositor feedback
        ///
        /// Requests the compositor to use the provided feedback string
        /// associated with this button. This request should be issued immediately
        /// after a wp_tablet_pad_group.mode_switch event from the corresponding
        /// group is received, or whenever a button is mapped to a different
        /// action. See wp_tablet_pad_group.mode_switch for more details.
        /// 
        /// Clients are encouraged to provide context-aware descriptions for
        /// the actions associated with each button, and compositors may use
        /// this information to offer visual feedback on the button layout
        /// (e.g. on-screen displays).
        /// 
        /// Button indices start at 0. Setting the feedback string on a button
        /// that is reserved by the compositor (i.e. not belonging to any
        /// wp_tablet_pad_group) does not generate an error but the compositor
        /// is free to ignore the request.
        /// 
        /// The provided string 'description' is a UTF-8 encoded string to be
        /// associated with this ring, and is considered user-visible; general
        /// internationalization rules apply.
        /// 
        /// The serial argument will be that of the last
        /// wp_tablet_pad_group.mode_switch event received for the group of this
        /// button. Requests providing other serials than the most recent one will
        /// be ignored.
        pub fn set_feedback(&self, button: u32, description: String, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let description = CString::new(description).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_tablet_pad_v2.set_feedback:description"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_PAD_V2_SET_FEEDBACK, button, description.as_ptr(), serial) };
            RequestResult::Sent(())
        }
        /// destroy the pad object
        ///
        /// Destroy the wp_tablet_pad object. Objects created from this object
        /// are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_PAD_V2_DESTROY) };

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
