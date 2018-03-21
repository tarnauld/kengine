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

use std::os::raw::{c_char, c_void};

use wayland_sys::common::*;

const NULLPTR: *const c_void = 0 as *const c_void;

static mut types_null: [*const wl_interface; 4] = [
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];

// xdg_shell

static mut xdg_shell_requests_get_xdg_surface_types: [*const wl_interface; 2] = [
    unsafe { &xdg_surface_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
static mut xdg_shell_requests_get_xdg_popup_types: [*const wl_interface; 7] = [
    unsafe { &xdg_popup_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
pub static mut xdg_shell_requests: [wl_message; 5] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"use_unstable_version\0" as *const u8 as *const c_char, signature: b"i\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"get_xdg_surface\0" as *const u8 as *const c_char, signature: b"no\0" as *const u8 as *const c_char, types: unsafe { &xdg_shell_requests_get_xdg_surface_types as *const _ } },
    wl_message { name: b"get_xdg_popup\0" as *const u8 as *const c_char, signature: b"nooouii\0" as *const u8 as *const c_char, types: unsafe { &xdg_shell_requests_get_xdg_popup_types as *const _ } },
    wl_message { name: b"pong\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut xdg_shell_events: [wl_message; 1] = [
    wl_message { name: b"ping\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut xdg_shell_interface: wl_interface = wl_interface {
    name: b"xdg_shell\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 5,
    requests: unsafe { &xdg_shell_requests as *const _ },
    event_count: 1,
    events: unsafe { &xdg_shell_events as *const _ },
};

// xdg_surface

static mut xdg_surface_requests_set_parent_types: [*const wl_interface; 1] = [
    unsafe { &xdg_surface_interface as *const wl_interface },
];
static mut xdg_surface_requests_show_window_menu_types: [*const wl_interface; 4] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut xdg_surface_requests_move_types: [*const wl_interface; 2] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
static mut xdg_surface_requests_resize_types: [*const wl_interface; 3] = [
    unsafe { &wl_seat_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
    NULLPTR as *const wl_interface,
];
static mut xdg_surface_requests_set_fullscreen_types: [*const wl_interface; 1] = [
    unsafe { &wl_output_interface as *const wl_interface },
];
pub static mut xdg_surface_requests: [wl_message; 14] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_parent\0" as *const u8 as *const c_char, signature: b"?o\0" as *const u8 as *const c_char, types: unsafe { &xdg_surface_requests_set_parent_types as *const _ } },
    wl_message { name: b"set_title\0" as *const u8 as *const c_char, signature: b"s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_app_id\0" as *const u8 as *const c_char, signature: b"s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"show_window_menu\0" as *const u8 as *const c_char, signature: b"ouii\0" as *const u8 as *const c_char, types: unsafe { &xdg_surface_requests_show_window_menu_types as *const _ } },
    wl_message { name: b"move\0" as *const u8 as *const c_char, signature: b"ou\0" as *const u8 as *const c_char, types: unsafe { &xdg_surface_requests_move_types as *const _ } },
    wl_message { name: b"resize\0" as *const u8 as *const c_char, signature: b"ouu\0" as *const u8 as *const c_char, types: unsafe { &xdg_surface_requests_resize_types as *const _ } },
    wl_message { name: b"ack_configure\0" as *const u8 as *const c_char, signature: b"u\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_window_geometry\0" as *const u8 as *const c_char, signature: b"iiii\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_maximized\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"unset_maximized\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_fullscreen\0" as *const u8 as *const c_char, signature: b"?o\0" as *const u8 as *const c_char, types: unsafe { &xdg_surface_requests_set_fullscreen_types as *const _ } },
    wl_message { name: b"unset_fullscreen\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_minimized\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut xdg_surface_events: [wl_message; 2] = [
    wl_message { name: b"configure\0" as *const u8 as *const c_char, signature: b"iiau\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"close\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut xdg_surface_interface: wl_interface = wl_interface {
    name: b"xdg_surface\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 14,
    requests: unsafe { &xdg_surface_requests as *const _ },
    event_count: 2,
    events: unsafe { &xdg_surface_events as *const _ },
};

// xdg_popup

pub static mut xdg_popup_requests: [wl_message; 1] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut xdg_popup_events: [wl_message; 1] = [
    wl_message { name: b"popup_done\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut xdg_popup_interface: wl_interface = wl_interface {
    name: b"xdg_popup\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &xdg_popup_requests as *const _ },
    event_count: 1,
    events: unsafe { &xdg_popup_events as *const _ },
};

