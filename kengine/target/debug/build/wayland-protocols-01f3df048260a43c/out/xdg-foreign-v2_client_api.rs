//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2015-2016 Red Hat Inc.

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

pub mod zxdg_exporter_v2 {
    //! interface for exporting surfaces
    //!
    //! A global interface used for exporting surfaces that can later be imported
    //! using xdg_importer.
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

    pub struct ZxdgExporterV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgExporterV2 {}
    unsafe impl Sync for ZxdgExporterV2 {}
    unsafe impl Proxy for ZxdgExporterV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgExporterV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgExporterV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgExporterV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgExporterV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgExporterV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_exporter_v2_interface } }
        fn interface_name() -> &'static str { "zxdg_exporter_v2"  }
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

        fn equals(&self, other: &ZxdgExporterV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgExporterV2 {
            ZxdgExporterV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZXDG_EXPORTER_V2_DESTROY: u32 = 0;
    const ZXDG_EXPORTER_V2_EXPORT_TOPLEVEL: u32 = 1;
    impl ZxdgExporterV2 {
        /// destroy the xdg_exporter object
        ///
        /// Notify the compositor that the xdg_exporter object will no longer be
        /// used.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_EXPORTER_V2_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// export a toplevel surface
        ///
        /// The export_toplevel request exports the passed surface so that it can later be
        /// imported via xdg_importer. When called, a new xdg_exported object will
        /// be created and xdg_exported.handle will be sent immediately. See the
        /// corresponding interface and event for details.
        /// 
        /// A surface may be exported multiple times, and each exported handle may
        /// be used to create a xdg_imported multiple times. Only xdg_toplevel
        /// equivalent surfaces may be exported.
        pub fn export_toplevel(&self, surface: &super::wl_surface::WlSurface) ->RequestResult<super::zxdg_exported_v2::ZxdgExportedV2> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZXDG_EXPORTER_V2_EXPORT_TOPLEVEL, &zxdg_exported_v2_interface, ptr::null_mut::<wl_proxy>(), surface.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod zxdg_importer_v2 {
    //! interface for importing surfaces
    //!
    //! A global interface used for importing surfaces exported by xdg_exporter.
    //! With this interface, a client can create a reference to a surface of
    //! another client.
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

    pub struct ZxdgImporterV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgImporterV2 {}
    unsafe impl Sync for ZxdgImporterV2 {}
    unsafe impl Proxy for ZxdgImporterV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgImporterV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgImporterV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgImporterV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgImporterV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgImporterV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_importer_v2_interface } }
        fn interface_name() -> &'static str { "zxdg_importer_v2"  }
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

        fn equals(&self, other: &ZxdgImporterV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgImporterV2 {
            ZxdgImporterV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZXDG_IMPORTER_V2_DESTROY: u32 = 0;
    const ZXDG_IMPORTER_V2_IMPORT_TOPLEVEL: u32 = 1;
    impl ZxdgImporterV2 {
        /// destroy the xdg_importer object
        ///
        /// Notify the compositor that the xdg_importer object will no longer be
        /// used.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_IMPORTER_V2_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// import a toplevel surface
        ///
        /// The import_toplevel request imports a surface from any client given a handle
        /// retrieved by exporting said surface using xdg_exporter.export_toplevel.
        /// When called, a new xdg_imported object will be created. This new object
        /// represents the imported surface, and the importing client can
        /// manipulate its relationship using it. See xdg_imported for details.
        pub fn import_toplevel(&self, handle: String) ->RequestResult<super::zxdg_imported_v2::ZxdgImportedV2> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let handle = CString::new(handle).unwrap_or_else(|_| panic!("Got a String with interior null in zxdg_importer_v2.import_toplevel:handle"));
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZXDG_IMPORTER_V2_IMPORT_TOPLEVEL, &zxdg_imported_v2_interface, ptr::null_mut::<wl_proxy>(), handle.as_ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
    }
}
pub mod zxdg_exported_v2 {
    //! an exported surface handle
    //!
    //! A xdg_exported object represents an exported reference to a surface. The
    //! exported surface may be referenced as long as the xdg_exported object not
    //! destroyed. Destroying the xdg_exported invalidates any relationship the
    //! importer may have established using xdg_imported.
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

    pub struct ZxdgExportedV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgExportedV2 {}
    unsafe impl Sync for ZxdgExportedV2 {}
    unsafe impl Proxy for ZxdgExportedV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgExportedV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgExportedV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgExportedV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgExportedV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgExportedV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_exported_v2_interface } }
        fn interface_name() -> &'static str { "zxdg_exported_v2"  }
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

        fn equals(&self, other: &ZxdgExportedV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgExportedV2 {
            ZxdgExportedV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZxdgExportedV2 {
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
                    let handle = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(0) as *const *const _)).to_bytes()).into_owned()};
                    (implementation.handle)(evq, idata,  self, handle);
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
        /// the exported surface handle
        ///
        /// The handle event contains the unique handle of this exported surface
        /// reference. It may be shared with any client, which then can use it to
        /// import the surface by calling xdg_importer.import_toplevel. A handle
        /// may be used to import the surface multiple times.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_exported_v2, handle
        pub handle: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_exported_v2: &ZxdgExportedV2, handle: String),
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
            && (self.handle as usize == other.handle as usize)

        }
    }

    const ZXDG_EXPORTED_V2_DESTROY: u32 = 0;
    impl ZxdgExportedV2 {
        /// unexport the exported surface
        ///
        /// Revoke the previously exported surface. This invalidates any
        /// relationship the importer may have set up using the xdg_imported created
        /// given the handle sent via xdg_exported.handle.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_EXPORTED_V2_DESTROY) };

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
pub mod zxdg_imported_v2 {
    //! an imported surface handle
    //!
    //! A xdg_imported object represents an imported reference to surface exported
    //! by some client. A client can use this interface to manipulate
    //! relationships between its own surfaces and the imported surface.
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

    pub struct ZxdgImportedV2 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZxdgImportedV2 {}
    unsafe impl Sync for ZxdgImportedV2 {}
    unsafe impl Proxy for ZxdgImportedV2 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZxdgImportedV2 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZxdgImportedV2 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZxdgImportedV2 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZxdgImportedV2 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZxdgImportedV2 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zxdg_imported_v2_interface } }
        fn interface_name() -> &'static str { "zxdg_imported_v2"  }
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

        fn equals(&self, other: &ZxdgImportedV2) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZxdgImportedV2 {
            ZxdgImportedV2 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZxdgImportedV2 {
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
                    (implementation.destroyed)(evq, idata,  self);
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
        /// the imported surface handle has been destroyed
        ///
        /// The imported surface handle has been destroyed and any relationship set
        /// up has been invalidated. This may happen for various reasons, for
        /// example if the exported surface or the exported surface handle has been
        /// destroyed, if the handle used for importing was invalid.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zxdg_imported_v2
        pub destroyed: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zxdg_imported_v2: &ZxdgImportedV2),
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
            && (self.destroyed as usize == other.destroyed as usize)

        }
    }

    const ZXDG_IMPORTED_V2_DESTROY: u32 = 0;
    const ZXDG_IMPORTED_V2_SET_PARENT_OF: u32 = 1;
    impl ZxdgImportedV2 {
        /// destroy the xdg_imported object
        ///
        /// Notify the compositor that it will no longer use the xdg_imported
        /// object. Any relationship that may have been set up will at this point
        /// be invalidated.
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_IMPORTED_V2_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// set as the parent of some surface
        ///
        /// Set the imported surface as the parent of some surface of the client.
        /// The passed surface must be a xdg_toplevel equivalent. Calling this
        /// function sets up a surface to surface relation with the same stacking
        /// and positioning semantics as xdg_toplevel.set_parent.
        pub fn set_parent_of(&self, surface: &super::wl_surface::WlSurface) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZXDG_IMPORTED_V2_SET_PARENT_OF, surface.ptr()) };
            RequestResult::Sent(())
        }
    }
}
