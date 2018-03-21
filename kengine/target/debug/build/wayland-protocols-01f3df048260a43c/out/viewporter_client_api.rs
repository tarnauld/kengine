//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2013-2016 Collabora, Ltd.

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

pub mod wp_viewporter {
    //! surface cropping and scaling
    //!
    //! The global interface exposing surface cropping and scaling
    //! capabilities is used to instantiate an interface extension for a
    //! wl_surface object. This extended interface will then allow
    //! cropping and scaling the surface contents, effectively
    //! disconnecting the direct relationship between the buffer and the
    //! surface size.
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

    pub struct WpViewporter {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for WpViewporter {}
    unsafe impl Sync for WpViewporter {}
    unsafe impl Proxy for WpViewporter {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> WpViewporter {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            WpViewporter { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> WpViewporter {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                WpViewporter { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                WpViewporter { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &wp_viewporter_interface } }
        fn interface_name() -> &'static str { "wp_viewporter"  }
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

        fn equals(&self, other: &WpViewporter) -> bool {
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

        unsafe fn clone_unchecked(&self) -> WpViewporter {
            WpViewporter {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// the surface already has a viewport object associated
        ViewportExists = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::ViewportExists),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    const WP_VIEWPORTER_DESTROY: u32 = 0;
    const WP_VIEWPORTER_GET_VIEWPORT: u32 = 1;
    impl WpViewporter {
        /// unbind from the cropping and scaling interface
        ///
        /// Informs the server that the client will not be using this
        /// protocol object anymore. This does not affect any other objects,
        /// wp_viewport objects included.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), WP_VIEWPORTER_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// extend surface interface for crop and scale
        ///
        /// Instantiate an interface extension for the given wl_surface to
        /// crop and scale its content. If the given wl_surface already has
        /// a wp_viewport object associated, the viewport_exists
        /// protocol error is raised.
        pub fn get_viewport(&self, surface: &super::wl_surface::WlSurface) ->RequestResult<super::wp_viewport::WpViewport> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), WP_VIEWPORTER_GET_VIEWPORT, &wp_viewport_interface, ptr::null_mut::<wl_proxy>(), surface.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod wp_viewport {
    //! crop and scale interface to a wl_surface
    //!
    //! An additional interface to a wl_surface object, which allows the
    //! client to specify the cropping and scaling of the surface
    //! contents.
    //! 
    //! This interface works with two concepts: the source rectangle (src_x,
    //! src_y, src_width, src_height), and the destination size (dst_width,
    //! dst_height). The contents of the source rectangle are scaled to the
    //! destination size, and content outside the source rectangle is ignored.
    //! This state is double-buffered, and is applied on the next
    //! wl_surface.commit.
    //! 
    //! The two parts of crop and scale state are independent: the source
    //! rectangle, and the destination size. Initially both are unset, that
    //! is, no scaling is applied. The whole of the current wl_buffer is
    //! used as the source, and the surface size is as defined in
    //! wl_surface.attach.
    //! 
    //! If the destination size is set, it causes the surface size to become
    //! dst_width, dst_height. The source (rectangle) is scaled to exactly
    //! this size. This overrides whatever the attached wl_buffer size is,
    //! unless the wl_buffer is NULL. If the wl_buffer is NULL, the surface
    //! has no content and therefore no size. Otherwise, the size is always
    //! at least 1x1 in surface local coordinates.
    //! 
    //! If the source rectangle is set, it defines what area of the wl_buffer is
    //! taken as the source. If the source rectangle is set and the destination
    //! size is not set, then src_width and src_height must be integers, and the
    //! surface size becomes the source rectangle size. This results in cropping
    //! without scaling. If src_width or src_height are not integers and
    //! destination size is not set, the bad_size protocol error is raised when
    //! the surface state is applied.
    //! 
    //! The coordinate transformations from buffer pixel coordinates up to
    //! the surface-local coordinates happen in the following order:
    //! 1. buffer_transform (wl_surface.set_buffer_transform)
    //! 2. buffer_scale (wl_surface.set_buffer_scale)
    //! 3. crop and scale (wp_viewport.set*)
    //! This means, that the source rectangle coordinates of crop and scale
    //! are given in the coordinates after the buffer transform and scale,
    //! i.e. in the coordinates that would be the surface-local coordinates
    //! if the crop and scale was not applied.
    //! 
    //! If src_x or src_y are negative, the bad_value protocol error is raised.
    //! Otherwise, if the source rectangle is partially or completely outside of
    //! the non-NULL wl_buffer, then the out_of_buffer protocol error is raised
    //! when the surface state is applied. A NULL wl_buffer does not raise the
    //! out_of_buffer error.
    //! 
    //! The x, y arguments of wl_surface.attach are applied as normal to
    //! the surface. They indicate how many pixels to remove from the
    //! surface size from the left and the top. In other words, they are
    //! still in the surface-local coordinate system, just like dst_width
    //! and dst_height are.
    //! 
    //! If the wl_surface associated with the wp_viewport is destroyed,
    //! all wp_viewport requests except 'destroy' raise the protocol error
    //! no_surface.
    //! 
    //! If the wp_viewport object is destroyed, the crop and scale
    //! state is removed from the wl_surface. The change will be applied
    //! on the next wl_surface.commit.
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

    pub struct WpViewport {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for WpViewport {}
    unsafe impl Sync for WpViewport {}
    unsafe impl Proxy for WpViewport {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> WpViewport {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            WpViewport { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> WpViewport {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                WpViewport { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                WpViewport { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &wp_viewport_interface } }
        fn interface_name() -> &'static str { "wp_viewport"  }
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

        fn equals(&self, other: &WpViewport) -> bool {
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

        unsafe fn clone_unchecked(&self) -> WpViewport {
            WpViewport {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// negative or zero values in width or height
        BadValue = 0,
        /// destination size is not integer
        BadSize = 1,
        /// source rectangle extends outside of the content area
        OutOfBuffer = 2,
        /// the wl_surface was destroyed
        NoSurface = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::BadValue),
                1 => Some(Error::BadSize),
                2 => Some(Error::OutOfBuffer),
                3 => Some(Error::NoSurface),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    const WP_VIEWPORT_DESTROY: u32 = 0;
    const WP_VIEWPORT_SET_SOURCE: u32 = 1;
    const WP_VIEWPORT_SET_DESTINATION: u32 = 2;
    impl WpViewport {
        /// remove scaling and cropping from the surface
        ///
        /// The associated wl_surface's crop and scale state is removed.
        /// The change is applied on the next wl_surface.commit.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), WP_VIEWPORT_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// set the source rectangle for cropping
        ///
        /// Set the source rectangle of the associated wl_surface. See
        /// wp_viewport for the description, and relation to the wl_buffer
        /// size.
        /// 
        /// If all of x, y, width and height are -1.0, the source rectangle is
        /// unset instead. Any other set of values where width or height are zero
        /// or negative, or x or y are negative, raise the bad_value protocol
        /// error.
        /// 
        /// The crop and scale state is double-buffered state, and will be
        /// applied on the next wl_surface.commit.
        pub fn set_source(&self, x: f64, y: f64, width: f64, height: f64) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let x = wl_fixed_from_double(x);
            let y = wl_fixed_from_double(y);
            let width = wl_fixed_from_double(width);
            let height = wl_fixed_from_double(height);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), WP_VIEWPORT_SET_SOURCE, x, y, width, height) };
            RequestResult::Sent(())
        }
        /// set the surface size for scaling
        ///
        /// Set the destination size of the associated wl_surface. See
        /// wp_viewport for the description, and relation to the wl_buffer
        /// size.
        /// 
        /// If width is -1 and height is -1, the destination size is unset
        /// instead. Any other pair of values for width and height that
        /// contains zero or negative values raises the bad_value protocol
        /// error.
        /// 
        /// The crop and scale state is double-buffered state, and will be
        /// applied on the next wl_surface.commit.
        pub fn set_destination(&self, width: i32, height: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), WP_VIEWPORT_SET_DESTINATION, width, height) };
            RequestResult::Sent(())
        }
    }
}
