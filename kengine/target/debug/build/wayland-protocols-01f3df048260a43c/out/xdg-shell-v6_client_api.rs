//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2008-2013 Kristian Høgsberg
    Copyright © 2013      Rafael Antognolli
    Copyright © 2013      Jasper St. Pierre
    Copyright © 2010-2013 Intel Corporation

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

pub mod zxdg_shell_v6 {
    //! create desktop-style surfaces
    //!
    //! xdg_shell allows clients to turn a wl_surface into a "real window"
    //! which can be dragged, resized, stacked, and moved around by the
    //! user. Everything about this interface is suited towards traditional
    //! desktop environments.
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

    pub struct ZxdgShellV6 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgShellV6 {}
    unsafe impl Sync for ZxdgShellV6 {}
    unsafe impl Proxy for ZxdgShellV6 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgShellV6 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgShellV6 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgShellV6 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgShellV6 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgShellV6 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_shell_v6_interface } }
        fn interface_name() -> &'static str { "zxdg_shell_v6"  }
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

        fn equals(&self, other: &ZxdgShellV6) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgShellV6 {
            ZxdgShellV6 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZxdgShellV6 {
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
                    let serial = {*(args.offset(0) as *const u32)};
                    (implementation.ping)(evq, idata,  self, serial);
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
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// given wl_surface has another role
        Role = 0,
        /// xdg_shell was destroyed before children
        DefunctSurfaces = 1,
        /// the client tried to map or destroy a non-topmost popup
        NotTheTopmostPopup = 2,
        /// the client specified an invalid popup parent surface
        InvalidPopupParent = 3,
        /// the client provided an invalid surface state
        InvalidSurfaceState = 4,
        /// the client provided an invalid positioner
        InvalidPositioner = 5,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                1 => Some(Error::DefunctSurfaces),
                2 => Some(Error::NotTheTopmostPopup),
                3 => Some(Error::InvalidPopupParent),
                4 => Some(Error::InvalidSurfaceState),
                5 => Some(Error::InvalidPositioner),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// check if the client is alive
        ///
        /// The ping event asks the client if it's still alive. Pass the
        /// serial specified in the event back to the compositor by sending
        /// a "pong" request back with the specified serial. See xdg_shell.ping.
        /// 
        /// Compositors can use this to determine if the client is still
        /// alive. It's unspecified what will happen if the client doesn't
        /// respond to the ping request, or in what timeframe. Clients should
        /// try to respond in a reasonable amount of time.
        /// 
        /// A compositor is free to ping in any way it wants, but a client must
        /// always respond to any xdg_shell object it created.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_shell_v6, serial
        pub ping: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_shell_v6: &ZxdgShellV6, serial: u32),
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
            && (self.ping as usize == other.ping as usize)

        }
    }

    const ZXDG_SHELL_V6_DESTROY: u32 = 0;
    const ZXDG_SHELL_V6_CREATE_POSITIONER: u32 = 1;
    const ZXDG_SHELL_V6_GET_XDG_SURFACE: u32 = 2;
    const ZXDG_SHELL_V6_PONG: u32 = 3;
    impl ZxdgShellV6 {
        /// destroy xdg_shell
        ///
        /// Destroy this xdg_shell object.
        /// 
        /// Destroying a bound xdg_shell object while there are surfaces
        /// still alive created by this xdg_shell object instance is illegal
        /// and will result in a protocol error.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_SHELL_V6_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// create a positioner object
        ///
        /// Create a positioner object. A positioner object is used to position
        /// surfaces relative to some parent surface. See the interface description
        /// and xdg_surface.get_popup for details.
        pub fn create_positioner(&self) ->RequestResult<super::zxdg_positioner_v6::ZxdgPositionerV6> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZXDG_SHELL_V6_CREATE_POSITIONER, &zxdg_positioner_v6_interface, ptr::null_mut::<wl_proxy>()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// create a shell surface from a surface
        ///
        /// This creates an xdg_surface for the given surface. While xdg_surface
        /// itself is not a role, the corresponding surface may only be assigned
        /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup.
        /// 
        /// This creates an xdg_surface for the given surface. An xdg_surface is
        /// used as basis to define a role to a given surface, such as xdg_toplevel
        /// or xdg_popup. It also manages functionality shared between xdg_surface
        /// based surface roles.
        /// 
        /// See the documentation of xdg_surface for more details about what an
        /// xdg_surface is and how it is used.
        pub fn get_xdg_surface(&self, surface: &super::wl_surface::WlSurface) ->RequestResult<super::zxdg_surface_v6::ZxdgSurfaceV6> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZXDG_SHELL_V6_GET_XDG_SURFACE, &zxdg_surface_v6_interface, ptr::null_mut::<wl_proxy>(), surface.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// respond to a ping event
        ///
        /// A client must respond to a ping event with a pong request or
        /// the client may be deemed unresponsive. See xdg_shell.ping.
        pub fn pong(&self, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_SHELL_V6_PONG, serial) };
            RequestResult::Sent(())
        }
    }
}
pub mod zxdg_positioner_v6 {
    //! child surface positioner
    //!
    //! The xdg_positioner provides a collection of rules for the placement of a
    //! child surface relative to a parent surface. Rules can be defined to ensure
    //! the child surface remains within the visible area's borders, and to
    //! specify how the child surface changes its position, such as sliding along
    //! an axis, or flipping around a rectangle. These positioner-created rules are
    //! constrained by the requirement that a child surface must intersect with or
    //! be at least partially adjacent to its parent surface.
    //! 
    //! See the various requests for details about possible rules.
    //! 
    //! At the time of the request, the compositor makes a copy of the rules
    //! specified by the xdg_positioner. Thus, after the request is complete the
    //! xdg_positioner object can be destroyed or reused; further changes to the
    //! object will have no effect on previous usages.
    //! 
    //! For an xdg_positioner object to be considered complete, it must have a
    //! non-zero size set by set_size, and a non-zero anchor rectangle set by
    //! set_anchor_rect. Passing an incomplete xdg_positioner object when
    //! positioning a surface raises an error.
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

    pub struct ZxdgPositionerV6 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgPositionerV6 {}
    unsafe impl Sync for ZxdgPositionerV6 {}
    unsafe impl Proxy for ZxdgPositionerV6 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgPositionerV6 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgPositionerV6 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgPositionerV6 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgPositionerV6 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgPositionerV6 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_positioner_v6_interface } }
        fn interface_name() -> &'static str { "zxdg_positioner_v6"  }
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

        fn equals(&self, other: &ZxdgPositionerV6) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgPositionerV6 {
            ZxdgPositionerV6 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// invalid input provided
        InvalidInput = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidInput),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    bitflags! {
        pub struct Anchor: u32 {
            /// the center of the anchor rectangle
            const None = 0;
            /// the top edge of the anchor rectangle
            const Top = 1;
            /// the bottom edge of the anchor rectangle
            const Bottom = 2;
            /// the left edge of the anchor rectangle
            const Left = 4;
            /// the right edge of the anchor rectangle
            const Right = 8;
        }
    }
    impl Anchor {
        pub fn from_raw(n: u32) -> Option<Anchor> {
            Some(Anchor::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    bitflags! {
        pub struct Gravity: u32 {
            /// center over the anchor edge
            const None = 0;
            /// position above the anchor edge
            const Top = 1;
            /// position below the anchor edge
            const Bottom = 2;
            /// position to the left of the anchor edge
            const Left = 4;
            /// position to the right of the anchor edge
            const Right = 8;
        }
    }
    impl Gravity {
        pub fn from_raw(n: u32) -> Option<Gravity> {
            Some(Gravity::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    bitflags! {
        /// constraint adjustments
        ///
        /// The constraint adjustment value define ways the compositor will adjust
        /// the position of the surface, if the unadjusted position would result
        /// in the surface being partly constrained.
        /// 
        /// Whether a surface is considered 'constrained' is left to the compositor
        /// to determine. For example, the surface may be partly outside the
        /// compositor's defined 'work area', thus necessitating the child surface's
        /// position be adjusted until it is entirely inside the work area.
        /// 
        /// The adjustments can be combined, according to a defined precedence: 1)
        /// Flip, 2) Slide, 3) Resize.
        pub struct ConstraintAdjustment: u32 {
            /// don't move the child surface when constrained
            ///
            /// Don't alter the surface position even if it is constrained on some
            /// axis, for example partially outside the edge of a monitor.
            const None = 0;
            /// move along the x axis until unconstrained
            ///
            /// Slide the surface along the x axis until it is no longer constrained.
            /// 
            /// First try to slide towards the direction of the gravity on the x axis
            /// until either the edge in the opposite direction of the gravity is
            /// unconstrained or the edge in the direction of the gravity is
            /// constrained.
            /// 
            /// Then try to slide towards the opposite direction of the gravity on the
            /// x axis until either the edge in the direction of the gravity is
            /// unconstrained or the edge in the opposite direction of the gravity is
            /// constrained.
            const SlideX = 1;
            /// move along the y axis until unconstrained
            ///
            /// Slide the surface along the y axis until it is no longer constrained.
            /// 
            /// First try to slide towards the direction of the gravity on the y axis
            /// until either the edge in the opposite direction of the gravity is
            /// unconstrained or the edge in the direction of the gravity is
            /// constrained.
            /// 
            /// Then try to slide towards the opposite direction of the gravity on the
            /// y axis until either the edge in the direction of the gravity is
            /// unconstrained or the edge in the opposite direction of the gravity is
            /// constrained.
            const SlideY = 2;
            /// invert the anchor and gravity on the x axis
            ///
            /// Invert the anchor and gravity on the x axis if the surface is
            /// constrained on the x axis. For example, if the left edge of the
            /// surface is constrained, the gravity is 'left' and the anchor is
            /// 'left', change the gravity to 'right' and the anchor to 'right'.
            /// 
            /// If the adjusted position also ends up being constrained, the resulting
            /// position of the flip_x adjustment will be the one before the
            /// adjustment.
            const FlipX = 4;
            /// invert the anchor and gravity on the y axis
            ///
            /// Invert the anchor and gravity on the y axis if the surface is
            /// constrained on the y axis. For example, if the bottom edge of the
            /// surface is constrained, the gravity is 'bottom' and the anchor is
            /// 'bottom', change the gravity to 'top' and the anchor to 'top'.
            /// 
            /// If the adjusted position also ends up being constrained, the resulting
            /// position of the flip_y adjustment will be the one before the
            /// adjustment.
            const FlipY = 8;
            /// horizontally resize the surface
            ///
            /// Resize the surface horizontally so that it is completely
            /// unconstrained.
            const ResizeX = 16;
            /// vertically resize the surface
            ///
            /// Resize the surface vertically so that it is completely unconstrained.
            const ResizeY = 32;
        }
    }
    impl ConstraintAdjustment {
        pub fn from_raw(n: u32) -> Option<ConstraintAdjustment> {
            Some(ConstraintAdjustment::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    const ZXDG_POSITIONER_V6_DESTROY: u32 = 0;
    const ZXDG_POSITIONER_V6_SET_SIZE: u32 = 1;
    const ZXDG_POSITIONER_V6_SET_ANCHOR_RECT: u32 = 2;
    const ZXDG_POSITIONER_V6_SET_ANCHOR: u32 = 3;
    const ZXDG_POSITIONER_V6_SET_GRAVITY: u32 = 4;
    const ZXDG_POSITIONER_V6_SET_CONSTRAINT_ADJUSTMENT: u32 = 5;
    const ZXDG_POSITIONER_V6_SET_OFFSET: u32 = 6;
    impl ZxdgPositionerV6 {
        /// destroy the xdg_positioner object
        ///
        /// Notify the compositor that the xdg_positioner will no longer be used.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POSITIONER_V6_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// set the size of the to-be positioned rectangle
        ///
        /// Set the size of the surface that is to be positioned with the positioner
        /// object. The size is in surface-local coordinates and corresponds to the
        /// window geometry. See xdg_surface.set_window_geometry.
        /// 
        /// If a zero or negative size is set the invalid_input error is raised.
        pub fn set_size(&self, width: i32, height: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POSITIONER_V6_SET_SIZE, width, height) };
            RequestResult::Sent(())
        }
        /// set the anchor rectangle within the parent surface
        ///
        /// Specify the anchor rectangle within the parent surface that the child
        /// surface will be placed relative to. The rectangle is relative to the
        /// window geometry as defined by xdg_surface.set_window_geometry of the
        /// parent surface. The rectangle must be at least 1x1 large.
        /// 
        /// When the xdg_positioner object is used to position a child surface, the
        /// anchor rectangle may not extend outside the window geometry of the
        /// positioned child's parent surface.
        /// 
        /// If a zero or negative size is set the invalid_input error is raised.
        pub fn set_anchor_rect(&self, x: i32, y: i32, width: i32, height: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POSITIONER_V6_SET_ANCHOR_RECT, x, y, width, height) };
            RequestResult::Sent(())
        }
        /// set anchor rectangle anchor edges
        ///
        /// Defines a set of edges for the anchor rectangle. These are used to
        /// derive an anchor point that the child surface will be positioned
        /// relative to. If two orthogonal edges are specified (e.g. 'top' and
        /// 'left'), then the anchor point will be the intersection of the edges
        /// (e.g. the top left position of the rectangle); otherwise, the derived
        /// anchor point will be centered on the specified edge, or in the center of
        /// the anchor rectangle if no edge is specified.
        /// 
        /// If two parallel anchor edges are specified (e.g. 'left' and 'right'),
        /// the invalid_input error is raised.
        pub fn set_anchor(&self, anchor: Anchor) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POSITIONER_V6_SET_ANCHOR, anchor) };
            RequestResult::Sent(())
        }
        /// set child surface gravity
        ///
        /// Defines in what direction a surface should be positioned, relative to
        /// the anchor point of the parent surface. If two orthogonal gravities are
        /// specified (e.g. 'bottom' and 'right'), then the child surface will be
        /// placed in the specified direction; otherwise, the child surface will be
        /// centered over the anchor point on any axis that had no gravity
        /// specified.
        /// 
        /// If two parallel gravities are specified (e.g. 'left' and 'right'), the
        /// invalid_input error is raised.
        pub fn set_gravity(&self, gravity: Gravity) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POSITIONER_V6_SET_GRAVITY, gravity) };
            RequestResult::Sent(())
        }
        /// set the adjustment to be done when constrained
        ///
        /// Specify how the window should be positioned if the originally intended
        /// position caused the surface to be constrained, meaning at least
        /// partially outside positioning boundaries set by the compositor. The
        /// adjustment is set by constructing a bitmask describing the adjustment to
        /// be made when the surface is constrained on that axis.
        /// 
        /// If no bit for one axis is set, the compositor will assume that the child
        /// surface should not change its position on that axis when constrained.
        /// 
        /// If more than one bit for one axis is set, the order of how adjustments
        /// are applied is specified in the corresponding adjustment descriptions.
        /// 
        /// The default adjustment is none.
        pub fn set_constraint_adjustment(&self, constraint_adjustment: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POSITIONER_V6_SET_CONSTRAINT_ADJUSTMENT, constraint_adjustment) };
            RequestResult::Sent(())
        }
        /// set surface position offset
        ///
        /// Specify the surface position offset relative to the position of the
        /// anchor on the anchor rectangle and the anchor on the surface. For
        /// example if the anchor of the anchor rectangle is at (x, y), the surface
        /// has the gravity bottom|right, and the offset is (ox, oy), the calculated
        /// surface position will be (x + ox, y + oy). The offset position of the
        /// surface is the one used for constraint testing. See
        /// set_constraint_adjustment.
        /// 
        /// An example use case is placing a popup menu on top of a user interface
        /// element, while aligning the user interface element of the parent surface
        /// with some user interface element placed somewhere in the popup surface.
        pub fn set_offset(&self, x: i32, y: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POSITIONER_V6_SET_OFFSET, x, y) };
            RequestResult::Sent(())
        }
    }
}
pub mod zxdg_surface_v6 {
    //! desktop user interface surface base interface
    //!
    //! An interface that may be implemented by a wl_surface, for
    //! implementations that provide a desktop-style user interface.
    //! 
    //! It provides a base set of functionality required to construct user
    //! interface elements requiring management by the compositor, such as
    //! toplevel windows, menus, etc. The types of functionality are split into
    //! xdg_surface roles.
    //! 
    //! Creating an xdg_surface does not set the role for a wl_surface. In order
    //! to map an xdg_surface, the client must create a role-specific object
    //! using, e.g., get_toplevel, get_popup. The wl_surface for any given
    //! xdg_surface can have at most one role, and may not be assigned any role
    //! not based on xdg_surface.
    //! 
    //! A role must be assigned before any other requests are made to the
    //! xdg_surface object.
    //! 
    //! The client must call wl_surface.commit on the corresponding wl_surface
    //! for the xdg_surface state to take effect.
    //! 
    //! Creating an xdg_surface from a wl_surface which has a buffer attached or
    //! committed is a client error, and any attempts by a client to attach or
    //! manipulate a buffer prior to the first xdg_surface.configure call must
    //! also be treated as errors.
    //! 
    //! For a surface to be mapped by the compositor, the following conditions
    //! must be met: (1) the client has assigned a xdg_surface based role to the
    //! surface, (2) the client has set and committed the xdg_surface state and
    //! the role dependent state to the surface and (3) the client has committed a
    //! buffer to the surface.
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

    pub struct ZxdgSurfaceV6 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgSurfaceV6 {}
    unsafe impl Sync for ZxdgSurfaceV6 {}
    unsafe impl Proxy for ZxdgSurfaceV6 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgSurfaceV6 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgSurfaceV6 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgSurfaceV6 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgSurfaceV6 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgSurfaceV6 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_surface_v6_interface } }
        fn interface_name() -> &'static str { "zxdg_surface_v6"  }
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

        fn equals(&self, other: &ZxdgSurfaceV6) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgSurfaceV6 {
            ZxdgSurfaceV6 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZxdgSurfaceV6 {
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
                    let serial = {*(args.offset(0) as *const u32)};
                    (implementation.configure)(evq, idata,  self, serial);
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
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        NotConstructed = 1,
        AlreadyConstructed = 2,
        UnconfiguredBuffer = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                1 => Some(Error::NotConstructed),
                2 => Some(Error::AlreadyConstructed),
                3 => Some(Error::UnconfiguredBuffer),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// suggest a surface change
        ///
        /// The configure event marks the end of a configure sequence. A configure
        /// sequence is a set of one or more events configuring the state of the
        /// xdg_surface, including the final xdg_surface.configure event.
        /// 
        /// Where applicable, xdg_surface surface roles will during a configure
        /// sequence extend this event as a latched state sent as events before the
        /// xdg_surface.configure event. Such events should be considered to make up
        /// a set of atomically applied configuration states, where the
        /// xdg_surface.configure commits the accumulated state.
        /// 
        /// Clients should arrange their surface for the new states, and then send
        /// an ack_configure request with the serial sent in this configure event at
        /// some point before committing the new surface.
        /// 
        /// If the client receives multiple configure events before it can respond
        /// to one, it is free to discard all but the last event it received.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_surface_v6, serial
        pub configure: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_surface_v6: &ZxdgSurfaceV6, serial: u32),
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
            && (self.configure as usize == other.configure as usize)

        }
    }

    const ZXDG_SURFACE_V6_DESTROY: u32 = 0;
    const ZXDG_SURFACE_V6_GET_TOPLEVEL: u32 = 1;
    const ZXDG_SURFACE_V6_GET_POPUP: u32 = 2;
    const ZXDG_SURFACE_V6_SET_WINDOW_GEOMETRY: u32 = 3;
    const ZXDG_SURFACE_V6_ACK_CONFIGURE: u32 = 4;
    impl ZxdgSurfaceV6 {
        /// destroy the xdg_surface
        ///
        /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
        /// after its role object has been destroyed.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_SURFACE_V6_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// assign the xdg_toplevel surface role
        ///
        /// This creates an xdg_toplevel object for the given xdg_surface and gives
        /// the associated wl_surface the xdg_toplevel role.
        /// 
        /// See the documentation of xdg_toplevel for more details about what an
        /// xdg_toplevel is and how it is used.
        pub fn get_toplevel(&self) ->RequestResult<super::zxdg_toplevel_v6::ZxdgToplevelV6> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZXDG_SURFACE_V6_GET_TOPLEVEL, &zxdg_toplevel_v6_interface, ptr::null_mut::<wl_proxy>()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// assign the xdg_popup surface role
        ///
        /// This creates an xdg_popup object for the given xdg_surface and gives the
        /// associated wl_surface the xdg_popup role.
        /// 
        /// See the documentation of xdg_popup for more details about what an
        /// xdg_popup is and how it is used.
        pub fn get_popup(&self, parent: &super::zxdg_surface_v6::ZxdgSurfaceV6, positioner: &super::zxdg_positioner_v6::ZxdgPositionerV6) ->RequestResult<super::zxdg_popup_v6::ZxdgPopupV6> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZXDG_SURFACE_V6_GET_POPUP, &zxdg_popup_v6_interface, ptr::null_mut::<wl_proxy>(), parent.ptr(), positioner.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// set the new window geometry
        ///
        /// The window geometry of a surface is its "visible bounds" from the
        /// user's perspective. Client-side decorations often have invisible
        /// portions like drop-shadows which should be ignored for the
        /// purposes of aligning, placing and constraining windows.
        /// 
        /// The window geometry is double buffered, and will be applied at the
        /// time wl_surface.commit of the corresponding wl_surface is called.
        /// 
        /// Once the window geometry of the surface is set, it is not possible to
        /// unset it, and it will remain the same until set_window_geometry is
        /// called again, even if a new subsurface or buffer is attached.
        /// 
        /// If never set, the value is the full bounds of the surface,
        /// including any subsurfaces. This updates dynamically on every
        /// commit. This unset is meant for extremely simple clients.
        /// 
        /// The arguments are given in the surface-local coordinate space of
        /// the wl_surface associated with this xdg_surface.
        /// 
        /// The width and height must be greater than zero. Setting an invalid size
        /// will raise an error. When applied, the effective window geometry will be
        /// the set window geometry clamped to the bounding rectangle of the
        /// combined geometry of the surface of the xdg_surface and the associated
        /// subsurfaces.
        pub fn set_window_geometry(&self, x: i32, y: i32, width: i32, height: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_SURFACE_V6_SET_WINDOW_GEOMETRY, x, y, width, height) };
            RequestResult::Sent(())
        }
        /// ack a configure event
        ///
        /// When a configure event is received, if a client commits the
        /// surface in response to the configure event, then the client
        /// must make an ack_configure request sometime before the commit
        /// request, passing along the serial of the configure event.
        /// 
        /// For instance, for toplevel surfaces the compositor might use this
        /// information to move a surface to the top left only when the client has
        /// drawn itself for the maximized or fullscreen state.
        /// 
        /// If the client receives multiple configure events before it
        /// can respond to one, it only has to ack the last configure event.
        /// 
        /// A client is not required to commit immediately after sending
        /// an ack_configure request - it may even ack_configure several times
        /// before its next surface commit.
        /// 
        /// A client may send multiple ack_configure requests before committing, but
        /// only the last request sent before a commit indicates which configure
        /// event the client really is responding to.
        pub fn ack_configure(&self, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_SURFACE_V6_ACK_CONFIGURE, serial) };
            RequestResult::Sent(())
        }
    }
}
pub mod zxdg_toplevel_v6 {
    //! toplevel surface
    //!
    //! This interface defines an xdg_surface role which allows a surface to,
    //! among other things, set window-like properties such as maximize,
    //! fullscreen, and minimize, set application-specific metadata like title and
    //! id, and well as trigger user interactive operations such as interactive
    //! resize and move.
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

    pub struct ZxdgToplevelV6 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgToplevelV6 {}
    unsafe impl Sync for ZxdgToplevelV6 {}
    unsafe impl Proxy for ZxdgToplevelV6 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgToplevelV6 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgToplevelV6 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgToplevelV6 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgToplevelV6 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgToplevelV6 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_toplevel_v6_interface } }
        fn interface_name() -> &'static str { "zxdg_toplevel_v6"  }
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

        fn equals(&self, other: &ZxdgToplevelV6) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgToplevelV6 {
            ZxdgToplevelV6 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZxdgToplevelV6 {
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
                    let width = {*(args.offset(0) as *const i32)};
                    let height = {*(args.offset(1) as *const i32)};
                    let states = {let array = *(args.offset(2) as *const *mut wl_array); ::std::slice::from_raw_parts((*array).data as *const u8, (*array).size as usize).to_owned()};
                    (implementation.configure)(evq, idata,  self, width, height, states);
                },
                1 => {
                    (implementation.close)(evq, idata,  self);
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
    /// edge values for resizing
    ///
    /// These values are used to indicate which edge of a surface
    /// is being dragged in a resize operation.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ResizeEdge {
        None = 0,
        Top = 1,
        Bottom = 2,
        Left = 4,
        TopLeft = 5,
        BottomLeft = 6,
        Right = 8,
        TopRight = 9,
        BottomRight = 10,
    }
    impl ResizeEdge {
        pub fn from_raw(n: u32) -> Option<ResizeEdge> {
            match n {
                0 => Some(ResizeEdge::None),
                1 => Some(ResizeEdge::Top),
                2 => Some(ResizeEdge::Bottom),
                4 => Some(ResizeEdge::Left),
                5 => Some(ResizeEdge::TopLeft),
                6 => Some(ResizeEdge::BottomLeft),
                8 => Some(ResizeEdge::Right),
                9 => Some(ResizeEdge::TopRight),
                10 => Some(ResizeEdge::BottomRight),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    /// types of state on the surface
    ///
    /// The different state values used on the surface. This is designed for
    /// state values like maximized, fullscreen. It is paired with the
    /// configure event to ensure that both the client and the compositor
    /// setting the state can be synchronized.
    /// 
    /// States set in this way are double-buffered. They will get applied on
    /// the next commit.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum State {
        /// the surface is maximized
        ///
        /// The surface is maximized. The window geometry specified in the configure
        /// event must be obeyed by the client.
        Maximized = 1,
        /// the surface is fullscreen
        ///
        /// The surface is fullscreen. The window geometry specified in the configure
        /// event must be obeyed by the client.
        Fullscreen = 2,
        /// the surface is being resized
        ///
        /// The surface is being resized. The window geometry specified in the
        /// configure event is a maximum; the client cannot resize beyond it.
        /// Clients that have aspect ratio or cell sizing configuration can use
        /// a smaller size, however.
        Resizing = 3,
        /// the surface is now activated
        ///
        /// Client window decorations should be painted as if the window is
        /// active. Do not assume this means that the window actually has
        /// keyboard or pointer focus.
        Activated = 4,
    }
    impl State {
        pub fn from_raw(n: u32) -> Option<State> {
            match n {
                1 => Some(State::Maximized),
                2 => Some(State::Fullscreen),
                3 => Some(State::Resizing),
                4 => Some(State::Activated),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// suggest a surface change
        ///
        /// This configure event asks the client to resize its toplevel surface or
        /// to change its state. The configured state should not be applied
        /// immediately. See xdg_surface.configure for details.
        /// 
        /// The width and height arguments specify a hint to the window
        /// about how its surface should be resized in window geometry
        /// coordinates. See set_window_geometry.
        /// 
        /// If the width or height arguments are zero, it means the client
        /// should decide its own window dimension. This may happen when the
        /// compositor needs to configure the state of the surface but doesn't
        /// have any information about any previous or expected dimension.
        /// 
        /// The states listed in the event specify how the width/height
        /// arguments should be interpreted, and possibly how it should be
        /// drawn.
        /// 
        /// Clients must send an ack_configure in response to this event. See
        /// xdg_surface.configure and xdg_surface.ack_configure for details.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_toplevel_v6, width, height, states
        pub configure: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_toplevel_v6: &ZxdgToplevelV6, width: i32, height: i32, states: Vec<u8>),
        /// surface wants to be closed
        ///
        /// The close event is sent by the compositor when the user
        /// wants the surface to be closed. This should be equivalent to
        /// the user clicking the close button in client-side decorations,
        /// if your application has any.
        /// 
        /// This is only a request that the user intends to close the
        /// window. The client may choose to ignore this request, or show
        /// a dialog to ask the user to save their data, etc.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_toplevel_v6
        pub close: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_toplevel_v6: &ZxdgToplevelV6),
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
            && (self.configure as usize == other.configure as usize)
            && (self.close as usize == other.close as usize)

        }
    }

    const ZXDG_TOPLEVEL_V6_DESTROY: u32 = 0;
    const ZXDG_TOPLEVEL_V6_SET_PARENT: u32 = 1;
    const ZXDG_TOPLEVEL_V6_SET_TITLE: u32 = 2;
    const ZXDG_TOPLEVEL_V6_SET_APP_ID: u32 = 3;
    const ZXDG_TOPLEVEL_V6_SHOW_WINDOW_MENU: u32 = 4;
    const ZXDG_TOPLEVEL_V6_MOVE: u32 = 5;
    const ZXDG_TOPLEVEL_V6_RESIZE: u32 = 6;
    const ZXDG_TOPLEVEL_V6_SET_MAX_SIZE: u32 = 7;
    const ZXDG_TOPLEVEL_V6_SET_MIN_SIZE: u32 = 8;
    const ZXDG_TOPLEVEL_V6_SET_MAXIMIZED: u32 = 9;
    const ZXDG_TOPLEVEL_V6_UNSET_MAXIMIZED: u32 = 10;
    const ZXDG_TOPLEVEL_V6_SET_FULLSCREEN: u32 = 11;
    const ZXDG_TOPLEVEL_V6_UNSET_FULLSCREEN: u32 = 12;
    const ZXDG_TOPLEVEL_V6_SET_MINIMIZED: u32 = 13;
    impl ZxdgToplevelV6 {
        /// destroy the xdg_toplevel
        ///
        /// Unmap and destroy the window. The window will be effectively
        /// hidden from the user's point of view, and all state like
        /// maximization, fullscreen, and so on, will be lost.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// set the parent of this surface
        ///
        /// Set the "parent" of this surface. This window should be stacked
        /// above a parent. The parent surface must be mapped as long as this
        /// surface is mapped.
        /// 
        /// Parent windows should be set on dialogs, toolboxes, or other
        /// "auxiliary" surfaces, so that the parent is raised when the dialog
        /// is raised.
        pub fn set_parent(&self, parent: Option<&super::zxdg_toplevel_v6::ZxdgToplevelV6>) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SET_PARENT, parent.map(Proxy::ptr).unwrap_or(ptr::null_mut())) };
            RequestResult::Sent(())
        }
        /// set surface title
        ///
        /// Set a short title for the surface.
        /// 
        /// This string may be used to identify the surface in a task bar,
        /// window list, or other user interface elements provided by the
        /// compositor.
        /// 
        /// The string must be encoded in UTF-8.
        pub fn set_title(&self, title: String) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let title = CString::new(title).unwrap_or_else(|_| panic!("Got a String with interior null in zxdg_toplevel_v6.set_title:title"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SET_TITLE, title.as_ptr()) };
            RequestResult::Sent(())
        }
        /// set application ID
        ///
        /// Set an application identifier for the surface.
        /// 
        /// The app ID identifies the general class of applications to which
        /// the surface belongs. The compositor can use this to group multiple
        /// surfaces together, or to determine how to launch a new application.
        /// 
        /// For D-Bus activatable applications, the app ID is used as the D-Bus
        /// service name.
        /// 
        /// The compositor shell will try to group application surfaces together
        /// by their app ID. As a best practice, it is suggested to select app
        /// ID's that match the basename of the application's .desktop file.
        /// For example, "org.freedesktop.FooViewer" where the .desktop file is
        /// "org.freedesktop.FooViewer.desktop".
        /// 
        /// See the desktop-entry specification [0] for more details on
        /// application identifiers and how they relate to well-known D-Bus
        /// names and .desktop files.
        /// 
        /// [0] http://standards.freedesktop.org/desktop-entry-spec/
        pub fn set_app_id(&self, app_id: String) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let app_id = CString::new(app_id).unwrap_or_else(|_| panic!("Got a String with interior null in zxdg_toplevel_v6.set_app_id:app_id"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SET_APP_ID, app_id.as_ptr()) };
            RequestResult::Sent(())
        }
        /// show the window menu
        ///
        /// Clients implementing client-side decorations might want to show
        /// a context menu when right-clicking on the decorations, giving the
        /// user a menu that they can use to maximize or minimize the window.
        /// 
        /// This request asks the compositor to pop up such a window menu at
        /// the given position, relative to the local surface coordinates of
        /// the parent surface. There are no guarantees as to what menu items
        /// the window menu contains.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event.
        pub fn show_window_menu(&self, seat: &super::wl_seat::WlSeat, serial: u32, x: i32, y: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SHOW_WINDOW_MENU, seat.ptr(), serial, x, y) };
            RequestResult::Sent(())
        }
        /// start an interactive move
        ///
        /// Start an interactive, user-driven move of the surface.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event. The passed
        /// serial is used to determine the type of interactive move (touch,
        /// pointer, etc).
        /// 
        /// The server may ignore move requests depending on the state of
        /// the surface (e.g. fullscreen or maximized), or if the passed serial
        /// is no longer valid.
        /// 
        /// If triggered, the surface will lose the focus of the device
        /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
        /// compositor to visually indicate that the move is taking place, such as
        /// updating a pointer cursor, during the move. There is no guarantee
        /// that the device focus will return when the move is completed.
        pub fn _move(&self, seat: &super::wl_seat::WlSeat, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_MOVE, seat.ptr(), serial) };
            RequestResult::Sent(())
        }
        /// start an interactive resize
        ///
        /// Start a user-driven, interactive resize of the surface.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event. The passed
        /// serial is used to determine the type of interactive resize (touch,
        /// pointer, etc).
        /// 
        /// The server may ignore resize requests depending on the state of
        /// the surface (e.g. fullscreen or maximized).
        /// 
        /// If triggered, the client will receive configure events with the
        /// "resize" state enum value and the expected sizes. See the "resize"
        /// enum value for more details about what is required. The client
        /// must also acknowledge configure events using "ack_configure". After
        /// the resize is completed, the client will receive another "configure"
        /// event without the resize state.
        /// 
        /// If triggered, the surface also will lose the focus of the device
        /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
        /// compositor to visually indicate that the resize is taking place,
        /// such as updating a pointer cursor, during the resize. There is no
        /// guarantee that the device focus will return when the resize is
        /// completed.
        /// 
        /// The edges parameter specifies how the surface should be resized,
        /// and is one of the values of the resize_edge enum. The compositor
        /// may use this information to update the surface position for
        /// example when dragging the top left corner. The compositor may also
        /// use this information to adapt its behavior, e.g. choose an
        /// appropriate cursor image.
        pub fn resize(&self, seat: &super::wl_seat::WlSeat, serial: u32, edges: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_RESIZE, seat.ptr(), serial, edges) };
            RequestResult::Sent(())
        }
        /// set the maximum size
        ///
        /// Set a maximum size for the window.
        /// 
        /// The client can specify a maximum size so that the compositor does
        /// not try to configure the window beyond this size.
        /// 
        /// The width and height arguments are in window geometry coordinates.
        /// See xdg_surface.set_window_geometry.
        /// 
        /// Values set in this way are double-buffered. They will get applied
        /// on the next commit.
        /// 
        /// The compositor can use this information to allow or disallow
        /// different states like maximize or fullscreen and draw accurate
        /// animations.
        /// 
        /// Similarly, a tiling window manager may use this information to
        /// place and resize client windows in a more effective way.
        /// 
        /// The client should not rely on the compositor to obey the maximum
        /// size. The compositor may decide to ignore the values set by the
        /// client and request a larger size.
        /// 
        /// If never set, or a value of zero in the request, means that the
        /// client has no expected maximum size in the given dimension.
        /// As a result, a client wishing to reset the maximum size
        /// to an unspecified state can use zero for width and height in the
        /// request.
        /// 
        /// Requesting a maximum size to be smaller than the minimum size of
        /// a surface is illegal and will result in a protocol error.
        /// 
        /// The width and height must be greater than or equal to zero. Using
        /// strictly negative values for width and height will result in a
        /// protocol error.
        pub fn set_max_size(&self, width: i32, height: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SET_MAX_SIZE, width, height) };
            RequestResult::Sent(())
        }
        /// set the minimum size
        ///
        /// Set a minimum size for the window.
        /// 
        /// The client can specify a minimum size so that the compositor does
        /// not try to configure the window below this size.
        /// 
        /// The width and height arguments are in window geometry coordinates.
        /// See xdg_surface.set_window_geometry.
        /// 
        /// Values set in this way are double-buffered. They will get applied
        /// on the next commit.
        /// 
        /// The compositor can use this information to allow or disallow
        /// different states like maximize or fullscreen and draw accurate
        /// animations.
        /// 
        /// Similarly, a tiling window manager may use this information to
        /// place and resize client windows in a more effective way.
        /// 
        /// The client should not rely on the compositor to obey the minimum
        /// size. The compositor may decide to ignore the values set by the
        /// client and request a smaller size.
        /// 
        /// If never set, or a value of zero in the request, means that the
        /// client has no expected minimum size in the given dimension.
        /// As a result, a client wishing to reset the minimum size
        /// to an unspecified state can use zero for width and height in the
        /// request.
        /// 
        /// Requesting a minimum size to be larger than the maximum size of
        /// a surface is illegal and will result in a protocol error.
        /// 
        /// The width and height must be greater than or equal to zero. Using
        /// strictly negative values for width and height will result in a
        /// protocol error.
        pub fn set_min_size(&self, width: i32, height: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SET_MIN_SIZE, width, height) };
            RequestResult::Sent(())
        }
        /// maximize the window
        ///
        /// Maximize the surface.
        /// 
        /// After requesting that the surface should be maximized, the compositor
        /// will respond by emitting a configure event with the "maximized" state
        /// and the required window geometry. The client should then update its
        /// content, drawing it in a maximized state, i.e. without shadow or other
        /// decoration outside of the window geometry. The client must also
        /// acknowledge the configure when committing the new content (see
        /// ack_configure).
        /// 
        /// It is up to the compositor to decide how and where to maximize the
        /// surface, for example which output and what region of the screen should
        /// be used.
        /// 
        /// If the surface was already maximized, the compositor will still emit
        /// a configure event with the "maximized" state.
        pub fn set_maximized(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SET_MAXIMIZED) };
            RequestResult::Sent(())
        }
        /// unmaximize the window
        ///
        /// Unmaximize the surface.
        /// 
        /// After requesting that the surface should be unmaximized, the compositor
        /// will respond by emitting a configure event without the "maximized"
        /// state. If available, the compositor will include the window geometry
        /// dimensions the window had prior to being maximized in the configure
        /// request. The client must then update its content, drawing it in a
        /// regular state, i.e. potentially with shadow, etc. The client must also
        /// acknowledge the configure when committing the new content (see
        /// ack_configure).
        /// 
        /// It is up to the compositor to position the surface after it was
        /// unmaximized; usually the position the surface had before maximizing, if
        /// applicable.
        /// 
        /// If the surface was already not maximized, the compositor will still
        /// emit a configure event without the "maximized" state.
        pub fn unset_maximized(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_UNSET_MAXIMIZED) };
            RequestResult::Sent(())
        }
        /// set the window as fullscreen on a monitor
        ///
        /// Make the surface fullscreen.
        /// 
        /// You can specify an output that you would prefer to be fullscreen.
        /// If this value is NULL, it's up to the compositor to choose which
        /// display will be used to map this surface.
        /// 
        /// If the surface doesn't cover the whole output, the compositor will
        /// position the surface in the center of the output and compensate with
        /// black borders filling the rest of the output.
        pub fn set_fullscreen(&self, output: Option<&super::wl_output::WlOutput>) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SET_FULLSCREEN, output.map(Proxy::ptr).unwrap_or(ptr::null_mut())) };
            RequestResult::Sent(())
        }
        pub fn unset_fullscreen(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_UNSET_FULLSCREEN) };
            RequestResult::Sent(())
        }
        /// set the window as minimized
        ///
        /// Request that the compositor minimize your surface. There is no
        /// way to know if the surface is currently minimized, nor is there
        /// any way to unset minimization on this surface.
        /// 
        /// If you are looking to throttle redrawing when minimized, please
        /// instead use the wl_surface.frame event for this, as this will
        /// also work with live previews on windows in Alt-Tab, Expose or
        /// similar compositor features.
        pub fn set_minimized(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_TOPLEVEL_V6_SET_MINIMIZED) };
            RequestResult::Sent(())
        }
    }
}
pub mod zxdg_popup_v6 {
    //! short-lived, popup surfaces for menus
    //!
    //! A popup surface is a short-lived, temporary surface. It can be used to
    //! implement for example menus, popovers, tooltips and other similar user
    //! interface concepts.
    //! 
    //! A popup can be made to take an explicit grab. See xdg_popup.grab for
    //! details.
    //! 
    //! When the popup is dismissed, a popup_done event will be sent out, and at
    //! the same time the surface will be unmapped. See the xdg_popup.popup_done
    //! event for details.
    //! 
    //! Explicitly destroying the xdg_popup object will also dismiss the popup and
    //! unmap the surface. Clients that want to dismiss the popup when another
    //! surface of their own is clicked should dismiss the popup using the destroy
    //! request.
    //! 
    //! The parent surface must have either the xdg_toplevel or xdg_popup surface
    //! role.
    //! 
    //! A newly created xdg_popup will be stacked on top of all previously created
    //! xdg_popup surfaces associated with the same xdg_toplevel.
    //! 
    //! The parent of an xdg_popup must be mapped (see the xdg_surface
    //! description) before the xdg_popup itself.
    //! 
    //! The x and y arguments passed when creating the popup object specify
    //! where the top left of the popup should be placed, relative to the
    //! local surface coordinates of the parent surface. See
    //! xdg_surface.get_popup. An xdg_popup must intersect with or be at least
    //! partially adjacent to its parent surface.
    //! 
    //! The client must call wl_surface.commit on the corresponding wl_surface
    //! for the xdg_popup state to take effect.
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

    pub struct ZxdgPopupV6 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgPopupV6 {}
    unsafe impl Sync for ZxdgPopupV6 {}
    unsafe impl Proxy for ZxdgPopupV6 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgPopupV6 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgPopupV6 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgPopupV6 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgPopupV6 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgPopupV6 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_popup_v6_interface } }
        fn interface_name() -> &'static str { "zxdg_popup_v6"  }
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

        fn equals(&self, other: &ZxdgPopupV6) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgPopupV6 {
            ZxdgPopupV6 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZxdgPopupV6 {
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
                    let width = {*(args.offset(2) as *const i32)};
                    let height = {*(args.offset(3) as *const i32)};
                    (implementation.configure)(evq, idata,  self, x, y, width, height);
                },
                1 => {
                    (implementation.popup_done)(evq, idata,  self);
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
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// tried to grab after being mapped
        InvalidGrab = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidGrab),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// configure the popup surface
        ///
        /// This event asks the popup surface to configure itself given the
        /// configuration. The configured state should not be applied immediately.
        /// See xdg_surface.configure for details.
        /// 
        /// The x and y arguments represent the position the popup was placed at
        /// given the xdg_positioner rule, relative to the upper left corner of the
        /// window geometry of the parent surface.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_popup_v6, x, y, width, height
        pub configure: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_popup_v6: &ZxdgPopupV6, x: i32, y: i32, width: i32, height: i32),
        /// popup interaction is done
        ///
        /// The popup_done event is sent out when a popup is dismissed by the
        /// compositor. The client should destroy the xdg_popup object at this
        /// point.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_popup_v6
        pub popup_done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_popup_v6: &ZxdgPopupV6),
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
            && (self.configure as usize == other.configure as usize)
            && (self.popup_done as usize == other.popup_done as usize)

        }
    }

    const ZXDG_POPUP_V6_DESTROY: u32 = 0;
    const ZXDG_POPUP_V6_GRAB: u32 = 1;
    impl ZxdgPopupV6 {
        /// remove xdg_popup interface
        ///
        /// This destroys the popup. Explicitly destroying the xdg_popup
        /// object will also dismiss the popup, and unmap the surface.
        /// 
        /// If this xdg_popup is not the "topmost" popup, a protocol error
        /// will be sent.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POPUP_V6_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// make the popup take an explicit grab
        ///
        /// This request makes the created popup take an explicit grab. An explicit
        /// grab will be dismissed when the user dismisses the popup, or when the
        /// client destroys the xdg_popup. This can be done by the user clicking
        /// outside the surface, using the keyboard, or even locking the screen
        /// through closing the lid or a timeout.
        /// 
        /// If the compositor denies the grab, the popup will be immediately
        /// dismissed.
        /// 
        /// This request must be used in response to some sort of user action like a
        /// button press, key press, or touch down event. The serial number of the
        /// event should be passed as 'serial'.
        /// 
        /// The parent of a grabbing popup must either be an xdg_toplevel surface or
        /// another xdg_popup with an explicit grab. If the parent is another
        /// xdg_popup it means that the popups are nested, with this popup now being
        /// the topmost popup.
        /// 
        /// Nested popups must be destroyed in the reverse order they were created
        /// in, e.g. the only popup you are allowed to destroy at all times is the
        /// topmost one.
        /// 
        /// When compositors choose to dismiss a popup, they may dismiss every
        /// nested grabbing popup as well. When a compositor dismisses popups, it
        /// will follow the same dismissing order as required from the client.
        /// 
        /// The parent of a grabbing popup must either be another xdg_popup with an
        /// active explicit grab, or an xdg_popup or xdg_toplevel, if there are no
        /// explicit grabs already taken.
        /// 
        /// If the topmost grabbing popup is destroyed, the grab will be returned to
        /// the parent of the popup, if that parent previously had an explicit grab.
        /// 
        /// If the parent is a grabbing popup which has already been dismissed, this
        /// popup will be immediately dismissed. If the parent is a popup that did
        /// not take an explicit grab, an error will be raised.
        /// 
        /// During a popup grab, the client owning the grab will receive pointer
        /// and touch events for all their surfaces as normal (similar to an
        /// "owner-events" grab in X11 parlance), while the top most grabbing popup
        /// will always have keyboard focus.
        pub fn grab(&self, seat: &super::wl_seat::WlSeat, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_POPUP_V6_GRAB, seat.ptr(), serial) };
            RequestResult::Sent(())
        }
    }
}
