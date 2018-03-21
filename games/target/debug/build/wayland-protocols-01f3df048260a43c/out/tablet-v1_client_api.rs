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

pub mod zwp_tablet_manager_v1 {
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

    pub struct ZwpTabletManagerV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletManagerV1 {}
    unsafe impl Sync for ZwpTabletManagerV1 {}
    unsafe impl Proxy for ZwpTabletManagerV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletManagerV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletManagerV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletManagerV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_manager_v1_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_manager_v1"  }
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

        fn equals(&self, other: &ZwpTabletManagerV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletManagerV1 {
            ZwpTabletManagerV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZWP_TABLET_MANAGER_V1_GET_TABLET_SEAT: u32 = 0;
    const ZWP_TABLET_MANAGER_V1_DESTROY: u32 = 1;
    impl ZwpTabletManagerV1 {
        /// get the tablet seat
        ///
        /// Get the wp_tablet_seat object for the given seat. This object
        /// provides access to all graphics tablets in this seat.
        pub fn get_tablet_seat(&self, seat: &super::wl_seat::WlSeat) ->RequestResult<super::zwp_tablet_seat_v1::ZwpTabletSeatV1> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_TABLET_MANAGER_V1_GET_TABLET_SEAT, &zwp_tablet_seat_v1_interface, ptr::null_mut::<wl_proxy>(), seat.ptr()) };
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_MANAGER_V1_DESTROY) };

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
pub mod zwp_tablet_seat_v1 {
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

    pub struct ZwpTabletSeatV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletSeatV1 {}
    unsafe impl Sync for ZwpTabletSeatV1 {}
    unsafe impl Proxy for ZwpTabletSeatV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletSeatV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletSeatV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletSeatV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletSeatV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletSeatV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_seat_v1_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_seat_v1"  }
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

        fn equals(&self, other: &ZwpTabletSeatV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletSeatV1 {
            ZwpTabletSeatV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletSeatV1 {
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_seat_v1, id
        pub tablet_added: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_seat_v1: &ZwpTabletSeatV1, id: super::zwp_tablet_v1::ZwpTabletV1),
        /// a new tool has been used with a tablet
        ///
        /// This event is sent whenever a tool that has not previously been used
        /// with a tablet comes into use. This event only provides the object id
        /// of the tool; any static information about the tool (capabilities,
        /// type, etc.) is sent through the wp_tablet_tool interface.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_seat_v1, id
        pub tool_added: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_seat_v1: &ZwpTabletSeatV1, id: super::zwp_tablet_tool_v1::ZwpTabletToolV1),
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

        }
    }

    const ZWP_TABLET_SEAT_V1_DESTROY: u32 = 0;
    impl ZwpTabletSeatV1 {
        /// release the memory for the tablet seat object
        ///
        /// Destroy the wp_tablet_seat object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_SEAT_V1_DESTROY) };

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
pub mod zwp_tablet_tool_v1 {
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

    pub struct ZwpTabletToolV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletToolV1 {}
    unsafe impl Sync for ZwpTabletToolV1 {}
    unsafe impl Proxy for ZwpTabletToolV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletToolV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletToolV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletToolV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletToolV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletToolV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_tool_v1_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_tool_v1"  }
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

        fn equals(&self, other: &ZwpTabletToolV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletToolV1 {
            ZwpTabletToolV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletToolV1 {
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
                    let tilt_x = {*(args.offset(0) as *const i32)};
                    let tilt_y = {*(args.offset(1) as *const i32)};
                    (implementation.tilt)(evq, idata,  self, tilt_x, tilt_y);
                },
                14 => {
                    let degrees = {*(args.offset(0) as *const i32)};
                    (implementation.rotation)(evq, idata,  self, degrees);
                },
                15 => {
                    let position = {*(args.offset(0) as *const i32)};
                    (implementation.slider)(evq, idata,  self, position);
                },
                16 => {
                    let degrees = {*(args.offset(0) as *const i32)};
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, tool_type
        pub type_: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, tool_type: Type),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, hardware_serial_hi, hardware_serial_lo
        pub hardware_serial: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, hardware_serial_hi: u32, hardware_serial_lo: u32),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, hardware_id_hi, hardware_id_lo
        pub hardware_id_wacom: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, hardware_id_hi: u32, hardware_id_lo: u32),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, capability
        pub capability: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, capability: Capability),
        /// tool description events sequence complete
        ///
        /// This event signals the end of the initial burst of descriptive
        /// events. A client may consider the static description of the tool to
        /// be complete and finalize initialization of the tool.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1
        pub done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1
        pub removed: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, serial, tablet, surface
        pub proximity_in: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, serial: u32, tablet: &super::zwp_tablet_v1::ZwpTabletV1, surface: &super::wl_surface::WlSurface),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1
        pub proximity_out: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, serial
        pub down: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, serial: u32),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1
        pub up: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1),
        /// motion event
        ///
        /// Sent whenever a tablet tool moves.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, x, y
        pub motion: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, x: f64, y: f64),
        /// pressure change event
        ///
        /// Sent whenever the pressure axis on a tool changes. The value of this
        /// event is normalized to a value between 0 and 65535.
        /// 
        /// Note that pressure may be nonzero even when a tool is not in logical
        /// contact. See the down and up events for more details.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, pressure
        pub pressure: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, pressure: u32),
        /// distance change event
        ///
        /// Sent whenever the distance axis on a tool changes. The value of this
        /// event is normalized to a value between 0 and 65535.
        /// 
        /// Note that distance may be nonzero even when a tool is not in logical
        /// contact. See the down and up events for more details.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, distance
        pub distance: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, distance: u32),
        /// tilt change event
        ///
        /// Sent whenever one or both of the tilt axes on a tool change. Each tilt
        /// value is in 0.01 of a degree, relative to the z-axis of the tablet.
        /// The angle is positive when the top of a tool tilts along the
        /// positive x or y axis.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, tilt_x, tilt_y
        pub tilt: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, tilt_x: i32, tilt_y: i32),
        /// z-rotation change event
        ///
        /// Sent whenever the z-rotation axis on the tool changes. The
        /// rotation value is in 0.01 of a degree clockwise from the tool's
        /// logical neutral position.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, degrees
        pub rotation: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, degrees: i32),
        /// Slider position change event
        ///
        /// Sent whenever the slider position on the tool changes. The
        /// value is normalized between -65535 and 65535, with 0 as the logical
        /// neutral position of the slider.
        /// 
        /// The slider is available on e.g. the Wacom Airbrush tool.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, position
        pub slider: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, position: i32),
        /// Wheel delta event
        ///
        /// Sent whenever the wheel on the tool emits an event. This event
        /// contains two values for the same axis change. The degrees value is
        /// in 0.01 of a degree in the same orientation as the
        /// wl_pointer.vertical_scroll axis. The clicks value is in discrete
        /// logical clicks of the mouse wheel. This value may be zero if the
        /// movement of the wheel was less than one logical click.
        /// 
        /// Clients should choose either value and avoid mixing degrees and
        /// clicks. The compositor may accumulate values smaller than a logical
        /// click and emulate click events when a certain threshold is met.
        /// Thus, wl_tablet_tool.wheel events with non-zero clicks values may
        /// have different degrees values.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, degrees, clicks
        pub wheel: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, degrees: i32, clicks: i32),
        /// button event
        ///
        /// Sent whenever a button on the tool is pressed or released.
        /// 
        /// If a button is held down when the tool moves in or out of proximity,
        /// button events are generated by the compositor. See
        /// wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for
        /// details.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, serial, button, state
        pub button: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, serial: u32, button: u32, state: ButtonState),
        /// frame event
        ///
        /// Marks the end of a series of axis and/or button updates from the
        /// tablet. The Wayland protocol requires axis updates to be sent
        /// sequentially, however all events within a frame should be considered
        /// one hardware event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_tool_v1, time
        pub frame: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_tool_v1: &ZwpTabletToolV1, time: u32),
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

    const ZWP_TABLET_TOOL_V1_SET_CURSOR: u32 = 0;
    const ZWP_TABLET_TOOL_V1_DESTROY: u32 = 1;
    impl ZwpTabletToolV1 {
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
        /// This request gives the surface the role of a cursor. The role
        /// assigned by this request is the same as assigned by
        /// wl_pointer.set_cursor meaning the same surface can be
        /// used both as a wl_pointer cursor and a wp_tablet cursor. If the
        /// surface already has another role, it raises a protocol error.
        /// The surface may be used on multiple tablets and across multiple
        /// seats.
        pub fn set_cursor(&self, serial: u32, surface: Option<&super::wl_surface::WlSurface>, hotspot_x: i32, hotspot_y: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_TOOL_V1_SET_CURSOR, serial, surface.map(Proxy::ptr).unwrap_or(ptr::null_mut()), hotspot_x, hotspot_y) };
            RequestResult::Sent(())
        }
        /// destroy the tool object
        ///
        /// This destroys the client's resource for this tool object.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_TOOL_V1_DESTROY) };

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
pub mod zwp_tablet_v1 {
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

    pub struct ZwpTabletV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTabletV1 {}
    unsafe impl Sync for ZwpTabletV1 {}
    unsafe impl Proxy for ZwpTabletV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTabletV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTabletV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTabletV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTabletV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTabletV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_tablet_v1_interface } }
        fn interface_name() -> &'static str { "zwp_tablet_v1"  }
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

        fn equals(&self, other: &ZwpTabletV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTabletV1 {
            ZwpTabletV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTabletV1 {
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v1, name
        pub name: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v1: &ZwpTabletV1, name: String),
        /// tablet device USB vendor/product id
        ///
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v1, vid, pid
        pub id: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v1: &ZwpTabletV1, vid: u32, pid: u32),
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
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v1, path
        pub path: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v1: &ZwpTabletV1, path: String),
        /// tablet description events sequence complete
        ///
        /// This event is sent immediately to signal the end of the initial
        /// burst of descriptive events. A client may consider the static
        /// description of the tablet to be complete and finalize initialization
        /// of the tablet.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v1
        pub done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v1: &ZwpTabletV1),
        /// tablet removed event
        ///
        /// Sent when the tablet has been removed from the system. When a tablet
        /// is removed, some tools may be removed.
        /// 
        /// When this event is received, the client must wp_tablet.destroy
        /// the object.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_tablet_v1
        pub removed: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_tablet_v1: &ZwpTabletV1),
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

    const ZWP_TABLET_V1_DESTROY: u32 = 0;
    impl ZwpTabletV1 {
        /// destroy the tablet object
        ///
        /// This destroys the client's resource for this tablet object.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TABLET_V1_DESTROY) };

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
