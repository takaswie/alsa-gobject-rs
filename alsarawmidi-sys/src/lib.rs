// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate libc;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type ALSARawmidiStreamDirection = c_int;
pub const ALSARAWMIDI_STREAM_DIRECTION_OUTPUT: ALSARawmidiStreamDirection = 0;
pub const ALSARAWMIDI_STREAM_DIRECTION_INPUT: ALSARawmidiStreamDirection = 1;

pub type ALSARawmidiStreamPairError = c_int;
pub const ALSARAWMIDI_STREAM_PAIR_ERROR_FAILED: ALSARawmidiStreamPairError = 0;
pub const ALSARAWMIDI_STREAM_PAIR_ERROR_DISCONNECTED: ALSARawmidiStreamPairError = 1;
pub const ALSARAWMIDI_STREAM_PAIR_ERROR_UNREADABLE: ALSARawmidiStreamPairError = 2;

// Flags
pub type ALSARawmidiStreamPairInfoFlag = c_uint;
pub const ALSARAWMIDI_STREAM_PAIR_INFO_FLAG_OUTPUT: ALSARawmidiStreamPairInfoFlag = 1;
pub const ALSARAWMIDI_STREAM_PAIR_INFO_FLAG_INPUT: ALSARawmidiStreamPairInfoFlag = 2;
pub const ALSARAWMIDI_STREAM_PAIR_INFO_FLAG_DUPLEX: ALSARawmidiStreamPairInfoFlag = 4;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSARawmidiStreamPairClass {
    pub parent_class: gobject::GObjectClass,
    pub handle_messages: Option<unsafe extern "C" fn(*mut ALSARawmidiStreamPair)>,
    pub handle_disconnection: Option<unsafe extern "C" fn(*mut ALSARawmidiStreamPair)>,
}

impl ::std::fmt::Debug for ALSARawmidiStreamPairClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSARawmidiStreamPairClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .field("handle_messages", &self.handle_messages)
        .field("handle_disconnection", &self.handle_disconnection)
        .finish()
    }
}

#[repr(C)]
pub struct _ALSARawmidiStreamPairPrivate(c_void);

pub type ALSARawmidiStreamPairPrivate = *mut _ALSARawmidiStreamPairPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSARawmidiSubstreamInfoClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSARawmidiSubstreamInfoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSARawmidiSubstreamInfoClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
pub struct _ALSARawmidiSubstreamInfoPrivate(c_void);

pub type ALSARawmidiSubstreamInfoPrivate = *mut _ALSARawmidiSubstreamInfoPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSARawmidiSubstreamParamsClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSARawmidiSubstreamParamsClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSARawmidiSubstreamParamsClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
pub struct _ALSARawmidiSubstreamParamsPrivate(c_void);

pub type ALSARawmidiSubstreamParamsPrivate = *mut _ALSARawmidiSubstreamParamsPrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSARawmidiSubstreamStatusClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSARawmidiSubstreamStatusClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSARawmidiSubstreamStatusClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
pub struct _ALSARawmidiSubstreamStatusPrivate(c_void);

pub type ALSARawmidiSubstreamStatusPrivate = *mut _ALSARawmidiSubstreamStatusPrivate;

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSARawmidiStreamPair {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ALSARawmidiStreamPairPrivate,
}

impl ::std::fmt::Debug for ALSARawmidiStreamPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSARawmidiStreamPair @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .field("priv_", &self.priv_)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSARawmidiSubstreamInfo {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ALSARawmidiSubstreamInfoPrivate,
}

impl ::std::fmt::Debug for ALSARawmidiSubstreamInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSARawmidiSubstreamInfo @ {:?}",
            self as *const _
        ))
        .field("parent_instance", &self.parent_instance)
        .field("priv_", &self.priv_)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSARawmidiSubstreamParams {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ALSARawmidiSubstreamParamsPrivate,
}

impl ::std::fmt::Debug for ALSARawmidiSubstreamParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSARawmidiSubstreamParams @ {:?}",
            self as *const _
        ))
        .field("parent_instance", &self.parent_instance)
        .field("priv_", &self.priv_)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSARawmidiSubstreamStatus {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ALSARawmidiSubstreamStatusPrivate,
}

impl ::std::fmt::Debug for ALSARawmidiSubstreamStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSARawmidiSubstreamStatus @ {:?}",
            self as *const _
        ))
        .field("parent_instance", &self.parent_instance)
        .field("priv_", &self.priv_)
        .finish()
    }
}

#[link(name = "alsarawmidi")]
extern "C" {

    //=========================================================================
    // ALSARawmidiStreamDirection
    //=========================================================================
    pub fn alsarawmidi_stream_direction_get_type() -> GType;

    //=========================================================================
    // ALSARawmidiStreamPairError
    //=========================================================================
    pub fn alsarawmidi_stream_pair_error_get_type() -> GType;
    pub fn alsarawmidi_stream_pair_error_quark() -> glib::GQuark;

    //=========================================================================
    // ALSARawmidiStreamPairInfoFlag
    //=========================================================================
    pub fn alsarawmidi_stream_pair_info_flag_get_type() -> GType;

    //=========================================================================
    // ALSARawmidiStreamPair
    //=========================================================================
    pub fn alsarawmidi_stream_pair_get_type() -> GType;
    pub fn alsarawmidi_stream_pair_new() -> *mut ALSARawmidiStreamPair;
    pub fn alsarawmidi_stream_pair_create_source(
        self_: *mut ALSARawmidiStreamPair,
        gsrc: *mut *mut glib::GSource,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_drain_substream(
        self_: *mut ALSARawmidiStreamPair,
        direction: ALSARawmidiStreamDirection,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_drop_substream(
        self_: *mut ALSARawmidiStreamPair,
        direction: ALSARawmidiStreamDirection,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_get_protocol_version(
        self_: *mut ALSARawmidiStreamPair,
        proto_ver_triplet: *mut *const [u16; 3],
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_get_substream_info(
        self_: *mut ALSARawmidiStreamPair,
        direction: ALSARawmidiStreamDirection,
        substream_info: *mut *mut ALSARawmidiSubstreamInfo,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_get_substream_status(
        self_: *mut ALSARawmidiStreamPair,
        direction: ALSARawmidiStreamDirection,
        substream_status: *const *mut ALSARawmidiSubstreamStatus,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_open(
        self_: *mut ALSARawmidiStreamPair,
        card_id: c_uint,
        device_id: c_uint,
        subdevice_id: c_uint,
        access_modes: ALSARawmidiStreamPairInfoFlag,
        open_flag: c_int,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_read_from_substream(
        self_: *mut ALSARawmidiStreamPair,
        buf: *const *mut u8,
        buf_size: *mut size_t,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_set_substream_params(
        self_: *mut ALSARawmidiStreamPair,
        direction: ALSARawmidiStreamDirection,
        substream_params: *mut ALSARawmidiSubstreamParams,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_stream_pair_write_to_substream(
        self_: *mut ALSARawmidiStreamPair,
        buf: *const u8,
        buf_size: size_t,
        error: *mut *mut glib::GError,
    );

    //=========================================================================
    // ALSARawmidiSubstreamInfo
    //=========================================================================
    pub fn alsarawmidi_substream_info_get_type() -> GType;

    //=========================================================================
    // ALSARawmidiSubstreamParams
    //=========================================================================
    pub fn alsarawmidi_substream_params_get_type() -> GType;
    pub fn alsarawmidi_substream_params_new() -> *mut ALSARawmidiSubstreamParams;

    //=========================================================================
    // ALSARawmidiSubstreamStatus
    //=========================================================================
    pub fn alsarawmidi_substream_status_get_type() -> GType;
    pub fn alsarawmidi_substream_status_new() -> *mut ALSARawmidiSubstreamStatus;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn alsarawmidi_get_device_id_list(
        card_id: c_uint,
        entries: *mut *mut c_uint,
        entry_count: *mut size_t,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_get_rawmidi_devnode(
        card_id: c_uint,
        device_id: c_uint,
        devnode: *mut *mut c_char,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_get_rawmidi_sysname(
        card_id: c_uint,
        device_id: c_uint,
        sysname: *mut *mut c_char,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_get_subdevice_id_list(
        card: c_uint,
        device: c_uint,
        direction: ALSARawmidiStreamDirection,
        entries: *mut *mut c_uint,
        entry_count: *mut size_t,
        error: *mut *mut glib::GError,
    );
    pub fn alsarawmidi_get_substream_info(
        card_id: c_uint,
        device_id: c_uint,
        direction: ALSARawmidiStreamDirection,
        subdevice_id: c_uint,
        substream_info: *mut *mut ALSARawmidiSubstreamInfo,
        error: *mut *mut glib::GError,
    );

}
