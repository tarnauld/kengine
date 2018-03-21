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

use std::os::raw::{c_char, c_void};

use wayland_sys::common::*;

const NULLPTR: *const c_void = 0 as *const c_void;

static mut types_null: [*const wl_interface; 6] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];

// zwp_linux_dmabuf_v1

static mut zwp_linux_dmabuf_v1_requests_create_params_types: [*const wl_interface; 1] = [
    unsafe { &zwp_linux_buffer_params_v1_interface as *const wl_interface },
];
pub static mut zwp_linux_dmabuf_v1_requests: [wl_message; 2] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"create_params\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &zwp_linux_dmabuf_v1_requests_create_params_types as *const _ } },
];
pub static mut zwp_linux_dmabuf_v1_events: [wl_message; 2] = [
    wl_message { name: b"format\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"modifier\0" as *const u8 as *const c_char, signature: b"3uuu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut zwp_linux_dmabuf_v1_interface: wl_interface = wl_interface {
    name: b"zwp_linux_dmabuf_v1\0" as *const u8 as *const c_char,
    version: 3,
    request_count: 2,
    requests: unsafe { &zwp_linux_dmabuf_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwp_linux_dmabuf_v1_events as *const _ },
};

// zwp_linux_buffer_params_v1

static mut zwp_linux_buffer_params_v1_requests_create_immed_types: [*const wl_interface; 5] = [
    unsafe { &wl_buffer_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut zwp_linux_buffer_params_v1_requests: [wl_message; 4] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"add\0" as *const u8 as *const c_char, signature: b"huuuuu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"create\0" as *const u8 as *const c_char, signature: b"iiuu\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"create_immed\0" as *const u8 as *const c_char, signature: b"2niiuu\0" as *const u8 as *const c_char, types: unsafe { &zwp_linux_buffer_params_v1_requests_create_immed_types as *const _ } },
];
static mut zwp_linux_buffer_params_v1_events_created_types: [*const wl_interface; 1] = [
    unsafe { &wl_buffer_interface as *const wl_interface },
];
pub static mut zwp_linux_buffer_params_v1_events: [wl_message; 2] = [
    wl_message { name: b"created\0" as *const u8 as *const c_char, signature: b"n\0" as *const u8 as *const c_char, types: unsafe { &zwp_linux_buffer_params_v1_events_created_types as *const _ } },
    wl_message { name: b"failed\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut zwp_linux_buffer_params_v1_interface: wl_interface = wl_interface {
    name: b"zwp_linux_buffer_params_v1\0" as *const u8 as *const c_char,
    version: 3,
    request_count: 4,
    requests: unsafe { &zwp_linux_buffer_params_v1_requests as *const _ },
    event_count: 2,
    events: unsafe { &zwp_linux_buffer_params_v1_events as *const _ },
};

