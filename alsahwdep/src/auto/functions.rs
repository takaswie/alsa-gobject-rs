// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsahwdep_sys;
use glib;
use glib::translate::*;
use std::mem;
use std::ptr;
use DeviceInfo;
use glib::GString;


pub fn get_device_id_list(card_id: u32) -> Result<Vec<u32>, glib::Error> {
    unsafe {
        let mut entries = ptr::null_mut();
        let mut entry_count = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = alsahwdep_sys::alsahwdep_get_device_id_list(card_id, &mut entries, entry_count.as_mut_ptr(), &mut error);
        if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(entries, entry_count.assume_init() as usize)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_device_info(card_id: u32, device_id: u32) -> Result<DeviceInfo, glib::Error> {
    unsafe {
        let mut device_info = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsahwdep_sys::alsahwdep_get_device_info(card_id, device_id, &mut device_info, &mut error);
        if error.is_null() { Ok(from_glib_full(device_info)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_hwdep_devnode(card_id: u32, device_id: u32) -> Result<GString, glib::Error> {
    unsafe {
        let mut devnode = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsahwdep_sys::alsahwdep_get_hwdep_devnode(card_id, device_id, &mut devnode, &mut error);
        if error.is_null() { Ok(from_glib_full(devnode)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_hwdep_sysname(card_id: u32, device_id: u32) -> Result<GString, glib::Error> {
    unsafe {
        let mut sysname = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsahwdep_sys::alsahwdep_get_hwdep_sysname(card_id, device_id, &mut sysname, &mut error);
        if error.is_null() { Ok(from_glib_full(sysname)) } else { Err(from_glib_full(error)) }
    }
}
