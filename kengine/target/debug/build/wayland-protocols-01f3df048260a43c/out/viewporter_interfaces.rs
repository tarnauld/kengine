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

use std::os::raw::{c_char, c_void};

use wayland_sys::common::*;

const NULLPTR: *const c_void = 0 as *const c_void;

static mut types_null: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];

// wp_viewporter

static mut wp_viewporter_requests_get_viewport_types: [*const wl_interface; 2] = [
    unsafe { &wp_viewport_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut wp_viewporter_requests: [wl_message; 2] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"get_viewport\0" as *const u8 as *const c_char, signature: b"no\0" as *const u8 as *const c_char, types: unsafe { &wp_viewporter_requests_get_viewport_types as *const _ } },
];

pub static mut wp_viewporter_interface: wl_interface = wl_interface {
    name: b"wp_viewporter\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &wp_viewporter_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// wp_viewport

pub static mut wp_viewport_requests: [wl_message; 3] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_source\0" as *const u8 as *const c_char, signature: b"ffff\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_destination\0" as *const u8 as *const c_char, signature: b"ii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut wp_viewport_interface: wl_interface = wl_interface {
    name: b"wp_viewport\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 3,
    requests: unsafe { &wp_viewport_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

