//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2014, 2015 Collabora, Ltd.

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

pub mod zwp_linux_dmabuf_v1 {
    //! factory for creating dmabuf-based wl_buffers
    //!
    //! Following the interfaces from:
    //! https://www.khronos.org/registry/egl/extensions/EXT/EGL_EXT_image_dma_buf_import.txt
    //! and the Linux DRM sub-system's AddFb2 ioctl.
    //! 
    //! This interface offers ways to create generic dmabuf-based
    //! wl_buffers. Immediately after a client binds to this interface,
    //! the set of supported formats and format modifiers is sent with
    //! 'format' and 'modifier' events.
    //! 
    //! The following are required from clients:
    //! 
    //! - Clients must ensure that either all data in the dma-buf is
    //! coherent for all subsequent read access or that coherency is
    //! correctly handled by the underlying kernel-side dma-buf
    //! implementation.
    //! 
    //! - Don't make any more attachments after sending the buffer to the
    //! compositor. Making more attachments later increases the risk of
    //! the compositor not being able to use (re-import) an existing
    //! dmabuf-based wl_buffer.
    //! 
    //! The underlying graphics stack must ensure the following:
    //! 
    //! - The dmabuf file descriptors relayed to the server will stay valid
    //! for the whole lifetime of the wl_buffer. This means the server may
    //! at any time use those fds to import the dmabuf into any kernel
    //! sub-system that might accept it.
    //! 
    //! To create a wl_buffer from one or more dmabufs, a client creates a
    //! zwp_linux_dmabuf_params_v1 object with a zwp_linux_dmabuf_v1.create_params
    //! request. All planes required by the intended format are added with
    //! the 'add' request. Finally, a 'create' or 'create_immed' request is
    //! issued, which has the following outcome depending on the import success.
    //! 
    //! The 'create' request,
    //! - on success, triggers a 'created' event which provides the final
    //! wl_buffer to the client.
    //! - on failure, triggers a 'failed' event to convey that the server
    //! cannot use the dmabufs received from the client.
    //! 
    //! For the 'create_immed' request,
    //! - on success, the server immediately imports the added dmabufs to
    //! create a wl_buffer. No event is sent from the server in this case.
    //! - on failure, the server can choose to either:
    //! - terminate the client by raising a fatal error.
    //! - mark the wl_buffer as failed, and send a 'failed' event to the
    //! client. If the client uses a failed wl_buffer as an argument to any
    //! request, the behaviour is compositor implementation-defined.
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

    pub struct ZwpLinuxDmabufV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpLinuxDmabufV1 {}
    unsafe impl Sync for ZwpLinuxDmabufV1 {}
    unsafe impl Proxy for ZwpLinuxDmabufV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpLinuxDmabufV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpLinuxDmabufV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpLinuxDmabufV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpLinuxDmabufV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpLinuxDmabufV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_linux_dmabuf_v1_interface } }
        fn interface_name() -> &'static str { "zwp_linux_dmabuf_v1"  }
        fn supported_version() -> u32 { 3 }
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

        fn equals(&self, other: &ZwpLinuxDmabufV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpLinuxDmabufV1 {
            ZwpLinuxDmabufV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpLinuxDmabufV1 {
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
                    let format = {*(args.offset(0) as *const u32)};
                    (implementation.format)(evq, idata,  self, format);
                },
                1 => {
                    let format = {*(args.offset(0) as *const u32)};
                    let modifier_hi = {*(args.offset(1) as *const u32)};
                    let modifier_lo = {*(args.offset(2) as *const u32)};
                    (implementation.modifier)(evq, idata,  self, format, modifier_hi, modifier_lo);
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
        /// supported buffer format
        ///
        /// This event advertises one buffer format that the server supports.
        /// All the supported formats are advertised once when the client
        /// binds to this interface. A roundtrip after binding guarantees
        /// that the client has received all supported formats.
        /// 
        /// For the definition of the format codes, see the
        /// zwp_linux_buffer_params_v1::create request.
        /// 
        /// Warning: the 'format' event is likely to be deprecated and replaced
        /// with the 'modifier' event introduced in zwp_linux_dmabuf_v1
        /// version 3, described below. Please refrain from using the information
        /// received from this event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_linux_dmabuf_v1, format
        pub format: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_linux_dmabuf_v1: &ZwpLinuxDmabufV1, format: u32),
        /// supported buffer format modifier
        ///
        /// This event advertises the formats that the server supports, along with
        /// the modifiers supported for each format. All the supported modifiers
        /// for all the supported formats are advertised once when the client
        /// binds to this interface. A roundtrip after binding guarantees that
        /// the client has received all supported format-modifier pairs.
        /// 
        /// For the definition of the format and modifier codes, see the
        /// zwp_linux_buffer_params_v1::create request.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_linux_dmabuf_v1, format, modifier_hi, modifier_lo
        ///
        /// This request only exists since version 3 of the interface
        pub modifier: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_linux_dmabuf_v1: &ZwpLinuxDmabufV1, format: u32, modifier_hi: u32, modifier_lo: u32),
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
            && (self.format as usize == other.format as usize)
            && (self.modifier as usize == other.modifier as usize)

        }
    }

    const ZWP_LINUX_DMABUF_V1_DESTROY: u32 = 0;
    const ZWP_LINUX_DMABUF_V1_CREATE_PARAMS: u32 = 1;
    impl ZwpLinuxDmabufV1 {
        /// unbind the factory
        ///
        /// Objects created through this interface, especially wl_buffers, will
        /// remain valid.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_LINUX_DMABUF_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// create a temporary object for buffer parameters
        ///
        /// This temporary object is used to collect multiple dmabuf handles into
        /// a single batch to create a wl_buffer. It can only be used once and
        /// should be destroyed after a 'created' or 'failed' event has been
        /// received.
        pub fn create_params(&self) ->RequestResult<super::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_LINUX_DMABUF_V1_CREATE_PARAMS, &zwp_linux_buffer_params_v1_interface, ptr::null_mut::<wl_proxy>()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod zwp_linux_buffer_params_v1 {
    //! parameters for creating a dmabuf-based wl_buffer
    //!
    //! This temporary object is a collection of dmabufs and other
    //! parameters that together form a single logical buffer. The temporary
    //! object may eventually create one wl_buffer unless cancelled by
    //! destroying it before requesting 'create'.
    //! 
    //! Single-planar formats only require one dmabuf, however
    //! multi-planar formats may require more than one dmabuf. For all
    //! formats, an 'add' request must be called once per plane (even if the
    //! underlying dmabuf fd is identical).
    //! 
    //! You must use consecutive plane indices ('plane_idx' argument for 'add')
    //! from zero to the number of planes used by the drm_fourcc format code.
    //! All planes required by the format must be given exactly once, but can
    //! be given in any order. Each plane index can be set only once.
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

    pub struct ZwpLinuxBufferParamsV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpLinuxBufferParamsV1 {}
    unsafe impl Sync for ZwpLinuxBufferParamsV1 {}
    unsafe impl Proxy for ZwpLinuxBufferParamsV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpLinuxBufferParamsV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpLinuxBufferParamsV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpLinuxBufferParamsV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpLinuxBufferParamsV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpLinuxBufferParamsV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_linux_buffer_params_v1_interface } }
        fn interface_name() -> &'static str { "zwp_linux_buffer_params_v1"  }
        fn supported_version() -> u32 { 3 }
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

        fn equals(&self, other: &ZwpLinuxBufferParamsV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpLinuxBufferParamsV1 {
            ZwpLinuxBufferParamsV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpLinuxBufferParamsV1 {
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
                    let buffer = {Proxy::from_ptr_new(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.created)(evq, idata,  self, buffer);
                },
                1 => {
                    (implementation.failed)(evq, idata,  self);
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
        /// the dmabuf_batch object has already been used to create a wl_buffer
        AlreadyUsed = 0,
        /// plane index out of bounds
        PlaneIdx = 1,
        /// the plane index was already set
        PlaneSet = 2,
        /// missing or too many planes to create a buffer
        Incomplete = 3,
        /// format not supported
        InvalidFormat = 4,
        /// invalid width or height
        InvalidDimensions = 5,
        /// offset + stride * height goes out of dmabuf bounds
        OutOfBounds = 6,
        /// invalid wl_buffer resulted from importing dmabufs via the create_immed request on given buffer_params
        InvalidWlBuffer = 7,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyUsed),
                1 => Some(Error::PlaneIdx),
                2 => Some(Error::PlaneSet),
                3 => Some(Error::Incomplete),
                4 => Some(Error::InvalidFormat),
                5 => Some(Error::InvalidDimensions),
                6 => Some(Error::OutOfBounds),
                7 => Some(Error::InvalidWlBuffer),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Flags {
        /// contents are y-inverted
        YInvert = 1,
        /// content is interlaced
        Interlaced = 2,
        /// bottom field first
        BottomFirst = 4,
    }
    impl Flags {
        pub fn from_raw(n: u32) -> Option<Flags> {
            match n {
                1 => Some(Flags::YInvert),
                2 => Some(Flags::Interlaced),
                4 => Some(Flags::BottomFirst),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// buffer creation succeeded
        ///
        /// This event indicates that the attempted buffer creation was
        /// successful. It provides the new wl_buffer referencing the dmabuf(s).
        /// 
        /// Upon receiving this event, the client should destroy the
        /// zlinux_dmabuf_params object.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_linux_buffer_params_v1, buffer
        pub created: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_linux_buffer_params_v1: &ZwpLinuxBufferParamsV1, buffer: super::wl_buffer::WlBuffer),
        /// buffer creation failed
        ///
        /// This event indicates that the attempted buffer creation has
        /// failed. It usually means that one of the dmabuf constraints
        /// has not been fulfilled.
        /// 
        /// Upon receiving this event, the client should destroy the
        /// zlinux_buffer_params object.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_linux_buffer_params_v1
        pub failed: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_linux_buffer_params_v1: &ZwpLinuxBufferParamsV1),
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
            && (self.created as usize == other.created as usize)
            && (self.failed as usize == other.failed as usize)

        }
    }

    const ZWP_LINUX_BUFFER_PARAMS_V1_DESTROY: u32 = 0;
    const ZWP_LINUX_BUFFER_PARAMS_V1_ADD: u32 = 1;
    const ZWP_LINUX_BUFFER_PARAMS_V1_CREATE: u32 = 2;
    const ZWP_LINUX_BUFFER_PARAMS_V1_CREATE_IMMED: u32 = 3;
    impl ZwpLinuxBufferParamsV1 {
        /// delete this object, used or not
        ///
        /// Cleans up the temporary data sent to the server for dmabuf-based
        /// wl_buffer creation.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_LINUX_BUFFER_PARAMS_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// add a dmabuf to the temporary set
        ///
        /// This request adds one dmabuf to the set in this
        /// zwp_linux_buffer_params_v1.
        /// 
        /// The 64-bit unsigned value combined from modifier_hi and modifier_lo
        /// is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
        /// fb modifier, which is defined in drm_mode.h of Linux UAPI.
        /// This is an opaque token. Drivers use this token to express tiling,
        /// compression, etc. driver-specific modifications to the base format
        /// defined by the DRM fourcc code.
        /// 
        /// This request raises the PLANE_IDX error if plane_idx is too large.
        /// The error PLANE_SET is raised if attempting to set a plane that
        /// was already set.
        pub fn add(&self, fd: ::std::os::unix::io::RawFd, plane_idx: u32, offset: u32, stride: u32, modifier_hi: u32, modifier_lo: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_LINUX_BUFFER_PARAMS_V1_ADD, fd, plane_idx, offset, stride, modifier_hi, modifier_lo) };
            RequestResult::Sent(())
        }
        /// create a wl_buffer from the given dmabufs
        ///
        /// This asks for creation of a wl_buffer from the added dmabuf
        /// buffers. The wl_buffer is not created immediately but returned via
        /// the 'created' event if the dmabuf sharing succeeds. The sharing
        /// may fail at runtime for reasons a client cannot predict, in
        /// which case the 'failed' event is triggered.
        /// 
        /// The 'format' argument is a DRM_FORMAT code, as defined by the
        /// libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
        /// authoritative source on how the format codes should work.
        /// 
        /// The 'flags' is a bitfield of the flags defined in enum "flags".
        /// 'y_invert' means the that the image needs to be y-flipped.
        /// 
        /// Flag 'interlaced' means that the frame in the buffer is not
        /// progressive as usual, but interlaced. An interlaced buffer as
        /// supported here must always contain both top and bottom fields.
        /// The top field always begins on the first pixel row. The temporal
        /// ordering between the two fields is top field first, unless
        /// 'bottom_first' is specified. It is undefined whether 'bottom_first'
        /// is ignored if 'interlaced' is not set.
        /// 
        /// This protocol does not convey any information about field rate,
        /// duration, or timing, other than the relative ordering between the
        /// two fields in one buffer. A compositor may have to estimate the
        /// intended field rate from the incoming buffer rate. It is undefined
        /// whether the time of receiving wl_surface.commit with a new buffer
        /// attached, applying the wl_surface state, wl_surface.frame callback
        /// trigger, presentation, or any other point in the compositor cycle
        /// is used to measure the frame or field times. There is no support
        /// for detecting missed or late frames/fields/buffers either, and
        /// there is no support whatsoever for cooperating with interlaced
        /// compositor output.
        /// 
        /// The composited image quality resulting from the use of interlaced
        /// buffers is explicitly undefined. A compositor may use elaborate
        /// hardware features or software to deinterlace and create progressive
        /// output frames from a sequence of interlaced input buffers, or it
        /// may produce substandard image quality. However, compositors that
        /// cannot guarantee reasonable image quality in all cases are recommended
        /// to just reject all interlaced buffers.
        /// 
        /// Any argument errors, including non-positive width or height,
        /// mismatch between the number of planes and the format, bad
        /// format, bad offset or stride, may be indicated by fatal protocol
        /// errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
        /// OUT_OF_BOUNDS.
        /// 
        /// Dmabuf import errors in the server that are not obvious client
        /// bugs are returned via the 'failed' event as non-fatal. This
        /// allows attempting dmabuf sharing and falling back in the client
        /// if it fails.
        /// 
        /// This request can be sent only once in the object's lifetime, after
        /// which the only legal request is destroy. This object should be
        /// destroyed after issuing a 'create' request. Attempting to use this
        /// object after issuing 'create' raises ALREADY_USED protocol error.
        /// 
        /// It is not mandatory to issue 'create'. If a client wants to
        /// cancel the buffer creation, it can just destroy this object.
        pub fn create(&self, width: i32, height: i32, format: u32, flags: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_LINUX_BUFFER_PARAMS_V1_CREATE, width, height, format, flags) };
            RequestResult::Sent(())
        }
        /// immediately create a wl_buffer from the given dmabufs
        ///
        /// This asks for immediate creation of a wl_buffer by importing the
        /// added dmabufs.
        /// 
        /// In case of import success, no event is sent from the server, and the
        /// wl_buffer is ready to be used by the client.
        /// 
        /// Upon import failure, either of the following may happen, as seen fit
        /// by the implementation:
        /// - the client is terminated with one of the following fatal protocol
        /// errors:
        /// - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
        /// in case of argument errors such as mismatch between the number
        /// of planes and the format, bad format, non-positive width or
        /// height, or bad offset or stride.
        /// - INVALID_WL_BUFFER, in case the cause for failure is unknown or
        /// plaform specific.
        /// - the server creates an invalid wl_buffer, marks it as failed and
        /// sends a 'failed' event to the client. The result of using this
        /// invalid wl_buffer as an argument in any request by the client is
        /// defined by the compositor implementation.
        /// 
        /// This takes the same arguments as a 'create' request, and obeys the
        /// same restrictions.
        ///
        /// This request is only available since version 2 of the interface
        pub fn create_immed(&self, width: i32, height: i32, format: u32, flags: u32) ->RequestResult<super::wl_buffer::WlBuffer> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_LINUX_BUFFER_PARAMS_V1_CREATE_IMMED, &wl_buffer_interface, ptr::null_mut::<wl_proxy>(), width, height, format, flags) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
