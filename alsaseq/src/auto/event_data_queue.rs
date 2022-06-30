// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
use glib::translate::*;
use gobject_sys;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventDataQueue(Boxed<alsaseq_sys::ALSASeqEventDataQueue>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsaseq_sys::alsaseq_event_data_queue_get_type(), ptr as *mut _) as *mut alsaseq_sys::ALSASeqEventDataQueue,
        free => |ptr| gobject_sys::g_boxed_free(alsaseq_sys::alsaseq_event_data_queue_get_type(), ptr as *mut _),
        get_type => || alsaseq_sys::alsaseq_event_data_queue_get_type(),
    }
}

impl EventDataQueue {
    pub fn get_position_param(&self) -> u32 {
        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_queue_get_position_param(
                self.to_glib_none().0,
                position.as_mut_ptr(),
            );
            let position = position.assume_init();
            position
        }
    }

    pub fn get_queue_id(&self) -> u8 {
        unsafe {
            let mut queue_id = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_queue_get_queue_id(
                self.to_glib_none().0,
                queue_id.as_mut_ptr(),
            );
            let queue_id = queue_id.assume_init();
            queue_id
        }
    }

    pub fn get_tick_time_param(&self) -> u32 {
        unsafe {
            let mut tick_time = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_queue_get_tick_time_param(
                self.to_glib_none().0,
                tick_time.as_mut_ptr(),
            );
            let tick_time = tick_time.assume_init();
            tick_time
        }
    }

    pub fn get_value_param(&self) -> i32 {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_queue_get_value_param(
                self.to_glib_none().0,
                value.as_mut_ptr(),
            );
            let value = value.assume_init();
            value
        }
    }

    pub fn set_position_param(&mut self, position: u32) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_queue_set_position_param(
                self.to_glib_none_mut().0,
                position,
            );
        }
    }

    pub fn set_queue_id(&mut self, queue_id: u8) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_queue_set_queue_id(self.to_glib_none_mut().0, queue_id);
        }
    }

    pub fn set_tick_time_param(&mut self, tick_time: u32) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_queue_set_tick_time_param(
                self.to_glib_none_mut().0,
                tick_time,
            );
        }
    }

    pub fn set_value_param(&mut self, value: i32) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_queue_set_value_param(self.to_glib_none_mut().0, value);
        }
    }
}

unsafe impl Send for EventDataQueue {}
