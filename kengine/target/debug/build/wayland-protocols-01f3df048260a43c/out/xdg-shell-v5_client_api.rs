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

pub mod xdg_shell {
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

    pub struct XdgShell {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for XdgShell {}
    unsafe impl Sync for XdgShell {}
    unsafe impl Proxy for XdgShell {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> XdgShell {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            XdgShell { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> XdgShell {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                XdgShell { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                XdgShell { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &xdg_shell_interface } }
        fn interface_name() -> &'static str { "xdg_shell"  }
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

        fn equals(&self, other: &XdgShell) -> bool {
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

        unsafe fn clone_unchecked(&self) -> XdgShell {
            XdgShell {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for XdgShell {
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
    /// latest protocol version
    ///
    /// The 'current' member of this enum gives the version of the
    /// protocol.  Implementations can compare this to the version
    /// they implement using static_assert to ensure the protocol and
    /// implementation versions match.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Version {
        /// Always the latest version
        Current = 5,
    }
    impl Version {
        pub fn from_raw(n: u32) -> Option<Version> {
            match n {
                5 => Some(Version::Current),
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
        /// xdg_shell was destroyed before children
        DefunctSurfaces = 1,
        /// the client tried to map or destroy a non-topmost popup
        NotTheTopmostPopup = 2,
        /// the client specified an invalid popup parent surface
        InvalidPopupParent = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                1 => Some(Error::DefunctSurfaces),
                2 => Some(Error::NotTheTopmostPopup),
                3 => Some(Error::InvalidPopupParent),
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
        /// a "pong" request back with the specified serial.
        /// 
        /// Compositors can use this to determine if the client is still
        /// alive. It's unspecified what will happen if the client doesn't
        /// respond to the ping request, or in what timeframe. Clients should
        /// try to respond in a reasonable amount of time.
        /// 
        /// A compositor is free to ping in any way it wants, but a client must
        /// always respond to any xdg_shell object it created.
        ///
        /// **Arguments:** event_queue_handle, interface_data, xdg_shell, serial
        pub ping: fn(evqh: &mut EventQueueHandle, data: &mut ID,  xdg_shell: &XdgShell, serial: u32),
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

    const XDG_SHELL_DESTROY: u32 = 0;
    const XDG_SHELL_USE_UNSTABLE_VERSION: u32 = 1;
    const XDG_SHELL_GET_XDG_SURFACE: u32 = 2;
    const XDG_SHELL_GET_XDG_POPUP: u32 = 3;
    const XDG_SHELL_PONG: u32 = 4;
    impl XdgShell {
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SHELL_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// enable use of this unstable version
        ///
        /// Negotiate the unstable version of the interface.  This
        /// mechanism is in place to ensure client and server agree on the
        /// unstable versions of the protocol that they speak or exit
        /// cleanly if they don't agree.  This request will go away once
        /// the xdg-shell protocol is stable.
        pub fn use_unstable_version(&self, version: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SHELL_USE_UNSTABLE_VERSION, version) };
            RequestResult::Sent(())
        }
        /// create a shell surface from a surface
        ///
        /// This creates an xdg_surface for the given surface and gives it the
        /// xdg_surface role. A wl_surface can only be given an xdg_surface role
        /// once. If get_xdg_surface is called with a wl_surface that already has
        /// an active xdg_surface associated with it, or if it had any other role,
        /// an error is raised.
        /// 
        /// See the documentation of xdg_surface for more details about what an
        /// xdg_surface is and how it is used.
        pub fn get_xdg_surface(&self, surface: &super::wl_surface::WlSurface) ->RequestResult<super::xdg_surface::XdgSurface> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), XDG_SHELL_GET_XDG_SURFACE, &xdg_surface_interface, ptr::null_mut::<wl_proxy>(), surface.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// create a popup for a surface
        ///
        /// This creates an xdg_popup for the given surface and gives it the
        /// xdg_popup role. A wl_surface can only be given an xdg_popup role
        /// once. If get_xdg_popup is called with a wl_surface that already has
        /// an active xdg_popup associated with it, or if it had any other role,
        /// an error is raised.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event.
        /// 
        /// See the documentation of xdg_popup for more details about what an
        /// xdg_popup is and how it is used.
        pub fn get_xdg_popup(&self, surface: &super::wl_surface::WlSurface, parent: &super::wl_surface::WlSurface, seat: &super::wl_seat::WlSeat, serial: u32, x: i32, y: i32) ->RequestResult<super::xdg_popup::XdgPopup> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), XDG_SHELL_GET_XDG_POPUP, &xdg_popup_interface, ptr::null_mut::<wl_proxy>(), surface.ptr(), parent.ptr(), seat.ptr(), serial, x, y) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// respond to a ping event
        ///
        /// A client must respond to a ping event with a pong request or
        /// the client may be deemed unresponsive.
        pub fn pong(&self, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SHELL_PONG, serial) };
            RequestResult::Sent(())
        }
    }
}
pub mod xdg_surface {
    //! A desktop window
    //!
    //! An interface that may be implemented by a wl_surface, for
    //! implementations that provide a desktop-style user interface.
    //! 
    //! It provides requests to treat surfaces like windows, allowing to set
    //! properties like maximized, fullscreen, minimized, and to move and resize
    //! them, and associate metadata like title and app id.
    //! 
    //! The client must call wl_surface.commit on the corresponding wl_surface
    //! for the xdg_surface state to take effect. Prior to committing the new
    //! state, it can set up initial configuration, such as maximizing or setting
    //! a window geometry.
    //! 
    //! Even without attaching a buffer the compositor must respond to initial
    //! committed configuration, for instance sending a configure event with
    //! expected window geometry if the client maximized its surface during
    //! initialization.
    //! 
    //! For a surface to be mapped by the compositor the client must have
    //! committed both an xdg_surface state and a buffer.
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

    pub struct XdgSurface {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for XdgSurface {}
    unsafe impl Sync for XdgSurface {}
    unsafe impl Proxy for XdgSurface {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> XdgSurface {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            XdgSurface { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> XdgSurface {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                XdgSurface { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                XdgSurface { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &xdg_surface_interface } }
        fn interface_name() -> &'static str { "xdg_surface"  }
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

        fn equals(&self, other: &XdgSurface) -> bool {
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

        unsafe fn clone_unchecked(&self) -> XdgSurface {
            XdgSurface {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for XdgSurface {
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
                    let serial = {*(args.offset(3) as *const u32)};
                    (implementation.configure)(evq, idata,  self, width, height, states, serial);
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
    /// 
    /// Desktop environments may extend this enum by taking up a range of
    /// values and documenting the range they chose in this description.
    /// They are not required to document the values for the range that they
    /// chose. Ideally, any good extensions from a desktop environment should
    /// make its way into standardization into this enum.
    /// 
    /// The current reserved ranges are:
    /// 
    /// 0x0000 - 0x0FFF: xdg-shell core values, documented below.
    /// 0x1000 - 0x1FFF: GNOME
    /// 0x2000 - 0x2FFF: EFL
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
        /// The configure event asks the client to resize its surface or to
        /// change its state.
        /// 
        /// The width and height arguments specify a hint to the window
        /// about how its surface should be resized in window geometry
        /// coordinates. See set_window_geometry.
        /// 
        /// If the width or height arguments are zero, it means the client
        /// should decide its own window dimension. This may happen when the
        /// compositor need to configure the state of the surface but doesn't
        /// have any information about any previous or expected dimension.
        /// 
        /// The states listed in the event specify how the width/height
        /// arguments should be interpreted, and possibly how it should be
        /// drawn.
        /// 
        /// Clients should arrange their surface for the new size and
        /// states, and then send a ack_configure request with the serial
        /// sent in this configure event at some point before committing
        /// the new surface.
        /// 
        /// If the client receives multiple configure events before it
        /// can respond to one, it is free to discard all but the last
        /// event it received.
        ///
        /// **Arguments:** event_queue_handle, interface_data, xdg_surface, width, height, states, serial
        pub configure: fn(evqh: &mut EventQueueHandle, data: &mut ID,  xdg_surface: &XdgSurface, width: i32, height: i32, states: Vec<u8>, serial: u32),
        /// surface wants to be closed
        ///
        /// The close event is sent by the compositor when the user
        /// wants the surface to be closed. This should be equivalent to
        /// the user clicking the close button in client-side decorations,
        /// if your application has any...
        /// 
        /// This is only a request that the user intends to close your
        /// window. The client may choose to ignore this request, or show
        /// a dialog to ask the user to save their data...
        ///
        /// **Arguments:** event_queue_handle, interface_data, xdg_surface
        pub close: fn(evqh: &mut EventQueueHandle, data: &mut ID,  xdg_surface: &XdgSurface),
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

    const XDG_SURFACE_DESTROY: u32 = 0;
    const XDG_SURFACE_SET_PARENT: u32 = 1;
    const XDG_SURFACE_SET_TITLE: u32 = 2;
    const XDG_SURFACE_SET_APP_ID: u32 = 3;
    const XDG_SURFACE_SHOW_WINDOW_MENU: u32 = 4;
    const XDG_SURFACE_MOVE: u32 = 5;
    const XDG_SURFACE_RESIZE: u32 = 6;
    const XDG_SURFACE_ACK_CONFIGURE: u32 = 7;
    const XDG_SURFACE_SET_WINDOW_GEOMETRY: u32 = 8;
    const XDG_SURFACE_SET_MAXIMIZED: u32 = 9;
    const XDG_SURFACE_UNSET_MAXIMIZED: u32 = 10;
    const XDG_SURFACE_SET_FULLSCREEN: u32 = 11;
    const XDG_SURFACE_UNSET_FULLSCREEN: u32 = 12;
    const XDG_SURFACE_SET_MINIMIZED: u32 = 13;
    impl XdgSurface {
        /// Destroy the xdg_surface
        ///
        /// Unmap and destroy the window. The window will be effectively
        /// hidden from the user's point of view, and all state like
        /// maximization, fullscreen, and so on, will be lost.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_DESTROY) };

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
        pub fn set_parent(&self, parent: Option<&super::xdg_surface::XdgSurface>) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_SET_PARENT, parent.map(Proxy::ptr).unwrap_or(ptr::null_mut())) };
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
            let title = CString::new(title).unwrap_or_else(|_| panic!("Got a String with interior null in xdg_surface.set_title:title"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_SET_TITLE, title.as_ptr()) };
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
        /// by their app ID.  As a best practice, it is suggested to select app
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
            let app_id = CString::new(app_id).unwrap_or_else(|_| panic!("Got a String with interior null in xdg_surface.set_app_id:app_id"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_SET_APP_ID, app_id.as_ptr()) };
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_SHOW_WINDOW_MENU, seat.ptr(), serial, x, y) };
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_MOVE, seat.ptr(), serial) };
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_RESIZE, seat.ptr(), serial, edges) };
            RequestResult::Sent(())
        }
        /// ack a configure event
        ///
        /// When a configure event is received, if a client commits the
        /// surface in response to the configure event, then the client
        /// must make an ack_configure request sometime before the commit
        /// request, passing along the serial of the configure event.
        /// 
        /// For instance, the compositor might use this information to move
        /// a surface to the top left only when the client has drawn itself
        /// for the maximized or fullscreen state.
        /// 
        /// If the client receives multiple configure events before it
        /// can respond to one, it only has to ack the last configure event.
        /// 
        /// A client is not required to commit immediately after sending
        /// an ack_configure request - it may even ack_configure several times
        /// before its next surface commit.
        /// 
        /// The compositor expects that the most recently received
        /// ack_configure request at the time of a commit indicates which
        /// configure event the client is responding to.
        pub fn ack_configure(&self, serial: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_ACK_CONFIGURE, serial) };
            RequestResult::Sent(())
        }
        /// set the new window geometry
        ///
        /// The window geometry of a window is its "visible bounds" from the
        /// user's perspective. Client-side decorations often have invisible
        /// portions like drop-shadows which should be ignored for the
        /// purposes of aligning, placing and constraining windows.
        /// 
        /// The window geometry is double buffered, and will be applied at the
        /// time wl_surface.commit of the corresponding wl_surface is called.
        /// 
        /// Once the window geometry of the surface is set once, it is not
        /// possible to unset it, and it will remain the same until
        /// set_window_geometry is called again, even if a new subsurface or
        /// buffer is attached.
        /// 
        /// If never set, the value is the full bounds of the surface,
        /// including any subsurfaces. This updates dynamically on every
        /// commit. This unset mode is meant for extremely simple clients.
        /// 
        /// If responding to a configure event, the window geometry in here
        /// must respect the sizing negotiations specified by the states in
        /// the configure event.
        /// 
        /// The arguments are given in the surface local coordinate space of
        /// the wl_surface associated with this xdg_surface.
        /// 
        /// The width and height must be greater than zero.
        pub fn set_window_geometry(&self, x: i32, y: i32, width: i32, height: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_SET_WINDOW_GEOMETRY, x, y, width, height) };
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_SET_MAXIMIZED) };
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_UNSET_MAXIMIZED) };
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_SET_FULLSCREEN, output.map(Proxy::ptr).unwrap_or(ptr::null_mut())) };
            RequestResult::Sent(())
        }
        pub fn unset_fullscreen(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_UNSET_FULLSCREEN) };
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_SURFACE_SET_MINIMIZED) };
            RequestResult::Sent(())
        }
    }
}
pub mod xdg_popup {
    //! short-lived, popup surfaces for menus
    //!
    //! A popup surface is a short-lived, temporary surface that can be
    //! used to implement menus. It takes an explicit grab on the surface
    //! that will be dismissed when the user dismisses the popup. This can
    //! be done by the user clicking outside the surface, using the keyboard,
    //! or even locking the screen through closing the lid or a timeout.
    //! 
    //! When the popup is dismissed, a popup_done event will be sent out,
    //! and at the same time the surface will be unmapped. The xdg_popup
    //! object is now inert and cannot be reactivated, so clients should
    //! destroy it. Explicitly destroying the xdg_popup object will also
    //! dismiss the popup and unmap the surface.
    //! 
    //! Clients will receive events for all their surfaces during this
    //! grab (which is an "owner-events" grab in X11 parlance). This is
    //! done so that users can navigate through submenus and other
    //! "nested" popup windows without having to dismiss the topmost
    //! popup.
    //! 
    //! Clients that want to dismiss the popup when another surface of
    //! their own is clicked should dismiss the popup using the destroy
    //! request.
    //! 
    //! The parent surface must have either an xdg_surface or xdg_popup
    //! role.
    //! 
    //! Specifying an xdg_popup for the parent means that the popups are
    //! nested, with this popup now being the topmost popup. Nested
    //! popups must be destroyed in the reverse order they were created
    //! in, e.g. the only popup you are allowed to destroy at all times
    //! is the topmost one.
    //! 
    //! If there is an existing popup when creating a new popup, the
    //! parent must be the current topmost popup.
    //! 
    //! A parent surface must be mapped before the new popup is mapped.
    //! 
    //! When compositors choose to dismiss a popup, they will likely
    //! dismiss every nested popup as well. When a compositor dismisses
    //! popups, it will follow the same dismissing order as required
    //! from the client.
    //! 
    //! The x and y arguments passed when creating the popup object specify
    //! where the top left of the popup should be placed, relative to the
    //! local surface coordinates of the parent surface. See
    //! xdg_shell.get_xdg_popup.
    //! 
    //! The client must call wl_surface.commit on the corresponding wl_surface
    //! for the xdg_popup state to take effect.
    //! 
    //! For a surface to be mapped by the compositor the client must have
    //! committed both the xdg_popup state and a buffer.
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

    pub struct XdgPopup {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for XdgPopup {}
    unsafe impl Sync for XdgPopup {}
    unsafe impl Proxy for XdgPopup {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> XdgPopup {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            XdgPopup { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> XdgPopup {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                XdgPopup { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                XdgPopup { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &xdg_popup_interface } }
        fn interface_name() -> &'static str { "xdg_popup"  }
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

        fn equals(&self, other: &XdgPopup) -> bool {
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

        unsafe fn clone_unchecked(&self) -> XdgPopup {
            XdgPopup {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for XdgPopup {
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
    pub struct Implementation<ID> {
        /// popup interaction is done
        ///
        /// The popup_done event is sent out when a popup is dismissed by the
        /// compositor. The client should destroy the xdg_popup object at this
        /// point.
        ///
        /// **Arguments:** event_queue_handle, interface_data, xdg_popup
        pub popup_done: fn(evqh: &mut EventQueueHandle, data: &mut ID,  xdg_popup: &XdgPopup),
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
            && (self.popup_done as usize == other.popup_done as usize)

        }
    }

    const XDG_POPUP_DESTROY: u32 = 0;
    impl XdgPopup {
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
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), XDG_POPUP_DESTROY) };

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
