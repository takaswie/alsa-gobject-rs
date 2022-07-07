// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Addr;
use glib::translate::*;

glib::wrapper! {
    /// A boxed object to express data of connect event.
    ///
    /// A [`EventDataConnect`][crate::EventDataConnect] is a boxed object to express data of connect event. The instance
    /// of object is one of data properties in event.
    ///
    /// The object wraps `struct snd_seq_connect` in UAPI of Linux sound subsystem.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventDataConnect(Boxed<ffi::ALSASeqEventDataConnect>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::alsaseq_event_data_connect_get_type(), ptr as *mut _) as *mut ffi::ALSASeqEventDataConnect,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::alsaseq_event_data_connect_get_type(), ptr as *mut _),
        type_ => || ffi::alsaseq_event_data_connect_get_type(),
    }
}

impl EventDataConnect {
    /// Set the source to the connection event.
    /// ## `dst`
    /// A [`Addr`][crate::Addr].
    #[doc(alias = "alsaseq_event_data_connect_set_dst")]
    pub fn set_dst(&mut self, dst: &Addr) {
        unsafe {
            ffi::alsaseq_event_data_connect_set_dst(
                self.to_glib_none_mut().0,
                dst.to_glib_none().0,
            );
        }
    }

    /// Set the source to the connection event.
    /// ## `src`
    /// A [`Addr`][crate::Addr].
    #[doc(alias = "alsaseq_event_data_connect_set_src")]
    pub fn set_src(&mut self, src: &Addr) {
        unsafe {
            ffi::alsaseq_event_data_connect_set_src(
                self.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
        }
    }
}

unsafe impl Send for EventDataConnect {}
