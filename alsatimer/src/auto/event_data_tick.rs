// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsatimer_sys;
use glib::translate::*;
use gobject_sys;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventDataTick(Boxed<alsatimer_sys::ALSATimerEventDataTick>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsatimer_sys::alsatimer_event_data_tick_get_type(), ptr as *mut _) as *mut alsatimer_sys::ALSATimerEventDataTick,
        free => |ptr| gobject_sys::g_boxed_free(alsatimer_sys::alsatimer_event_data_tick_get_type(), ptr as *mut _),
        get_type => || alsatimer_sys::alsatimer_event_data_tick_get_type(),
    }
}

impl EventDataTick {
    pub fn get_resolution(&self) -> u32 {
        unsafe {
            let mut resolution = mem::MaybeUninit::uninit();
            alsatimer_sys::alsatimer_event_data_tick_get_resolution(
                self.to_glib_none().0,
                resolution.as_mut_ptr(),
            );
            let resolution = resolution.assume_init();
            resolution
        }
    }

    pub fn get_ticks(&self) -> u32 {
        unsafe {
            let mut ticks = mem::MaybeUninit::uninit();
            alsatimer_sys::alsatimer_event_data_tick_get_ticks(
                self.to_glib_none().0,
                ticks.as_mut_ptr(),
            );
            let ticks = ticks.assume_init();
            ticks
        }
    }
}

unsafe impl Send for EventDataTick {}
