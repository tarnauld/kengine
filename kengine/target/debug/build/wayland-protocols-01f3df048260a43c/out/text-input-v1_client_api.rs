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

pub mod zwp_text_input_v1 {
    //! text input
    //!
    //! An object used for text input. Adds support for text input and input
    //! methods to applications. A text_input object is created from a
    //! wl_text_input_manager and corresponds typically to a text entry in an
    //! application.
    //! 
    //! Requests are used to activate/deactivate the text_input object and set
    //! state information like surrounding and selected text or the content type.
    //! The information about entered text is sent to the text_input object via
    //! the pre-edit and commit events. Using this interface removes the need
    //! for applications to directly process hardware key events and compose text
    //! out of them.
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

    pub struct ZwpTextInputV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTextInputV1 {}
    unsafe impl Sync for ZwpTextInputV1 {}
    unsafe impl Proxy for ZwpTextInputV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTextInputV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTextInputV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTextInputV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTextInputV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTextInputV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_text_input_v1_interface } }
        fn interface_name() -> &'static str { "zwp_text_input_v1"  }
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

        fn equals(&self, other: &ZwpTextInputV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTextInputV1 {
            ZwpTextInputV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    unsafe impl<ID: 'static> Implementable<ID> for ZwpTextInputV1 {
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
                    let surface = {Proxy::from_ptr_initialized(*(args.offset(0) as *const *mut wl_proxy))};
                    (implementation.enter)(evq, idata,  self, &surface);
                },
                1 => {
                    (implementation.leave)(evq, idata,  self);
                },
                2 => {
                    let map = {let array = *(args.offset(0) as *const *mut wl_array); ::std::slice::from_raw_parts((*array).data as *const u8, (*array).size as usize).to_owned()};
                    (implementation.modifiers_map)(evq, idata,  self, map);
                },
                3 => {
                    let state = {*(args.offset(0) as *const u32)};
                    (implementation.input_panel_state)(evq, idata,  self, state);
                },
                4 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let text = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(1) as *const *const _)).to_bytes()).into_owned()};
                    let commit = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(2) as *const *const _)).to_bytes()).into_owned()};
                    (implementation.preedit_string)(evq, idata,  self, serial, text, commit);
                },
                5 => {
                    let index = {*(args.offset(0) as *const u32)};
                    let length = {*(args.offset(1) as *const u32)};
                    let style = {*(args.offset(2) as *const u32)};
                    (implementation.preedit_styling)(evq, idata,  self, index, length, style);
                },
                6 => {
                    let index = {*(args.offset(0) as *const i32)};
                    (implementation.preedit_cursor)(evq, idata,  self, index);
                },
                7 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let text = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(1) as *const *const _)).to_bytes()).into_owned()};
                    (implementation.commit_string)(evq, idata,  self, serial, text);
                },
                8 => {
                    let index = {*(args.offset(0) as *const i32)};
                    let anchor = {*(args.offset(1) as *const i32)};
                    (implementation.cursor_position)(evq, idata,  self, index, anchor);
                },
                9 => {
                    let index = {*(args.offset(0) as *const i32)};
                    let length = {*(args.offset(1) as *const u32)};
                    (implementation.delete_surrounding_text)(evq, idata,  self, index, length);
                },
                10 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let time = {*(args.offset(1) as *const u32)};
                    let sym = {*(args.offset(2) as *const u32)};
                    let state = {*(args.offset(3) as *const u32)};
                    let modifiers = {*(args.offset(4) as *const u32)};
                    (implementation.keysym)(evq, idata,  self, serial, time, sym, state, modifiers);
                },
                11 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let language = {String::from_utf8_lossy(CStr::from_ptr(*(args.offset(1) as *const *const _)).to_bytes()).into_owned()};
                    (implementation.language)(evq, idata,  self, serial, language);
                },
                12 => {
                    let serial = {*(args.offset(0) as *const u32)};
                    let direction = {*(args.offset(1) as *const u32)};
                    (implementation.text_direction)(evq, idata,  self, serial, direction);
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
    /// content hint
    ///
    /// Content hint is a bitmask to allow to modify the behavior of the text
    /// input.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ContentHint {
        /// no special behaviour
        None = 0x0,
        /// auto completion, correction and capitalization
        Default = 0x7,
        /// hidden and sensitive text
        Password = 0xc0,
        /// suggest word completions
        AutoCompletion = 0x1,
        /// suggest word corrections
        AutoCorrection = 0x2,
        /// switch to uppercase letters at the start of a sentence
        AutoCapitalization = 0x4,
        /// prefer lowercase letters
        Lowercase = 0x8,
        /// prefer uppercase letters
        Uppercase = 0x10,
        /// prefer casing for titles and headings (can be language dependent)
        Titlecase = 0x20,
        /// characters should be hidden
        HiddenText = 0x40,
        /// typed text should not be stored
        SensitiveData = 0x80,
        /// just latin characters should be entered
        Latin = 0x100,
        /// the text input is multiline
        Multiline = 0x200,
    }
    impl ContentHint {
        pub fn from_raw(n: u32) -> Option<ContentHint> {
            match n {
                0x0 => Some(ContentHint::None),
                0x7 => Some(ContentHint::Default),
                0xc0 => Some(ContentHint::Password),
                0x1 => Some(ContentHint::AutoCompletion),
                0x2 => Some(ContentHint::AutoCorrection),
                0x4 => Some(ContentHint::AutoCapitalization),
                0x8 => Some(ContentHint::Lowercase),
                0x10 => Some(ContentHint::Uppercase),
                0x20 => Some(ContentHint::Titlecase),
                0x40 => Some(ContentHint::HiddenText),
                0x80 => Some(ContentHint::SensitiveData),
                0x100 => Some(ContentHint::Latin),
                0x200 => Some(ContentHint::Multiline),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    /// content purpose
    ///
    /// The content purpose allows to specify the primary purpose of a text
    /// input.
    /// 
    /// This allows an input method to show special purpose input panels with
    /// extra characters or to disallow some characters.
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ContentPurpose {
        /// default input, allowing all characters
        Normal = 0,
        /// allow only alphabetic characters
        Alpha = 1,
        /// allow only digits
        Digits = 2,
        /// input a number (including decimal separator and sign)
        Number = 3,
        /// input a phone number
        Phone = 4,
        /// input an URL
        Url = 5,
        /// input an email address
        Email = 6,
        /// input a name of a person
        Name = 7,
        /// input a password (combine with password or sensitive_data hint)
        Password = 8,
        /// input a date
        Date = 9,
        /// input a time
        Time = 10,
        /// input a date and time
        Datetime = 11,
        /// input for a terminal
        Terminal = 12,
    }
    impl ContentPurpose {
        pub fn from_raw(n: u32) -> Option<ContentPurpose> {
            match n {
                0 => Some(ContentPurpose::Normal),
                1 => Some(ContentPurpose::Alpha),
                2 => Some(ContentPurpose::Digits),
                3 => Some(ContentPurpose::Number),
                4 => Some(ContentPurpose::Phone),
                5 => Some(ContentPurpose::Url),
                6 => Some(ContentPurpose::Email),
                7 => Some(ContentPurpose::Name),
                8 => Some(ContentPurpose::Password),
                9 => Some(ContentPurpose::Date),
                10 => Some(ContentPurpose::Time),
                11 => Some(ContentPurpose::Datetime),
                12 => Some(ContentPurpose::Terminal),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum PreeditStyle {
        /// default style for composing text
        Default = 0,
        /// style should be the same as in non-composing text
        None = 1,
        Active = 2,
        Inactive = 3,
        Highlight = 4,
        Underline = 5,
        Selection = 6,
        Incorrect = 7,
    }
    impl PreeditStyle {
        pub fn from_raw(n: u32) -> Option<PreeditStyle> {
            match n {
                0 => Some(PreeditStyle::Default),
                1 => Some(PreeditStyle::None),
                2 => Some(PreeditStyle::Active),
                3 => Some(PreeditStyle::Inactive),
                4 => Some(PreeditStyle::Highlight),
                5 => Some(PreeditStyle::Underline),
                6 => Some(PreeditStyle::Selection),
                7 => Some(PreeditStyle::Incorrect),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum TextDirection {
        /// automatic text direction based on text and language
        Auto = 0,
        /// left-to-right
        Ltr = 1,
        /// right-to-left
        Rtl = 2,
    }
    impl TextDirection {
        pub fn from_raw(n: u32) -> Option<TextDirection> {
            match n {
                0 => Some(TextDirection::Auto),
                1 => Some(TextDirection::Ltr),
                2 => Some(TextDirection::Rtl),
                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub struct Implementation<ID> {
        /// enter event
        ///
        /// Notify the text_input object when it received focus. Typically in
        /// response to an activate request.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, surface
        pub enter: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, surface: &super::wl_surface::WlSurface),
        /// leave event
        ///
        /// Notify the text_input object when it lost focus. Either in response
        /// to a deactivate request or when the assigned surface lost focus or was
        /// destroyed.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1
        pub leave: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1),
        /// modifiers map
        ///
        /// Transfer an array of 0-terminated modifier names. The position in
        /// the array is the index of the modifier as used in the modifiers
        /// bitmask in the keysym event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, map
        pub modifiers_map: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, map: Vec<u8>),
        /// state of the input panel
        ///
        /// Notify when the visibility state of the input panel changed.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, state
        pub input_panel_state: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, state: u32),
        /// pre-edit
        ///
        /// Notify when a new composing text (pre-edit) should be set around the
        /// current cursor position. Any previously set composing text should
        /// be removed.
        /// 
        /// The commit text can be used to replace the preedit text on reset
        /// (for example on unfocus).
        /// 
        /// The text input should also handle all preedit_style and preedit_cursor
        /// events occurring directly before preedit_string.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, serial, text, commit
        pub preedit_string: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, serial: u32, text: String, commit: String),
        /// pre-edit styling
        ///
        /// Sets styling information on composing text. The style is applied for
        /// length bytes from index relative to the beginning of the composing
        /// text (as byte offset). Multiple styles can
        /// be applied to a composing text by sending multiple preedit_styling
        /// events.
        /// 
        /// This event is handled as part of a following preedit_string event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, index, length, style
        pub preedit_styling: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, index: u32, length: u32, style: u32),
        /// pre-edit cursor
        ///
        /// Sets the cursor position inside the composing text (as byte
        /// offset) relative to the start of the composing text. When index is a
        /// negative number no cursor is shown.
        /// 
        /// This event is handled as part of a following preedit_string event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, index
        pub preedit_cursor: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, index: i32),
        /// commit
        ///
        /// Notify when text should be inserted into the editor widget. The text to
        /// commit could be either just a single character after a key press or the
        /// result of some composing (pre-edit). It could also be an empty text
        /// when some text should be removed (see delete_surrounding_text) or when
        /// the input cursor should be moved (see cursor_position).
        /// 
        /// Any previously set composing text should be removed.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, serial, text
        pub commit_string: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, serial: u32, text: String),
        /// set cursor to new position
        ///
        /// Notify when the cursor or anchor position should be modified.
        /// 
        /// This event should be handled as part of a following commit_string
        /// event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, index, anchor
        pub cursor_position: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, index: i32, anchor: i32),
        /// delete surrounding text
        ///
        /// Notify when the text around the current cursor position should be
        /// deleted.
        /// 
        /// Index is relative to the current cursor (in bytes).
        /// Length is the length of deleted text (in bytes).
        /// 
        /// This event should be handled as part of a following commit_string
        /// event.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, index, length
        pub delete_surrounding_text: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, index: i32, length: u32),
        /// keysym
        ///
        /// Notify when a key event was sent. Key events should not be used
        /// for normal text input operations, which should be done with
        /// commit_string, delete_surrounding_text, etc. The key event follows
        /// the wl_keyboard key event convention. Sym is an XKB keysym, state a
        /// wl_keyboard key_state. Modifiers are a mask for effective modifiers
        /// (where the modifier indices are set by the modifiers_map event)
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, serial, time, sym, state, modifiers
        pub keysym: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, serial: u32, time: u32, sym: u32, state: u32, modifiers: u32),
        /// language
        ///
        /// Sets the language of the input text. The "language" argument is an
        /// RFC-3066 format language tag.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, serial, language
        pub language: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, serial: u32, language: String),
        /// text direction
        ///
        /// Sets the text direction of input text.
        /// 
        /// It is mainly needed for showing an input cursor on the correct side of
        /// the editor when there is no input done yet and making sure neutral
        /// direction text is laid out properly.
        ///
        /// **Arguments:** event_queue_handle, interface_data, zwp_text_input_v1, serial, direction
        pub text_direction: fn(evqh: &mut EventQueueHandle, data: &mut ID,  zwp_text_input_v1: &ZwpTextInputV1, serial: u32, direction: u32),
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
            && (self.enter as usize == other.enter as usize)
            && (self.leave as usize == other.leave as usize)
            && (self.modifiers_map as usize == other.modifiers_map as usize)
            && (self.input_panel_state as usize == other.input_panel_state as usize)
            && (self.preedit_string as usize == other.preedit_string as usize)
            && (self.preedit_styling as usize == other.preedit_styling as usize)
            && (self.preedit_cursor as usize == other.preedit_cursor as usize)
            && (self.commit_string as usize == other.commit_string as usize)
            && (self.cursor_position as usize == other.cursor_position as usize)
            && (self.delete_surrounding_text as usize == other.delete_surrounding_text as usize)
            && (self.keysym as usize == other.keysym as usize)
            && (self.language as usize == other.language as usize)
            && (self.text_direction as usize == other.text_direction as usize)

        }
    }

    const ZWP_TEXT_INPUT_V1_ACTIVATE: u32 = 0;
    const ZWP_TEXT_INPUT_V1_DEACTIVATE: u32 = 1;
    const ZWP_TEXT_INPUT_V1_SHOW_INPUT_PANEL: u32 = 2;
    const ZWP_TEXT_INPUT_V1_HIDE_INPUT_PANEL: u32 = 3;
    const ZWP_TEXT_INPUT_V1_RESET: u32 = 4;
    const ZWP_TEXT_INPUT_V1_SET_SURROUNDING_TEXT: u32 = 5;
    const ZWP_TEXT_INPUT_V1_SET_CONTENT_TYPE: u32 = 6;
    const ZWP_TEXT_INPUT_V1_SET_CURSOR_RECTANGLE: u32 = 7;
    const ZWP_TEXT_INPUT_V1_SET_PREFERRED_LANGUAGE: u32 = 8;
    const ZWP_TEXT_INPUT_V1_COMMIT_STATE: u32 = 9;
    const ZWP_TEXT_INPUT_V1_INVOKE_ACTION: u32 = 10;
    impl ZwpTextInputV1 {
        /// request activation
        ///
        /// Requests the text_input object to be activated (typically when the
        /// text entry gets focus).
        /// 
        /// The seat argument is a wl_seat which maintains the focus for this
        /// activation. The surface argument is a wl_surface assigned to the
        /// text_input object and tracked for focus lost. The enter event
        /// is emitted on successful activation.
        pub fn activate(&self, seat: &super::wl_seat::WlSeat, surface: &super::wl_surface::WlSurface) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_ACTIVATE, seat.ptr(), surface.ptr()) };
        }
        /// request deactivation
        ///
        /// Requests the text_input object to be deactivated (typically when the
        /// text entry lost focus). The seat argument is a wl_seat which was used
        /// for activation.
        pub fn deactivate(&self, seat: &super::wl_seat::WlSeat) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_DEACTIVATE, seat.ptr()) };
        }
        /// show input panels
        ///
        /// Requests input panels (virtual keyboard) to show.
        pub fn show_input_panel(&self) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_SHOW_INPUT_PANEL) };
        }
        /// hide input panels
        ///
        /// Requests input panels (virtual keyboard) to hide.
        pub fn hide_input_panel(&self) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_HIDE_INPUT_PANEL) };
        }
        /// reset
        ///
        /// Should be called by an editor widget when the input state should be
        /// reset, for example after the text was changed outside of the normal
        /// input method flow.
        pub fn reset(&self) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_RESET) };
        }
        /// sets the surrounding text
        ///
        /// Sets the plain surrounding text around the input position. Text is
        /// UTF-8 encoded. Cursor is the byte offset within the
        /// surrounding text. Anchor is the byte offset of the
        /// selection anchor within the surrounding text. If there is no selected
        /// text anchor, then it is the same as cursor.
        pub fn set_surrounding_text(&self, text: String, cursor: u32, anchor: u32) ->() {
            let text = CString::new(text).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_text_input_v1.set_surrounding_text:text"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_SET_SURROUNDING_TEXT, text.as_ptr(), cursor, anchor) };
        }
        /// set content purpose and hint
        ///
        /// Sets the content purpose and content hint. While the purpose is the
        /// basic purpose of an input field, the hint flags allow to modify some
        /// of the behavior.
        /// 
        /// When no content type is explicitly set, a normal content purpose with
        /// default hints (auto completion, auto correction, auto capitalization)
        /// should be assumed.
        pub fn set_content_type(&self, hint: u32, purpose: u32) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_SET_CONTENT_TYPE, hint, purpose) };
        }
        pub fn set_cursor_rectangle(&self, x: i32, y: i32, width: i32, height: i32) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_SET_CURSOR_RECTANGLE, x, y, width, height) };
        }
        /// sets preferred language
        ///
        /// Sets a specific language. This allows for example a virtual keyboard to
        /// show a language specific layout. The "language" argument is an RFC-3066
        /// format language tag.
        /// 
        /// It could be used for example in a word processor to indicate the
        /// language of the currently edited document or in an instant message
        /// application which tracks languages of contacts.
        pub fn set_preferred_language(&self, language: String) ->() {
            let language = CString::new(language).unwrap_or_else(|_| panic!("Got a String with interior null in zwp_text_input_v1.set_preferred_language:language"));
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_SET_PREFERRED_LANGUAGE, language.as_ptr()) };
        }
        pub fn commit_state(&self, serial: u32) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_COMMIT_STATE, serial) };
        }
        pub fn invoke_action(&self, button: u32, index: u32) ->() {
            unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal, self.ptr(), ZWP_TEXT_INPUT_V1_INVOKE_ACTION, button, index) };
        }
    }
}
pub mod zwp_text_input_manager_v1 {
    //! text input manager
    //!
    //! A factory for text_input objects. This object is a global singleton.
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

    pub struct ZwpTextInputManagerV1 {
        ptr: *mut wl_proxy,
        data: Option<Arc<(AtomicBool, AtomicPtr<()>)>>
    }

    unsafe impl Send for ZwpTextInputManagerV1 {}
    unsafe impl Sync for ZwpTextInputManagerV1 {}
    unsafe impl Proxy for ZwpTextInputManagerV1 {
        fn ptr(&self) -> *mut wl_proxy { self.ptr }

        unsafe fn from_ptr_new(ptr: *mut wl_proxy) -> ZwpTextInputManagerV1 {
            let data: *mut UserData = Box::into_raw(Box::new((
                ptr::null_mut(),
                Option::None,
                Arc::new((AtomicBool::new(true), AtomicPtr::new(ptr::null_mut()))),
            )));
            ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_set_user_data, ptr, data as *mut c_void);
            ZwpTextInputManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
        }
        unsafe fn from_ptr_initialized(ptr: *mut wl_proxy) -> ZwpTextInputManagerV1 {

            let implem = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_listener, ptr);
            let rust_managed = implem == &RUST_MANAGED as *const _ as *const _;

            if rust_managed {
                let data = ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_get_user_data, ptr) as *mut UserData;
                ZwpTextInputManagerV1 { ptr: ptr, data: Some((&*data).2.clone()) }
            } else {
                ZwpTextInputManagerV1 { ptr: ptr, data: Option::None }
            }
        }

        fn interface_ptr() -> *const wl_interface { unsafe { &zwp_text_input_manager_v1_interface } }
        fn interface_name() -> &'static str { "zwp_text_input_manager_v1"  }
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

        fn equals(&self, other: &ZwpTextInputManagerV1) -> bool {
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

        unsafe fn clone_unchecked(&self) -> ZwpTextInputManagerV1 {
            ZwpTextInputManagerV1 {
                ptr: self.ptr,
                data: self.data.clone()
            }
        }
    }
    const ZWP_TEXT_INPUT_MANAGER_V1_CREATE_TEXT_INPUT: u32 = 0;
    impl ZwpTextInputManagerV1 {
        /// create text input
        ///
        /// Creates a new text_input object.
        pub fn create_text_input(&self) ->super::zwp_text_input_v1::ZwpTextInputV1 {
            let ptr = unsafe { ffi_dispatch!(WAYLAND_CLIENT_HANDLE, wl_proxy_marshal_constructor, self.ptr(), ZWP_TEXT_INPUT_MANAGER_V1_CREATE_TEXT_INPUT, &zwp_text_input_v1_interface, ptr::null_mut::<wl_proxy>()) };
            let proxy = unsafe { Proxy::from_ptr_new(ptr) };
            proxy
        }
    }
}
