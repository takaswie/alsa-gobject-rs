// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DeviceInfoFlag;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    /// A GObject-derived object to express information of timer device.
    ///
    /// A [`DeviceInfo`][crate::DeviceInfo] is a GObject-derived object to express information of timer device.
    /// The call of alsatimer_get_device_info() returns an instance of the object according to the
    /// identifier of timer device.
    ///
    /// The object wraps `struct snd_timer_ginfo` in UAPI of Linux sound subsystem.
    ///
    /// # Implements
    ///
    /// [`DeviceInfoExt`][trait@crate::prelude::DeviceInfoExt]
    #[doc(alias = "ALSATimerDeviceInfo")]
    pub struct DeviceInfo(Object<ffi::ALSATimerDeviceInfo, ffi::ALSATimerDeviceInfoClass>);

    match fn {
        type_ => || ffi::alsatimer_device_info_get_type(),
    }
}

impl DeviceInfo {
    pub const NONE: Option<&'static DeviceInfo> = None;
}

/// Trait containing all [`struct@DeviceInfo`] methods.
///
/// # Implementors
///
/// [`DeviceInfo`][struct@crate::DeviceInfo]
pub trait DeviceInfoExt: 'static {
    /// The numeric ID of sound card.
    #[doc(alias = "card-id")]
    fn card_id(&self) -> i32;

    /// The flags of timer, one of [`DeviceInfoFlag`][crate::DeviceInfoFlag].
    fn flags(&self) -> DeviceInfoFlag;

    /// The string ID of timer.
    fn id(&self) -> Option<glib::GString>;

    /// The number of instances for the timer.
    #[doc(alias = "instance-count")]
    fn instance_count(&self) -> u32;

    /// The name of timer.
    fn name(&self) -> Option<glib::GString>;

    /// The resolution in nano seconds.
    fn resolution(&self) -> u64;

    /// The maximum resolution in nano seconds.
    #[doc(alias = "resolution-max")]
    fn resolution_max(&self) -> u64;

    /// The minimum resolution in nano seconds.
    #[doc(alias = "resolution-min")]
    fn resolution_min(&self) -> u64;

    #[doc(alias = "card-id")]
    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "flags")]
    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "instance-count")]
    fn connect_instance_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "resolution")]
    fn connect_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "resolution-max")]
    fn connect_resolution_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "resolution-min")]
    fn connect_resolution_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceInfo>> DeviceInfoExt for O {
    fn card_id(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "card-id")
    }

    fn flags(&self) -> DeviceInfoFlag {
        glib::ObjectExt::property(self.as_ref(), "flags")
    }

    fn id(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "id")
    }

    fn instance_count(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "instance-count")
    }

    fn name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "name")
    }

    fn resolution(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "resolution")
    }

    fn resolution_max(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "resolution-max")
    }

    fn resolution_min(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "resolution-min")
    }

    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<P: IsA<DeviceInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::card-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_card_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P: IsA<DeviceInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<DeviceInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_instance_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_instance_count_trampoline<
            P: IsA<DeviceInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::instance-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_instance_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<DeviceInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_trampoline<
            P: IsA<DeviceInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_resolution_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_max_trampoline<
            P: IsA<DeviceInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resolution-max\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resolution_max_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_resolution_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_min_trampoline<
            P: IsA<DeviceInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resolution-min\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resolution_min_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceInfo")
    }
}
