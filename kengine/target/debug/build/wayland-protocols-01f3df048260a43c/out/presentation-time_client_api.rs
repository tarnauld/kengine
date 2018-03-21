//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2013-2014 Collabora, Ltd.

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

pub mod wp_presentation {
    //! timed presentation related wl_surface requests
    //!
    //! The main feature of this interface is accurate presentation
    //! timing feedback to ensure smooth video playback while maintaining
    //! audio/video synchronization. Some features use the concept of a
    //! presentation clock, which is defined in the
    //! presentation.clock_id event.
    //! 
    //! A content update for a wl_surface is submitted by a
    //! wl_surface.commit request. Request 'feedback' associates with
    //! the wl_surface.commit and provides feedback on the content
    //! update, particularly the final realized presentation time.
    //! 
    //! 
    //! 
    //! When the final realized presentation time is available, e.g.
    //! after a framebuffer flip completes, the requested
    //! presentation_feedback.presented events are sent. The final
    //! presentation time can differ from the compositor's predicted
    //! display update time and the update's target time, especially
    //! when the compositor misses its target vertical blanking period.
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

    pub struct WpPresentation {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for WpPresentation {}
    unsafe impl Sync for WpPresentation {}
    unsafe impl Proxy for WpPresentation {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> WpPresentation {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            WpPresentation { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> WpPresentation {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                WpPresentation { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                WpPresentation { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &wp_presentation_interface } }
        fn interface_name() -> &'static str { "wp_presentation"  }
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

        fn equals(&self, other: &WpPresentation) -> bool {
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

        unsafe fn clone_unchecked(&self) -> WpPresentation {
            WpPresentation {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for WpPresentation {
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
                    let clk_id = {*(args.offset(0) as *const u32)};
                    (implementation.clock_id)(evq, idata,  self, clk_id);
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
    /// fatal presentation errors
    ///
    /// These fatal protocol errors may be emitted in response to
    /// illegal presentation requests.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// invalid value in tv_nsec
        InvalidTimestamp = 0,
        /// invalid flag
        InvalidFlag = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidTimestamp),
                1 => Some(Error::InvalidFlag),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// clock ID for timestamps
        ///
        /// This event tells the client in which clock domain the
        /// compositor interprets the timestamps used by the presentation
        /// extension. This clock is called the presentation clock.
        /// 
        /// The compositor sends this event when the client binds to the
        /// presentation interface. The presentation clock does not change
        /// during the lifetime of the client connection.
        /// 
        /// The clock identifier is platform dependent. On Linux/glibc,
        /// the identifier value is one of the clockid_t values accepted
        /// by clock_gettime(). clock_gettime() is defined by
        /// POSIX.1-2001.
        /// 
        /// Timestamps in this clock domain are expressed as tv_sec_hi,
        /// tv_sec_lo, tv_nsec triples, each component being an unsigned
        /// 32-bit value. Whole seconds are in tv_sec which is a 64-bit
        /// value combined from tv_sec_hi and tv_sec_lo, and the
        /// additional fractional part in tv_nsec as nanoseconds. Hence,
        /// for valid timestamps tv_nsec must be in [0, 999999999].
        /// 
        /// Note that clock_id applies only to the presentation clock,
        /// and implies nothing about e.g. the timestamps used in the
        /// Wayland core protocol input events.
        /// 
        /// Compositors should prefer a clock which does not jump and is
        /// not slewed e.g. by NTP. The absolute value of the clock is
        /// irrelevant. Precision of one millisecond or better is
        /// recommended. Clients must be able to query the current clock
        /// value directly, not by asking the compositor.
        ///
        /// **Arguments:** event_queue_handle, interface_data, wp_presentation, clk_id
        pub clock_id: fn(evqh: &mut EventQueueHandle, data: &mut ID,  wp_presentation: &WpPresentation, clk_id: u32),
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
            && (self.clock_id as usize == other.clock_id as usize)

        }
    }

    const WP_PRESENTATION_DESTROY: u32 = 0;
    const WP_PRESENTATION_FEEDBACK: u32 = 1;
    impl WpPresentation {
        /// unbind from the presentation interface
        ///
        /// Informs the server that the client will no longer be using
        /// this protocol object. Existing objects created by this object
        /// are not affected.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), WP_PRESENTATION_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// request presentation feedback information
        ///
        /// Request presentation feedback for the current content submission
        /// on the given surface. This creates a new presentation_feedback
        /// object, which will deliver the feedback information once. If
        /// multiple presentation_feedback objects are created for the same
        /// submission, they will all deliver the same information.
        /// 
        /// For details on what information is returned, see the
        /// presentation_feedback interface.
        pub fn feedback(&self, surface: &super::wl_surface::WlSurface) ->RequestResult<super::wp_presentation_feedback::WpPresentationFeedback> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), WP_PRESENTATION_FEEDBACK, &wp_presentation_feedback_interface, surface.ptr(), ptr::null_mut::<wl_proxy>()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod wp_presentation_feedback {
    //! presentation time feedback event
    //!
    //! A presentation_feedback object returns an indication that a
    //! wl_surface content update has become visible to the user.
    //! One object corresponds to one content update submission
    //! (wl_surface.commit). There are two possible outcomes: the
    //! content update is presented to the user, and a presentation
    //! timestamp delivered; or, the user did not see the content
    //! update because it was superseded or its surface destroyed,
    //! and the content update is discarded.
    //! 
    //! Once a presentation_feedback object has delivered a 'presented'
    //! or 'discarded' event it is automatically destroyed.
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

    pub struct WpPresentationFeedback {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for WpPresentationFeedback {}
    unsafe impl Sync for WpPresentationFeedback {}
    unsafe impl Proxy for WpPresentationFeedback {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> WpPresentationFeedback {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            WpPresentationFeedback { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> WpPresentationFeedback {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                WpPresentationFeedback { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                WpPresentationFeedback { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &wp_presentation_feedback_interface } }
        fn interface_name() -> &'static str { "wp_presentation_feedback"  }
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

        fn equals(&self, other: &WpPresentationFeedback) -> bool {
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

        unsafe fn clone_unchecked(&self) -> WpPresentationFeedback {
            WpPresentationFeedback {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for WpPresentationFeedback {
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
                    let output = {Proxy::from_ptr_initialized(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.sync_output)(evq, idata,  self, &output);
                },
                1 => {
                    let tv_sec_hi = {*(args.offset(0) as *const u32)};
                    let tv_sec_lo = {*(args.offset(1) as *const u32)};
                    let tv_nsec = {*(args.offset(2) as *const u32)};
                    let refresh = {*(args.offset(3) as *const u32)};
                    let seq_hi = {*(args.offset(4) as *const u32)};
                    let seq_lo = {*(args.offset(5) as *const u32)};
                    let flags = {*(args.offset(6) as *const u32)};
                    (implementation.presented)(evq, idata,  self, tv_sec_hi, tv_sec_lo, tv_nsec, refresh, seq_hi, seq_lo, flags);
                },
                2 => {
                    (implementation.discarded)(evq, idata,  self);
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
    /// bitmask of flags in presented event
    ///
    /// These flags provide information about how the presentation of
    /// the related content update was done. The intent is to help
    /// clients assess the reliability of the feedback and the visual
    /// quality with respect to possible tearing and timings. The
    /// flags are:
    /// 
    /// VSYNC:
    /// The presentation was synchronized to the "vertical retrace" by
    /// the display hardware such that tearing does not happen.
    /// Relying on user space scheduling is not acceptable for this
    /// flag. If presentation is done by a copy to the active
    /// frontbuffer, then it must guarantee that tearing cannot
    /// happen.
    /// 
    /// HW_CLOCK:
    /// The display hardware provided measurements that the hardware
    /// driver converted into a presentation timestamp. Sampling a
    /// clock in user space is not acceptable for this flag.
    /// 
    /// HW_COMPLETION:
    /// The display hardware signalled that it started using the new
    /// image content. The opposite of this is e.g. a timer being used
    /// to guess when the display hardware has switched to the new
    /// image content.
    /// 
    /// ZERO_COPY:
    /// The presentation of this update was done zero-copy. This means
    /// the buffer from the client was given to display hardware as
    /// is, without copying it. Compositing with OpenGL counts as
    /// copying, even if textured directly from the client buffer.
    /// Possible zero-copy cases include direct scanout of a
    /// fullscreen surface and a surface on a hardware overlay.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Kind {
        /// presentation was vsync'd
        Vsync = 0x1,
        /// hardware provided the presentation timestamp
        HwClock = 0x2,
        /// hardware signalled the start of the presentation
        HwCompletion = 0x4,
        /// presentation was done zero-copy
        ZeroCopy = 0x8,
    }
    impl Kind {
        pub fn from_raw(n: u32) -> Option<Kind> {
            match n {
                0x1 => Some(Kind::Vsync),
                0x2 => Some(Kind::HwClock),
                0x4 => Some(Kind::HwCompletion),
                0x8 => Some(Kind::ZeroCopy),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// presentation synchronized to this output
        ///
        /// As presentation can be synchronized to only one output at a
        /// time, this event tells which output it was. This event is only
        /// sent prior to the presented event.
        /// 
        /// As clients may bind to the same global wl_output multiple
        /// times, this event is sent for each bound instance that matches
        /// the synchronized output. If a client has not bound to the
        /// right wl_output global at all, this event is not sent.
        ///
        /// **Arguments:** event_queue_handle, interface_data, wp_presentation_feedback, output
        pub sync_output: fn(evqh: &mut EventQueueHandle, data: &mut ID,  wp_presentation_feedback: &WpPresentationFeedback, output: &super::wl_output::WlOutput),
        /// the content update was displayed
        ///
        /// The associated content update was displayed to the user at the
        /// indicated time (tv_sec_hi/lo, tv_nsec). For the interpretation of
        /// the timestamp, see presentation.clock_id event.
        /// 
        /// The timestamp corresponds to the time when the content update
        /// turned into light the first time on the surface's main output.
        /// Compositors may approximate this from the framebuffer flip
        /// completion events from the system, and the latency of the
        /// physical display path if known.
        /// 
        /// This event is preceded by all related sync_output events
        /// telling which output's refresh cycle the feedback corresponds
        /// to, i.e. the main output for the surface. Compositors are
        /// recommended to choose the output containing the largest part
        /// of the wl_surface, or keeping the output they previously
        /// chose. Having a stable presentation output association helps
        /// clients predict future output refreshes (vblank).
        /// 
        /// The 'refresh' argument gives the compositor's prediction of how
        /// many nanoseconds after tv_sec, tv_nsec the very next output
        /// refresh may occur. This is to further aid clients in
        /// predicting future refreshes, i.e., estimating the timestamps
        /// targeting the next few vblanks. If such prediction cannot
        /// usefully be done, the argument is zero.
        /// 
        /// If the output does not have a constant refresh rate, explicit
        /// video mode switches excluded, then the refresh argument must
        /// be zero.
        /// 
        /// The 64-bit value combined from seq_hi and seq_lo is the value
        /// of the output's vertical retrace counter when the content
        /// update was first scanned out to the display. This value must
        /// be compatible with the definition of MSC in
        /// GLX_OML_sync_control specification. Note, that if the display
        /// path has a non-zero latency, the time instant specified by
        /// this counter may differ from the timestamp's.
        /// 
        /// If the output does not have a concept of vertical retrace or a
        /// refresh cycle, or the output device is self-refreshing without
        /// a way to query the refresh count, then the arguments seq_hi
        /// and seq_lo must be zero.
        ///
        /// **Arguments:** event_queue_handle, interface_data, wp_presentation_feedback, tv_sec_hi, tv_sec_lo, tv_nsec, refresh, seq_hi, seq_lo, flags
        pub presented: fn(evqh: &mut EventQueueHandle, data: &mut ID,  wp_presentation_feedback: &WpPresentationFeedback, tv_sec_hi: u32, tv_sec_lo: u32, tv_nsec: u32, refresh: u32, seq_hi: u32, seq_lo: u32, flags: u32),
        /// the content update was not displayed
        ///
        /// The content update was never displayed to the user.
        ///
        /// **Arguments:** event_queue_handle, interface_data, wp_presentation_feedback
        pub discarded: fn(evqh: &mut EventQueueHandle, data: &mut ID,  wp_presentation_feedback: &WpPresentationFeedback),
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
            && (self.sync_output as usize == other.sync_output as usize)
            && (self.presented as usize == other.presented as usize)
            && (self.discarded as usize == other.discarded as usize)

        }
    }

    impl WpPresentationFeedback {
    }
}
