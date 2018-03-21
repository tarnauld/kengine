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

use std::os::raw::{c_char, c_void};

use wayland_sys::common::*;

const NULLPTR: *const c_void = 0 as *const c_void;

static mut types_null: [*const wl_interface; 1] = [
    NULLPTR as *const wl_interface,
];

// zxdg_exporter_v2

static mut zxdg_exporter_v2_requests_export_toplevel_types: [*const wl_interface; 2] = [
    unsafe { &zxdg_exported_v2_interface as *const wl_interface },
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zxdg_exporter_v2_requests: [wl_message; 2] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"export_toplevel\0" as *const u8 as *const c_char, signature: b"no\0" as *const u8 as *const c_char, types: unsafe { &zxdg_exporter_v2_requests_export_toplevel_types as *const _ } },
];

pub static mut zxdg_exporter_v2_interface: wl_interface = wl_interface {
    name: b"zxdg_exporter_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zxdg_exporter_v2_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// zxdg_importer_v2

static mut zxdg_importer_v2_requests_import_toplevel_types: [*const wl_interface; 2] = [
    unsafe { &zxdg_imported_v2_interface as *const wl_interface },
    NULLPTR as *const wl_interface,
];
pub static mut zxdg_importer_v2_requests: [wl_message; 2] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"import_toplevel\0" as *const u8 as *const c_char, signature: b"ns\0" as *const u8 as *const c_char, types: unsafe { &zxdg_importer_v2_requests_import_toplevel_types as *const _ } },
];

pub static mut zxdg_importer_v2_interface: wl_interface = wl_interface {
    name: b"zxdg_importer_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zxdg_importer_v2_requests as *const _ },
    event_count: 0,
    events: NULLPTR as *const wl_message,
};

// zxdg_exported_v2

pub static mut zxdg_exported_v2_requests: [wl_message; 1] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];
pub static mut zxdg_exported_v2_events: [wl_message; 1] = [
    wl_message { name: b"handle\0" as *const u8 as *const c_char, signature: b"s\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut zxdg_exported_v2_interface: wl_interface = wl_interface {
    name: b"zxdg_exported_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 1,
    requests: unsafe { &zxdg_exported_v2_requests as *const _ },
    event_count: 1,
    events: unsafe { &zxdg_exported_v2_events as *const _ },
};

// zxdg_imported_v2

static mut zxdg_imported_v2_requests_set_parent_of_types: [*const wl_interface; 1] = [
    unsafe { &wl_surface_interface as *const wl_interface },
];
pub static mut zxdg_imported_v2_requests: [wl_message; 2] = [
    wl_message { name: b"destroy\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
    wl_message { name: b"set_parent_of\0" as *const u8 as *const c_char, signature: b"o\0" as *const u8 as *const c_char, types: unsafe { &zxdg_imported_v2_requests_set_parent_of_types as *const _ } },
];
pub static mut zxdg_imported_v2_events: [wl_message; 1] = [
    wl_message { name: b"destroyed\0" as *const u8 as *const c_char, signature: b"\0" as *const u8 as *const c_char, types: unsafe { &types_null as *const _ } },
];

pub static mut zxdg_imported_v2_interface: wl_interface = wl_interface {
    name: b"zxdg_imported_v2\0" as *const u8 as *const c_char,
    version: 1,
    request_count: 2,
    requests: unsafe { &zxdg_imported_v2_requests as *const _ },
    event_count: 1,
    events: unsafe { &zxdg_imported_v2_events as *const _ },
};

