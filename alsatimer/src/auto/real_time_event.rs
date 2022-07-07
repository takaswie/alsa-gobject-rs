// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RealTimeEventType;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    /// A boxed object to express event of timer with real time.
    ///
    /// A [`RealTimeEvent`][crate::RealTimeEvent] includes real time at which the event is queued.
    ///
    /// The object wraps `struct snd_timer_tread` in UAPI of Linux sound subsystem.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RealTimeEvent(Boxed<ffi::ALSATimerRealTimeEvent>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::alsatimer_real_time_event_get_type(), ptr as *mut _) as *mut ffi::ALSATimerRealTimeEvent,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::alsatimer_real_time_event_get_type(), ptr as *mut _),
        type_ => || ffi::alsatimer_real_time_event_get_type(),
    }
}

impl RealTimeEvent {
    /// Get the kind of event for the real time event.
    ///
    /// # Returns
    ///
    ///
    /// ## `event`
    /// The type of real time event, one of [`RealTimeEventType`][crate::RealTimeEventType].
    #[doc(alias = "alsatimer_real_time_event_get_event")]
    #[doc(alias = "get_event")]
    pub fn event(&self) -> RealTimeEventType {
        unsafe {
            let mut event = mem::MaybeUninit::uninit();
            ffi::alsatimer_real_time_event_get_event(self.to_glib_none().0, event.as_mut_ptr());
            let event = event.assume_init();
            from_glib(event)
        }
    }

    /// Get the value depending on the type of real time event.
    ///
    /// # Returns
    ///
    ///
    /// ## `val`
    /// The value depending on the type of timestamp event.
    #[doc(alias = "alsatimer_real_time_event_get_val")]
    #[doc(alias = "get_val")]
    pub fn val(&self) -> u32 {
        unsafe {
            let mut val = mem::MaybeUninit::uninit();
            ffi::alsatimer_real_time_event_get_val(self.to_glib_none().0, val.as_mut_ptr());
            let val = val.assume_init();
            val
        }
    }
}

unsafe impl Send for RealTimeEvent {}
