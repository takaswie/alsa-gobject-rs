// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
use glib;
use glib::translate::*;
use gobject_sys;
use std::ptr;
use Addr;
use EventType;
use RemoveFilterFlag;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RemoveFilter(Boxed<alsaseq_sys::ALSASeqRemoveFilter>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsaseq_sys::alsaseq_remove_filter_get_type(), ptr as *mut _) as *mut alsaseq_sys::ALSASeqRemoveFilter,
        free => |ptr| gobject_sys::g_boxed_free(alsaseq_sys::alsaseq_remove_filter_get_type(), ptr as *mut _),
        get_type => || alsaseq_sys::alsaseq_remove_filter_get_type(),
    }
}

impl RemoveFilter {
    pub fn with_dest_addr(
        inout: RemoveFilterFlag,
        queue_id: u8,
        dest: &mut Addr,
    ) -> Result<RemoveFilter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsaseq_sys::alsaseq_remove_filter_new_with_dest_addr(
                inout.to_glib(),
                queue_id,
                dest.to_glib_none_mut().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_event_type(
        inout: RemoveFilterFlag,
        queue_id: u8,
        ev_type: EventType,
    ) -> Result<RemoveFilter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsaseq_sys::alsaseq_remove_filter_new_with_event_type(
                inout.to_glib(),
                queue_id,
                ev_type.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_note(inout: RemoveFilterFlag, queue_id: u8) -> Result<RemoveFilter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsaseq_sys::alsaseq_remove_filter_new_with_note(
                inout.to_glib(),
                queue_id,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_note_channel(
        inout: RemoveFilterFlag,
        queue_id: u8,
        channel: u8,
    ) -> Result<RemoveFilter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsaseq_sys::alsaseq_remove_filter_new_with_note_channel(
                inout.to_glib(),
                queue_id,
                channel,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_real_time(
        inout: RemoveFilterFlag,
        queue_id: u8,
        tv_sec: i32,
        tv_nsec: u32,
        after: bool,
    ) -> Result<RemoveFilter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsaseq_sys::alsaseq_remove_filter_new_with_real_time(
                inout.to_glib(),
                queue_id,
                tv_sec,
                tv_nsec,
                after.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_tag(
        inout: RemoveFilterFlag,
        queue_id: u8,
        tag: i8,
    ) -> Result<RemoveFilter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsaseq_sys::alsaseq_remove_filter_new_with_tag(
                inout.to_glib(),
                queue_id,
                tag,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn with_tick_time(
        inout: RemoveFilterFlag,
        queue_id: u8,
        tick_time: i32,
        after: bool,
    ) -> Result<RemoveFilter, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsaseq_sys::alsaseq_remove_filter_new_with_tick_time(
                inout.to_glib(),
                queue_id,
                tick_time,
                after.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl Send for RemoveFilter {}
