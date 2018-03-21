//
// This file was auto-generated, do not edit directly
//

/*
Copyright © 2012, 2013 Intel Corporation

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

pub mod zwp_input_method_context_v1 {
    //! input method context
    //!
    //! Corresponds to a text input on the input method side. An input method context
    //! is created on text input activation on the input method side. It allows
    //! receiving information about the text input from the application via events.
    //! Input method contexts do not keep state after deactivation and should be
    //! destroyed after deactivation is handled.
    //! 
    //! Text is generally UTF-8 encoded, indices and lengths are in bytes.
    //! 
    //! Serials are used to synchronize the state between the text input and
    //! an input method. New serials are sent by the text input in the
    //! commit_state request and are used by the input method to indicate
    //! the known text input state in events like preedit_string, commit_string,
    //! and keysym. The text input can then ignore events from the input method
    //! which are based on an outdated state (for example after a reset).
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

    pub struct ZwpInputMethodContextV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpInputMethodContextV1 {}
    unsafe impl Sync for ZwpInputMethodContextV1 {}
    unsafe impl Proxy for ZwpInputMethodContextV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpInputMethodContextV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpInputMethodContextV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpInputMethodContextV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpInputMethodContextV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpInputMethodContextV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_input_method_context_v1_interface } }
        fn interface_name() -> &'static str { "zwp_input_method_context_v1"  }
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

        fn equals(&self, other: &ZwpInputMethodContextV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpInputMethodContextV1 {
            ZwpInputMethodContextV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpInputMethodContextV1 {
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
                    let text = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(0) as *const *const _)).to_bytes()).into_owned()};
                    let cursor = {*(args.offset(1) as *const u32)};
                    let anchor = {*(args.offset(2) as *const u32)};
                    (implementation.surrounding_text)(evq, idata,  self, text, cursor, anchor);
                },
                1 => {
                    (implementation.reset)(evq, idata,  self);
                },
                2 => {
                    let hint = {*(args.offset(0) as *const u32)};
                    let purpose = {*(args.offset(1) as *const u32)};
                    (implementation.content_type)(evq, idata,  self, hint, purpose);
                },
                3 => {
                    let button = {*(args.offset(0) as *const u32)};
                    let index = {*(args.offset(1) as *const u32)};
                    (implementation.invoke_action)(evq, idata,  self, button, index);
                },
                4 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    (implementation.commit_state)(evq, idata,  self, serial);
                },
                5 => {
                    let language = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(0) as *const *const _)).to_bytes()).into_owned()};
                    (implementation.preferred_language)(evq, idata,  self, language);
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
        /// surrounding text event
        ///
        /// The plain surrounding text around the input position. Cursor is the
        /// position in bytes within the surrounding text relative to the beginning
        /// of the text. Anchor is the position in bytes of the selection anchor
        /// within the surrounding text relative to the beginning of the text. If
        /// there is no selected text then anchor is the same as cursor.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_input_method_context_v1, text, cursor, anchor
        pub surrounding_text: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_input_method_context_v1: &ZwpInputMethodContextV1, text: String, cursor: u32, anchor: u32),
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_input_method_context_v1
        pub reset: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_input_method_context_v1: &ZwpInputMethodContextV1),
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_input_method_context_v1, hint, purpose
        pub content_type: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_input_method_context_v1: &ZwpInputMethodContextV1, hint: u32, purpose: u32),
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_input_method_context_v1, button, index
        pub invoke_action: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_input_method_context_v1: &ZwpInputMethodContextV1, button: u32, index: u32),
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_input_method_context_v1, serial
        pub commit_state: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_input_method_context_v1: &ZwpInputMethodContextV1, serial: u32),
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_input_method_context_v1, language
        pub preferred_language: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_input_method_context_v1: &ZwpInputMethodContextV1, language: String),
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
            && (self.surrounding_text as usize == other.surrounding_text as usize)
            && (self.reset as usize == other.reset as usize)
            && (self.content_type as usize == other.content_type as usize)
            && (self.invoke_action as usize == other.invoke_action as usize)
            && (self.commit_state as usize == other.commit_state as usize)
            && (self.preferred_language as usize == other.preferred_language as usize)

        }
    }

    const ZWP_INPUT_METHOD_CONTEXT_V1_DESTROY: u32 = 0;
    const ZWP_INPUT_METHOD_CONTEXT_V1_COMMIT_STRING: u32 = 1;
    const ZWP_INPUT_METHOD_CONTEXT_V1_PREEDIT_STRING: u32 = 2;
    const ZWP_INPUT_METHOD_CONTEXT_V1_PREEDIT_STYLING: u32 = 3;
    const ZWP_INPUT_METHOD_CONTEXT_V1_PREEDIT_CURSOR: u32 = 4;
    const ZWP_INPUT_METHOD_CONTEXT_V1_DELETE_SURROUNDING_TEXT: u32 = 5;
    const ZWP_INPUT_METHOD_CONTEXT_V1_CURSOR_POSITION: u32 = 6;
    const ZWP_INPUT_METHOD_CONTEXT_V1_MODIFIERS_MAP: u32 = 7;
    const ZWP_INPUT_METHOD_CONTEXT_V1_KEYSYM: u32 = 8;
    const ZWP_INPUT_METHOD_CONTEXT_V1_GRAB_KEYBOARD: u32 = 9;
    const ZWP_INPUT_METHOD_CONTEXT_V1_KEY: u32 = 10;
    const ZWP_INPUT_METHOD_CONTEXT_V1_MODIFIERS: u32 = 11;
    const ZWP_INPUT_METHOD_CONTEXT_V1_LANGUAGE: u32 = 12;
    const ZWP_INPUT_METHOD_CONTEXT_V1_TEXT_DIRECTION: u32 = 13;
    impl ZwpInputMethodContextV1 {
        ///
        /// This is a destructor, you cannot send requests to this object once this method is called.
        pub fn destroy(&self) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_DESTROY) };

            if let Some(ref data) = self.data {
                data.0.store(false, ::std::sync::atomic::Ordering::SeqCst);
            }
            let udata = unsafe { &mut *(ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, self.ptr()) as *mut UserData) };
            let _impl = udata.1.take();
            ::std::mem::drop(_impl);
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_destroy, self.ptr()); }
            RequestResult::Sent(())
        }
        /// commit string
        ///
        /// Send the commit string text for insertion to the application.
        /// 
        /// The text to commit could be either just a single character after a key
        /// press or the result of some composing (pre-edit). It could be also an
        /// empty text when some text should be removed (see
        /// delete_surrounding_text) or when the input cursor should be moved (see
        /// cursor_position).
        /// 
        /// Any previously set composing text will be removed.
        pub fn commit_string(&self, serial: u32, text: String) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let text = CString::new(text).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_input_method_context_v1.commit_string:text"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_COMMIT_STRING, serial, text.as_ptr()) };
            RequestResult::Sent(())
        }
        /// pre-edit string
        ///
        /// Send the pre-edit string text to the application text input.
        /// 
        /// The commit text can be used to replace the pre-edit text on reset (for
        /// example on unfocus).
        /// 
        /// Previously sent preedit_style and preedit_cursor requests are also
        /// processed by the text_input.
        pub fn preedit_string(&self, serial: u32, text: String, commit: String) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let text = CString::new(text).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_input_method_context_v1.preedit_string:text"));
            let commit = CString::new(commit).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_input_method_context_v1.preedit_string:commit"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_PREEDIT_STRING, serial, text.as_ptr(), commit.as_ptr()) };
            RequestResult::Sent(())
        }
        /// pre-edit styling
        ///
        /// Set the styling information on composing text. The style is applied for
        /// length in bytes from index relative to the beginning of
        /// the composing text (as byte offset). Multiple styles can
        /// be applied to a composing text.
        /// 
        /// This request should be sent before sending a preedit_string request.
        pub fn preedit_styling(&self, index: u32, length: u32, style: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_PREEDIT_STYLING, index, length, style) };
            RequestResult::Sent(())
        }
        /// pre-edit cursor
        ///
        /// Set the cursor position inside the composing text (as byte offset)
        /// relative to the start of the composing text.
        /// 
        /// When index is negative no cursor should be displayed.
        /// 
        /// This request should be sent before sending a preedit_string request.
        pub fn preedit_cursor(&self, index: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_PREEDIT_CURSOR, index) };
            RequestResult::Sent(())
        }
        /// delete text
        ///
        /// Remove the surrounding text.
        /// 
        /// This request will be handled on the text_input side directly following
        /// a commit_string request.
        pub fn delete_surrounding_text(&self, index: i32, length: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_DELETE_SURROUNDING_TEXT, index, length) };
            RequestResult::Sent(())
        }
        /// set cursor to a new position
        ///
        /// Set the cursor and anchor to a new position. Index is the new cursor
        /// position in bytes (when >= 0 this is relative to the end of the inserted text,
        /// otherwise it is relative to the beginning of the inserted text). Anchor is
        /// the new anchor position in bytes (when >= 0 this is relative to the end of the
        /// inserted text, otherwise it is relative to the beginning of the inserted
        /// text). When there should be no selected text, anchor should be the same
        /// as index.
        /// 
        /// This request will be handled on the text_input side directly following
        /// a commit_string request.
        pub fn cursor_position(&self, index: i32, anchor: i32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_CURSOR_POSITION, index, anchor) };
            RequestResult::Sent(())
        }
        pub fn modifiers_map(&self, map: Vec<u8>) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let map = wl_array { size: map.len(), alloc: map.capacity(), data: map.as_ptr() as *mut _ };
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_MODIFIERS_MAP, &map as *const wl_array) };
            RequestResult::Sent(())
        }
        /// keysym
        ///
        /// Notify when a key event was sent. Key events should not be used for
        /// normal text input operations, which should be done with commit_string,
        /// delete_surrounding_text, etc. The key event follows the wl_keyboard key
        /// event convention. Sym is an XKB keysym, state is a wl_keyboard key_state.
        pub fn keysym(&self, serial: u32, time: u32, sym: u32, state: u32, modifiers: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_KEYSYM, serial, time, sym, state, modifiers) };
            RequestResult::Sent(())
        }
        /// grab hardware keyboard
        ///
        /// Allow an input method to receive hardware keyboard input and process
        /// key events to generate text events (with pre-edit) over the wire. This
        /// allows input methods which compose multiple key events for inputting
        /// text like it is done for CJK languages.
        pub fn grab_keyboard(&self) ->RequestResult<super::wl_keyboard::WlKeyboard> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_GRAB_KEYBOARD, &wl_keyboard_interface, ptr::null_mut::<wl_proxy>()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            RequestResult::Sent(proxy)
        }
        /// forward key event
        ///
        /// Forward a wl_keyboard::key event to the client that was not processed
        /// by the input method itself. Should be used when filtering key events
        /// with grab_keyboard.  The arguments should be the ones from the
        /// wl_keyboard::key event.
        /// 
        /// For generating custom key events use the keysym request instead.
        pub fn key(&self, serial: u32, time: u32, key: u32, state: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_KEY, serial, time, key, state) };
            RequestResult::Sent(())
        }
        /// forward modifiers event
        ///
        /// Forward a wl_keyboard::modifiers event to the client that was not
        /// processed by the input method itself.  Should be used when filtering
        /// key events with grab_keyboard. The arguments should be the ones
        /// from the wl_keyboard::modifiers event.
        pub fn modifiers(&self, serial: u32, mods_depressed: u32, mods_latched: u32, mods_locked: u32, group: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_MODIFIERS, serial, mods_depressed, mods_latched, mods_locked, group) };
            RequestResult::Sent(())
        }
        pub fn language(&self, serial: u32, language: String) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            let language = CString::new(language).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_input_method_context_v1.language:language"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_LANGUAGE, serial, language.as_ptr()) };
            RequestResult::Sent(())
        }
        pub fn text_direction(&self, serial: u32, direction: u32) ->RequestResult<()> {
            if self.status() == Liveness::Dead { return RequestResult::Destroyed }
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_METHOD_CONTEXT_V1_TEXT_DIRECTION, serial, direction) };
            RequestResult::Sent(())
        }
    }
}
pub mod zwp_input_method_v1 {
    //! input method
    //!
    //! An input method object is responsible for composing text in response to
    //! input from hardware or virtual keyboards. There is one input method
    //! object per seat. On activate there is a new input method context object
    //! created which allows the input method to communicate with the text input.
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

    pub struct ZwpInputMethodV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpInputMethodV1 {}
    unsafe impl Sync for ZwpInputMethodV1 {}
    unsafe impl Proxy for ZwpInputMethodV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpInputMethodV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpInputMethodV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpInputMethodV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpInputMethodV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpInputMethodV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_input_method_v1_interface } }
        fn interface_name() -> &'static str { "zwp_input_method_v1"  }
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

        fn equals(&self, other: &ZwpInputMethodV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpInputMethodV1 {
            ZwpInputMethodV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpInputMethodV1 {
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
                    (implementation.activate)(evq, idata,  self, id);
                },
                1 => {
                    let context = {Proxy::from_ptr_initialized(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.deactivate)(evq, idata,  self, &context);
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
        /// activate event
        ///
        /// A text input was activated. Creates an input method context object
        /// which allows communication with the text input.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_input_method_v1, id
        pub activate: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_input_method_v1: &ZwpInputMethodV1, id: super::zwp_input_method_context_v1::ZwpInputMethodContextV1),
        /// deactivate event
        ///
        /// The text input corresponding to the context argument was deactivated.
        /// The input method context should be destroyed after deactivation is
        /// handled.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_input_method_v1, context
        pub deactivate: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_input_method_v1: &ZwpInputMethodV1, context: &super::zwp_input_method_context_v1::ZwpInputMethodContextV1),
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
            && (self.activate as usize == other.activate as usize)
            && (self.deactivate as usize == other.deactivate as usize)

        }
    }

    impl ZwpInputMethodV1 {
    }
}
pub mod zwp_input_panel_v1 {
    //! interface for implementing keyboards
    //!
    //! Only one client can bind this interface at a time.
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

    pub struct ZwpInputPanelV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpInputPanelV1 {}
    unsafe impl Sync for ZwpInputPanelV1 {}
    unsafe impl Proxy for ZwpInputPanelV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpInputPanelV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpInputPanelV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpInputPanelV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpInputPanelV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpInputPanelV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_input_panel_v1_interface } }
        fn interface_name() -> &'static str { "zwp_input_panel_v1"  }
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

        fn equals(&self, other: &ZwpInputPanelV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpInputPanelV1 {
            ZwpInputPanelV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZWP_INPUT_PANEL_V1_GET_INPUT_PANEL_SURFACE: u32 = 0;
    impl ZwpInputPanelV1 {
        pub fn get_input_panel_surface(&self, surface: &super::wl_surface::WlSurface) ->super::zwp_input_panel_surface_v1::ZwpInputPanelSurfaceV1 {
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_INPUT_PANEL_V1_GET_INPUT_PANEL_SURFACE, &zwp_input_panel_surface_v1_interface, ptr::null_mut::<wl_proxy>(), surface.ptr()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            proxy
        }
    }
}
pub mod zwp_input_panel_surface_v1 {
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

    pub struct ZwpInputPanelSurfaceV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpInputPanelSurfaceV1 {}
    unsafe impl Sync for ZwpInputPanelSurfaceV1 {}
    unsafe impl Proxy for ZwpInputPanelSurfaceV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpInputPanelSurfaceV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpInputPanelSurfaceV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpInputPanelSurfaceV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpInputPanelSurfaceV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpInputPanelSurfaceV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_input_panel_surface_v1_interface } }
        fn interface_name() -> &'static str { "zwp_input_panel_surface_v1"  }
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

        fn equals(&self, other: &ZwpInputPanelSurfaceV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpInputPanelSurfaceV1 {
            ZwpInputPanelSurfaceV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Position {
        CenterBottom = 0,
    }
    impl Position {
        pub fn from_raw(n: u32) -> Option<Position> {
            match n {
                0 => Some(Position::CenterBottom),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    const ZWP_INPUT_PANEL_SURFACE_V1_SET_TOPLEVEL: u32 = 0;
    const ZWP_INPUT_PANEL_SURFACE_V1_SET_OVERLAY_PANEL: u32 = 1;
    impl ZwpInputPanelSurfaceV1 {
        /// set the surface type as a keyboard
        ///
        /// Set the input_panel_surface type to keyboard.
        /// 
        /// A keyboard surface is only shown when a text input is active.
        pub fn set_toplevel(&self, output: &super::wl_output::WlOutput, position: u32) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_PANEL_SURFACE_V1_SET_TOPLEVEL, output.ptr(), position) };
        }
        /// set the surface type as an overlay panel
        ///
        /// Set the input_panel_surface to be an overlay panel.
        /// 
        /// This is shown near the input cursor above the application window when
        /// a text input is active.
        pub fn set_overlay_panel(&self) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_INPUT_PANEL_SURFACE_V1_SET_OVERLAY_PANEL) };
        }
    }
}
