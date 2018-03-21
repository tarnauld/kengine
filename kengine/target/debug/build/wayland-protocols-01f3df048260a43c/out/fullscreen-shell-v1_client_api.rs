//
// This file was auto-generated, do not edit directly
//

pub mod zwp_fullscreen_shell_v1 {
    //! displays a single surface per output
    //!
    //! Displays a single surface per output.
    //! 
    //! This interface provides a mechanism for a single client to display
    //! simple full-screen surfaces.  While there technically may be multiple
    //! clients bound to this interface, only one of those clients should be
    //! shown at a time.
    //! 
    //! To present a surface, the client uses either the present_surface or
    //! present_surface_for_mode requests.  Presenting a surface takes effect
    //! on the next wl_surface.commit.  See the individual requests for
    //! details about scaling and mode switches.
    //! 
    //! The client can have at most one surface per output at any time.
    //! Requesting a surface to be presented on an output that already has a
    //! surface replaces the previously presented surface.  Presenting a null
    //! surface removes its content and effectively disables the output.
    //! Exactly what happens when an output is "disabled" is
    //! compositor-specific.  The same surface may be presented on multiple
    //! outputs simultaneously.
    //! 
    //! Once a surface is presented on an output, it stays on that output
    //! until either the client removes it or the compositor destroys the
    //! output.  This way, the client can update the output's contents by
    //! simply attaching a new buffer.
    //! 
    //! Warning! The protocol described in this file is experimental and
    //! backward incompatible changes may be made. Backward compatible changes
    //! may be added together with the corresponding interface version bump.
    //! Backward incompatible changes are done by bumping the version number in
    //! the protocol and interface names and resetting the interface version.
    //! Once the protocol is to be declared stable, the 'z' prefix and the
    //! version number in the protocol and interface names are removed and the
    //! interface version number is reset.
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

    pub struct ZwpFullscreenShellV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpFullscreenShellV1 {}
    unsafe impl Sync for ZwpFullscreenShellV1 {}
    unsafe impl Proxy for ZwpFullscreenShellV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpFullscreenShellV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpFullscreenShellV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpFullscreenShellV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpFullscreenShellV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpFullscreenShellV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_fullscreen_shell_v1_interface } }
        fn interface_name() -> &'static str { "zwp_fullscreen_shell_v1"  }
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

        fn equals(&self, other: &ZwpFullscreenShellV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpFullscreenShellV1 {
            ZwpFullscreenShellV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpFullscreenShellV1 {
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
                    let capability = {*(args.offset(0) as *const u32)};
                    (implementation.capability)(evq, idata,  self, capability);
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
    /// capabilities advertised by the compositor
    ///
    /// Various capabilities that can be advertised by the compositor.  They
    /// are advertised one-at-a-time when the wl_fullscreen_shell interface is
    /// bound.  See the wl_fullscreen_shell.capability event for more details.
    /// 
    /// ARBITRARY_MODES:
    /// This is a hint to the client that indicates that the compositor is
    /// capable of setting practically any mode on its outputs.  If this
    /// capability is provided, wl_fullscreen_shell.present_surface_for_mode
    /// will almost never fail and clients should feel free to set whatever
    /// mode they like.  If the compositor does not advertise this, it may
    /// still support some modes that are not advertised through wl_global.mode
    /// but it is less likely.
    /// 
    /// CURSOR_PLANE:
    /// This is a hint to the client that indicates that the compositor can
    /// handle a cursor surface from the client without actually compositing.
    /// This may be because of a hardware cursor plane or some other mechanism.
    /// If the compositor does not advertise this capability then setting
    /// wl_pointer.cursor may degrade performance or be ignored entirely.  If
    /// CURSOR_PLANE is not advertised, it is recommended that the client draw
    /// its own cursor and set wl_pointer.cursor(NULL).
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Capability {
        /// compositor is capable of almost any output mode
        ArbitraryModes = 1,
        /// compositor has a separate cursor plane
        CursorPlane = 2,
    }
    impl Capability {
        pub fn from_raw(n: u32) -> Option<Capability> {
            match n {
                1 => Some(Capability::ArbitraryModes),
                2 => Some(Capability::CursorPlane),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    /// different method to set the surface fullscreen
    ///
    /// Hints to indicate to the compositor how to deal with a conflict
    /// between the dimensions of the surface and the dimensions of the
    /// output. The compositor is free to ignore this parameter.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum PresentMethod {
        /// no preference, apply default policy
        Default = 0,
        /// center the surface on the output
        Center = 1,
        /// scale the surface, preserving aspect ratio, to the largest size that will fit on the output
        Zoom = 2,
        /// scale the surface, preserving aspect ratio, to fully fill the output cropping if needed
        ZoomCrop = 3,
        /// scale the surface to the size of the output ignoring aspect ratio
        Stretch = 4,
    }
    impl PresentMethod {
        pub fn from_raw(n: u32) -> Option<PresentMethod> {
            match n {
                0 => Some(PresentMethod::Default),
                1 => Some(PresentMethod::Center),
                2 => Some(PresentMethod::Zoom),
                3 => Some(PresentMethod::ZoomCrop),
                4 => Some(PresentMethod::Stretch),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    /// wl_fullscreen_shell error values
    ///
    /// These errors can be emitted in response to wl_fullscreen_shell requests.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// present_method is not known
        InvalidMethod = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidMethod),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// advertises a capability of the compositor
        ///
        /// Advertises a single capability of the compositor.
        /// 
        /// When the wl_fullscreen_shell interface is bound, this event is emitted
        /// once for each capability advertised.  Valid capabilities are given by
        /// the wl_fullscreen_shell.capability enum.  If clients want to take
        /// advantage of any of these capabilities, they should use a
        /// wl_display.sync request immediately after binding to ensure that they
        /// receive all the capability events.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_fullscreen_shell_v1, capability
        pub capability: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_fullscreen_shell_v1: &ZwpFullscreenShellV1, capability: u32),
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
            && (self.capability as usize == other.capability as usize)

        }
    }

    const ZWP_FULLSCREEN_SHELL_V1_RELEASE: u32 = 0;
    const ZWP_FULLSCREEN_SHELL_V1_PRESENT_SURFACE: u32 = 1;
    const ZWP_FULLSCREEN_SHELL_V1_PRESENT_SURFACE_FOR_MODE: u32 = 2;
    impl ZwpFullscreenShellV1 {
        /// release the wl_fullscreen_shell interface
        ///
        /// Release the binding from the wl_fullscreen_shell interface.
        /// 
        /// This destroys the server-side object and frees this binding.  If
        /// the client binds to wl_fullscreen_shell multiple times, it may wish
        /// to free some of those bindings.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn release(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_FULLSCREEN_SHELL_V1_RELEASE) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// present surface for display
        ///
        /// Present a surface on the given output.
        /// 
        /// If the output is null, the compositor will present the surface on
        /// whatever display (or displays) it thinks best.  In particular, this
        /// may replace any or all surfaces currently presented so it should
        /// not be used in combination with placing surfaces on specific
        /// outputs.
        /// 
        /// The method parameter is a hint to the compositor for how the surface
        /// is to be presented.  In particular, it tells the compositor how to
        /// handle a size mismatch between the presented surface and the
        /// output.  The compositor is free to ignore this parameter.
        /// 
        /// The "zoom", "zoom_crop", and "stretch" methods imply a scaling
        /// operation on the surface.  This will override any kind of output
        /// scaling, so the buffer_scale property of the surface is effectively
        /// ignored.
        pub fn present_surface(&self, surface: Option<&super::wl_surface::WlSurface>, method: u32, output: Option<&super::wl_output::WlOutput>) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_FULLSCREEN_SHELL_V1_PRESENT_SURFACE, surface.map(Proxy::ptr).unwrap_or(ptr::null_mut()), method, output.map(Proxy::ptr).unwrap_or(ptr::null_mut())) };
            RequestResult::Sent(())
        }
        /// present surface for display at a particular mode
        ///
        /// Presents a surface on the given output for a particular mode.
        /// 
        /// If the current size of the output differs from that of the surface,
        /// the compositor will attempt to change the size of the output to
        /// match the surface.  The result of the mode-switch operation will be
        /// returned via the provided wl_fullscreen_shell_mode_feedback object.
        /// 
        /// If the current output mode matches the one requested or if the
        /// compositor successfully switches the mode to match the surface,
        /// then the mode_successful event will be sent and the output will
        /// contain the contents of the given surface.  If the compositor
        /// cannot match the output size to the surface size, the mode_failed
        /// will be sent and the output will contain the contents of the
        /// previously presented surface (if any).  If another surface is
        /// presented on the given output before either of these has a chance
        /// to happen, the present_cancelled event will be sent.
        /// 
        /// Due to race conditions and other issues unknown to the client, no
        /// mode-switch operation is guaranteed to succeed.  However, if the
        /// mode is one advertised by wl_output.mode or if the compositor
        /// advertises the ARBITRARY_MODES capability, then the client should
        /// expect that the mode-switch operation will usually succeed.
        /// 
        /// If the size of the presented surface changes, the resulting output
        /// is undefined.  The compositor may attempt to change the output mode
        /// to compensate.  However, there is no guarantee that a suitable mode
        /// will be found and the client has no way to be notified of success
        /// or failure.
        /// 
        /// The framerate parameter specifies the desired framerate for the
        /// output in mHz.  The compositor is free to ignore this parameter.  A
        /// value of 0 indicates that the client has no preference.
        /// 
        /// If the value of wl_output.scale differs from wl_surface.buffer_scale,
        /// then the compositor may choose a mode that matches either the buffer
        /// size or the surface size.  In either case, the surface will fill the
        /// output.
        pub fn present_surface_for_mode(&self, surface: &super::wl_surface::WlSurface, output: &super::wl_output::WlOutput, framerate: i32) ->RequestResult<super::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_FULLSCREEN_SHELL_V1_PRESENT_SURFACE_FOR_MODE, &zwp_fullscreen_shell_mode_feedback_v1_interface, surface.ptr(), output.ptr(), framerate, ptr::null_mut::<wl_proxy>()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod zwp_fullscreen_shell_mode_feedback_v1 {
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

    pub struct ZwpFullscreenShellModeFeedbackV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpFullscreenShellModeFeedbackV1 {}
    unsafe impl Sync for ZwpFullscreenShellModeFeedbackV1 {}
    unsafe impl Proxy for ZwpFullscreenShellModeFeedbackV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpFullscreenShellModeFeedbackV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpFullscreenShellModeFeedbackV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpFullscreenShellModeFeedbackV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpFullscreenShellModeFeedbackV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpFullscreenShellModeFeedbackV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_fullscreen_shell_mode_feedback_v1_interface } }
        fn interface_name() -> &'static str { "zwp_fullscreen_shell_mode_feedback_v1"  }
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

        fn equals(&self, other: &ZwpFullscreenShellModeFeedbackV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpFullscreenShellModeFeedbackV1 {
            ZwpFullscreenShellModeFeedbackV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpFullscreenShellModeFeedbackV1 {
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
                    (implementation.mode_successful)(evq, idata,  self);
                },
                1 => {
                    (implementation.mode_failed)(evq, idata,  self);
                },
                2 => {
                    (implementation.present_cancelled)(evq, idata,  self);
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
        /// mode switch succeeded
        ///
        /// This event indicates that the attempted mode switch operation was
        /// successful.  A surface of the size requested in the mode switch
        /// will fill the output without scaling.
        /// 
        /// Upon receiving this event, the client should destroy the
        /// wl_fullscreen_shell_mode_feedback object.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_fullscreen_shell_mode_feedback_v1
        pub mode_successful: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_fullscreen_shell_mode_feedback_v1: &ZwpFullscreenShellModeFeedbackV1),
        /// mode switch failed
        ///
        /// This event indicates that the attempted mode switch operation
        /// failed.  This may be because the requested output mode is not
        /// possible or it may mean that the compositor does not want to allow it.
        /// 
        /// Upon receiving this event, the client should destroy the
        /// wl_fullscreen_shell_mode_feedback object.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_fullscreen_shell_mode_feedback_v1
        pub mode_failed: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_fullscreen_shell_mode_feedback_v1: &ZwpFullscreenShellModeFeedbackV1),
        /// mode switch cancelled
        ///
        /// This event indicates that the attempted mode switch operation was
        /// cancelled.  Most likely this is because the client requested a
        /// second mode switch before the first one completed.
        /// 
        /// Upon receiving this event, the client should destroy the
        /// wl_fullscreen_shell_mode_feedback object.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_fullscreen_shell_mode_feedback_v1
        pub present_cancelled: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_fullscreen_shell_mode_feedback_v1: &ZwpFullscreenShellModeFeedbackV1),
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
            && (self.mode_successful as usize == other.mode_successful as usize)
            && (self.mode_failed as usize == other.mode_failed as usize)
            && (self.present_cancelled as usize == other.present_cancelled as usize)

        }
    }

    impl ZwpFullscreenShellModeFeedbackV1 {
    }
}
