// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    /// A GObject-derived object to express status of timer device.
    ///
    /// A [`DeviceStatus`][crate::DeviceStatus] is a GObject-derived object to express status of timer device. The
    /// call of [`device_status()`][crate::device_status()] returns the instance of object.
    ///
    /// The object wraps 'struct snd_timer_gstatus' in UAPI of Linux sound subsystem.
    ///
    /// ## Properties
    ///
    ///
    /// #### `resolution`
    ///  The current resolution in nano seconds.
    ///
    /// Readable
    ///
    ///
    /// #### `resolution-denominator`
    ///  The denominator of current resolution in seconds.
    ///
    /// Readable
    ///
    ///
    /// #### `resolution-numerator`
    ///  The numerator of current resolution in seconds.
    ///
    /// Readable
    ///
    /// # Implements
    ///
    /// [`DeviceStatusExt`][trait@crate::prelude::DeviceStatusExt]
    #[doc(alias = "ALSATimerDeviceStatus")]
    pub struct DeviceStatus(Object<ffi::ALSATimerDeviceStatus, ffi::ALSATimerDeviceStatusClass>);

    match fn {
        type_ => || ffi::alsatimer_device_status_get_type(),
    }
}

impl DeviceStatus {
    pub const NONE: Option<&'static DeviceStatus> = None;

    /// Allocate and return an instance of [`DeviceStatus`][crate::DeviceStatus].
    ///
    /// # Returns
    ///
    /// An instance of [`DeviceStatus`][crate::DeviceStatus].
    #[doc(alias = "alsatimer_device_status_new")]
    pub fn new() -> DeviceStatus {
        unsafe { from_glib_full(ffi::alsatimer_device_status_new()) }
    }
}

impl Default for DeviceStatus {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DeviceStatus>> Sealed for T {}
}

/// Trait containing all [`struct@DeviceStatus`] methods.
///
/// # Implementors
///
/// [`DeviceStatus`][struct@crate::DeviceStatus]
pub trait DeviceStatusExt: IsA<DeviceStatus> + sealed::Sealed + 'static {
    /// The current resolution in nano seconds.
    fn resolution(&self) -> u64 {
        ObjectExt::property(self.as_ref(), "resolution")
    }

    /// The denominator of current resolution in seconds.
    #[doc(alias = "resolution-denominator")]
    fn resolution_denominator(&self) -> u64 {
        ObjectExt::property(self.as_ref(), "resolution-denominator")
    }

    /// The numerator of current resolution in seconds.
    #[doc(alias = "resolution-numerator")]
    fn resolution_numerator(&self) -> u64 {
        ObjectExt::property(self.as_ref(), "resolution-numerator")
    }

    #[doc(alias = "resolution")]
    fn connect_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_trampoline<
            P: IsA<DeviceStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resolution\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resolution_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resolution-denominator")]
    fn connect_resolution_denominator_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_denominator_trampoline<
            P: IsA<DeviceStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resolution-denominator\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resolution_denominator_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resolution-numerator")]
    fn connect_resolution_numerator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_numerator_trampoline<
            P: IsA<DeviceStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resolution-numerator\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resolution_numerator_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DeviceStatus>> DeviceStatusExt for O {}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceStatus")
    }
}
