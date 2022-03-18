// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    #[doc(alias = "ALSATimerDeviceInfoFlag")]
    pub struct DeviceInfoFlag: u32 {
        #[doc(alias = "ALSATIMER_DEVICE_INFO_FLAG_SLAVE")]
        const SLAVE = ffi::ALSATIMER_DEVICE_INFO_FLAG_SLAVE as u32;
    }
}

impl fmt::Display for DeviceInfoFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for DeviceInfoFlag {
    type GlibType = ffi::ALSATimerDeviceInfoFlag;

    fn into_glib(self) -> ffi::ALSATimerDeviceInfoFlag {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSATimerDeviceInfoFlag> for DeviceInfoFlag {
    unsafe fn from_glib(value: ffi::ALSATimerDeviceInfoFlag) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for DeviceInfoFlag {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::alsatimer_device_info_flag_get_type()) }
    }
}

impl glib::value::ValueType for DeviceInfoFlag {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DeviceInfoFlag {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DeviceInfoFlag {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "ALSATimerInstanceParamFlag")]
    pub struct InstanceParamFlag: u32 {
        #[doc(alias = "ALSATIMER_INSTANCE_PARAM_FLAG_AUTO")]
        const AUTO = ffi::ALSATIMER_INSTANCE_PARAM_FLAG_AUTO as u32;
        #[doc(alias = "ALSATIMER_INSTANCE_PARAM_FLAG_EXCLUSIVE")]
        const EXCLUSIVE = ffi::ALSATIMER_INSTANCE_PARAM_FLAG_EXCLUSIVE as u32;
        #[doc(alias = "ALSATIMER_INSTANCE_PARAM_FLAG_EARLY_EVENT")]
        const EARLY_EVENT = ffi::ALSATIMER_INSTANCE_PARAM_FLAG_EARLY_EVENT as u32;
    }
}

impl fmt::Display for InstanceParamFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for InstanceParamFlag {
    type GlibType = ffi::ALSATimerInstanceParamFlag;

    fn into_glib(self) -> ffi::ALSATimerInstanceParamFlag {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSATimerInstanceParamFlag> for InstanceParamFlag {
    unsafe fn from_glib(value: ffi::ALSATimerInstanceParamFlag) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InstanceParamFlag {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::alsatimer_instance_param_flag_get_type()) }
    }
}

impl glib::value::ValueType for InstanceParamFlag {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InstanceParamFlag {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InstanceParamFlag {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
