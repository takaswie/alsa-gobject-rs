// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    /// A GObject-derived object to express status of queue.
    ///
    /// A [`QueueStatus`][crate::QueueStatus] is a GObject-derived object to express status of queue. The call of
    /// [`queue_status()`][crate::queue_status()] returns the instance of object.
    ///
    /// The object wraps `struct snd_seq_queue_status` in UAPI of Linux sound subsystem.
    ///
    /// ## Properties
    ///
    ///
    /// #### `event-count`
    ///  The number of available events in the queue.
    ///
    /// Readable
    ///
    ///
    /// #### `queue-id`
    ///  The numeric ID of queue. An entry of ALSASeqSpecificQueueId is available as well.
    ///
    /// Readable
    ///
    ///
    /// #### `running`
    ///  Whether the queue is running or not.
    ///
    /// Readable
    ///
    /// # Implements
    ///
    /// [`QueueStatusExt`][trait@crate::prelude::QueueStatusExt], [`QueueStatusExtManual`][trait@crate::prelude::QueueStatusExtManual]
    #[doc(alias = "ALSASeqQueueStatus")]
    pub struct QueueStatus(Object<ffi::ALSASeqQueueStatus, ffi::ALSASeqQueueStatusClass>);

    match fn {
        type_ => || ffi::alsaseq_queue_status_get_type(),
    }
}

impl QueueStatus {
    pub const NONE: Option<&'static QueueStatus> = None;

    /// Allocate and returns an instance of [`QueueStatus`][crate::QueueStatus].
    ///
    /// # Returns
    ///
    /// An instance of [`QueueStatus`][crate::QueueStatus].
    #[doc(alias = "alsaseq_queue_status_new")]
    pub fn new() -> QueueStatus {
        unsafe { from_glib_full(ffi::alsaseq_queue_status_new()) }
    }
}

impl Default for QueueStatus {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::QueueStatus>> Sealed for T {}
}

/// Trait containing the part of [`struct@QueueStatus`] methods.
///
/// # Implementors
///
/// [`QueueStatus`][struct@crate::QueueStatus]
pub trait QueueStatusExt: IsA<QueueStatus> + sealed::Sealed + 'static {
    /// Get time as MIDI ticks.
    ///
    /// # Returns
    ///
    ///
    /// ## `tick_time`
    /// The value of MIDI ticks.
    #[doc(alias = "alsaseq_queue_status_get_tick_time")]
    #[doc(alias = "get_tick_time")]
    fn tick_time(&self) -> u32 {
        unsafe {
            let mut tick_time = std::mem::MaybeUninit::uninit();
            ffi::alsaseq_queue_status_get_tick_time(
                self.as_ref().to_glib_none().0,
                tick_time.as_mut_ptr(),
            );
            tick_time.assume_init()
        }
    }

    /// The number of available events in the queue.
    #[doc(alias = "event-count")]
    fn event_count(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "event-count")
    }

    /// The numeric ID of queue. An entry of ALSASeqSpecificQueueId is available as well.
    #[doc(alias = "queue-id")]
    fn queue_id(&self) -> u8 {
        ObjectExt::property(self.as_ref(), "queue-id")
    }

    /// Whether the queue is running or not.
    fn is_running(&self) -> bool {
        ObjectExt::property(self.as_ref(), "running")
    }

    #[doc(alias = "event-count")]
    fn connect_event_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_event_count_trampoline<
            P: IsA<QueueStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqQueueStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(QueueStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::event-count\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_event_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "queue-id")]
    fn connect_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_id_trampoline<
            P: IsA<QueueStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqQueueStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(QueueStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::queue-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_queue_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "running")]
    fn connect_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_running_trampoline<P: IsA<QueueStatus>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqQueueStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(QueueStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::running\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_running_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<QueueStatus>> QueueStatusExt for O {}
