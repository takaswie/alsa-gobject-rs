// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::InstanceParamFlag;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    /// A GObject-derived object to express parameters of user instance.
    ///
    /// A [`InstanceParams`][crate::InstanceParams] is a GObject-derived object to express parameters of user instance
    /// attached to any timer device or the other instance as slave. The call of
    /// [`UserInstanceExtManual::set_params()`][crate::prelude::UserInstanceExtManual::set_params()] requires the instance of object.
    ///
    /// The object wraps `struct snd_timer_params` in UAPI of Linux sound subsystem.
    ///
    /// # Implements
    ///
    /// [`InstanceParamsExt`][trait@crate::prelude::InstanceParamsExt], [`InstanceParamsExtManual`][trait@crate::prelude::InstanceParamsExtManual]
    #[doc(alias = "ALSATimerInstanceParams")]
    pub struct InstanceParams(Object<ffi::ALSATimerInstanceParams, ffi::ALSATimerInstanceParamsClass>);

    match fn {
        type_ => || ffi::alsatimer_instance_params_get_type(),
    }
}

impl InstanceParams {
    pub const NONE: Option<&'static InstanceParams> = None;

    /// Allocate and return an instance of [`InstanceParams`][crate::InstanceParams].
    ///
    /// # Returns
    ///
    /// An instance of [`InstanceParams`][crate::InstanceParams].
    #[doc(alias = "alsatimer_instance_params_new")]
    pub fn new() -> InstanceParams {
        unsafe { from_glib_full(ffi::alsatimer_instance_params_new()) }
    }
}

impl Default for InstanceParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing the part of [`struct@InstanceParams`] methods.
///
/// # Implementors
///
/// [`InstanceParams`][struct@crate::InstanceParams]
pub trait InstanceParamsExt: 'static {
    /// The flags for user instance, as a set of [`InstanceParamFlag`][crate::InstanceParamFlag].
    fn flags(&self) -> InstanceParamFlag;

    /// The flags for user instance, as a set of [`InstanceParamFlag`][crate::InstanceParamFlag].
    fn set_flags(&self, flags: InstanceParamFlag);

    /// The interval to generate event in tick count.
    fn interval(&self) -> u32;

    /// The interval to generate event in tick count.
    fn set_interval(&self, interval: u32);

    /// The size of queue.
    #[doc(alias = "queue-size")]
    fn queue_size(&self) -> u32;

    /// The size of queue.
    #[doc(alias = "queue-size")]
    fn set_queue_size(&self, queue_size: u32);

    #[doc(alias = "flags")]
    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "interval")]
    fn connect_interval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "queue-size")]
    fn connect_queue_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<InstanceParams>> InstanceParamsExt for O {
    fn flags(&self) -> InstanceParamFlag {
        glib::ObjectExt::property(self.as_ref(), "flags")
    }

    fn set_flags(&self, flags: InstanceParamFlag) {
        glib::ObjectExt::set_property(self.as_ref(), "flags", &flags)
    }

    fn interval(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "interval")
    }

    fn set_interval(&self, interval: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "interval", &interval)
    }

    fn queue_size(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "queue-size")
    }

    fn set_queue_size(&self, queue_size: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "queue-size", &queue_size)
    }

    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<
            P: IsA<InstanceParams>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerInstanceParams,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InstanceParams::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_interval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interval_trampoline<
            P: IsA<InstanceParams>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerInstanceParams,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InstanceParams::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::interval\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interval_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_queue_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_size_trampoline<
            P: IsA<InstanceParams>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerInstanceParams,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InstanceParams::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::queue-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_queue_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for InstanceParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InstanceParams")
    }
}
