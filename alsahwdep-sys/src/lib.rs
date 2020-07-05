// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal)]

extern crate libc;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type ALSAHwdepIfaceType = c_int;
pub const ALSAHWDEP_IFACE_TYPE_OPL2: ALSAHwdepIfaceType = 0;
pub const ALSAHWDEP_IFACE_TYPE_OPL3: ALSAHwdepIfaceType = 1;
pub const ALSAHWDEP_IFACE_TYPE_OPL4: ALSAHwdepIfaceType = 2;
pub const ALSAHWDEP_IFACE_TYPE_SB16CSP: ALSAHwdepIfaceType = 3;
pub const ALSAHWDEP_IFACE_TYPE_EMU10K1: ALSAHwdepIfaceType = 4;
pub const ALSAHWDEP_IFACE_TYPE_YSS225: ALSAHwdepIfaceType = 5;
pub const ALSAHWDEP_IFACE_TYPE_ICS2115: ALSAHwdepIfaceType = 6;
pub const ALSAHWDEP_IFACE_TYPE_SSCAPE: ALSAHwdepIfaceType = 7;
pub const ALSAHWDEP_IFACE_TYPE_VX: ALSAHwdepIfaceType = 8;
pub const ALSAHWDEP_IFACE_TYPE_MIXART: ALSAHwdepIfaceType = 9;
pub const ALSAHWDEP_IFACE_TYPE_USX2Y: ALSAHwdepIfaceType = 10;
pub const ALSAHWDEP_IFACE_TYPE_EMUX_WAVETABLE: ALSAHwdepIfaceType = 11;
pub const ALSAHWDEP_IFACE_TYPE_BLUETOOTH: ALSAHwdepIfaceType = 12;
pub const ALSAHWDEP_IFACE_TYPE_USX2Y_PCM: ALSAHwdepIfaceType = 13;
pub const ALSAHWDEP_IFACE_TYPE_PCXHR: ALSAHwdepIfaceType = 14;
pub const ALSAHWDEP_IFACE_TYPE_SB_RC: ALSAHwdepIfaceType = 15;
pub const ALSAHWDEP_IFACE_TYPE_HDA: ALSAHwdepIfaceType = 16;
pub const ALSAHWDEP_IFACE_TYPE_USB_STREAM: ALSAHwdepIfaceType = 17;
pub const ALSAHWDEP_IFACE_TYPE_FW_DICE: ALSAHwdepIfaceType = 18;
pub const ALSAHWDEP_IFACE_TYPE_FW_FIREWORKS: ALSAHwdepIfaceType = 19;
pub const ALSAHWDEP_IFACE_TYPE_FW_BEBOB: ALSAHwdepIfaceType = 20;
pub const ALSAHWDEP_IFACE_TYPE_FW_OXFW: ALSAHwdepIfaceType = 21;
pub const ALSAHWDEP_IFACE_TYPE_FW_DIGI00X: ALSAHwdepIfaceType = 22;
pub const ALSAHWDEP_IFACE_TYPE_FW_TASCAM: ALSAHwdepIfaceType = 23;
pub const ALSAHWDEP_IFACE_TYPE_LINE6: ALSAHwdepIfaceType = 24;
pub const ALSAHWDEP_IFACE_TYPE_FW_MOTU: ALSAHwdepIfaceType = 25;
pub const ALSAHWDEP_IFACE_TYPE_FW_FIREFACE: ALSAHwdepIfaceType = 26;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSAHwdepDeviceInfoClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSAHwdepDeviceInfoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSAHwdepDeviceInfoClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[repr(C)]
pub struct _ALSAHwdepDeviceInfoPrivate(c_void);

pub type ALSAHwdepDeviceInfoPrivate = *mut _ALSAHwdepDeviceInfoPrivate;

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSAHwdepDeviceInfo {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut ALSAHwdepDeviceInfoPrivate,
}

impl ::std::fmt::Debug for ALSAHwdepDeviceInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSAHwdepDeviceInfo @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .field("priv_", &self.priv_)
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // ALSAHwdepIfaceType
    //=========================================================================
    pub fn alsahwdep_iface_type_get_type() -> GType;

    //=========================================================================
    // ALSAHwdepDeviceInfo
    //=========================================================================
    pub fn alsahwdep_device_info_get_type() -> GType;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn alsahwdep_get_device_id_list(card_id: c_uint, entries: *mut *mut c_uint, entry_count: *mut size_t, error: *mut *mut glib::GError);
    pub fn alsahwdep_get_device_info(card_id: c_uint, device_id: c_uint, device_info: *mut *mut ALSAHwdepDeviceInfo, error: *mut *mut glib::GError);
    pub fn alsahwdep_get_hwdep_devnode(card_id: c_uint, device_id: c_uint, devnode: *mut *mut c_char, error: *mut *mut glib::GError);
    pub fn alsahwdep_get_hwdep_sysname(card_id: c_uint, device_id: c_uint, sysname: *mut *mut c_char, error: *mut *mut glib::GError);

}