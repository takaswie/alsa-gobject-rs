// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
use gobject_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventCntr(Boxed<alsaseq_sys::ALSASeqEventCntr>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsaseq_sys::alsaseq_event_cntr_get_type(), ptr as *mut _) as *mut alsaseq_sys::ALSASeqEventCntr,
        free => |ptr| gobject_sys::g_boxed_free(alsaseq_sys::alsaseq_event_cntr_get_type(), ptr as *mut _),
        get_type => || alsaseq_sys::alsaseq_event_cntr_get_type(),
    }
}

unsafe impl Send for EventCntr {}
