// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::IfaceType;
use glib::object::IsA;
use std::fmt;

glib::wrapper! {
    /// A GObject-derived object to express information of ALSA hwdep device.
    ///
    /// A [`DeviceInfo`][crate::DeviceInfo] is a GObject-derived object to express information of ALSA hwdep device.
    /// The call of [`DeviceCommonExt::device_info()`][crate::prelude::DeviceCommonExt::device_info()] returns an instance of the object.
    ///
    /// The object wraps `struct snd_hwdep_info` in UAPI of Linux sound subsystem.
    ///
    /// # Implements
    ///
    /// [`DeviceInfoExt`][trait@crate::prelude::DeviceInfoExt]
    #[doc(alias = "ALSAHwdepDeviceInfo")]
    pub struct DeviceInfo(Object<ffi::ALSAHwdepDeviceInfo, ffi::ALSAHwdepDeviceInfoClass>);

    match fn {
        type_ => || ffi::alsahwdep_device_info_get_type(),
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

    /// The numeric ID of device.
    #[doc(alias = "device-id")]
    fn device_id(&self) -> u32;

    /// The string ID of the hwdep device.
    fn id(&self) -> Option<glib::GString>;

    /// The type of interface for the hwdep device, one of ALSAHwdepIfaceType.
    fn iface(&self) -> IfaceType;

    /// The name of the hwdep device.
    fn name(&self) -> Option<glib::GString>;
}

impl<O: IsA<DeviceInfo>> DeviceInfoExt for O {
    fn card_id(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "card-id")
    }

    fn device_id(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "device-id")
    }

    fn id(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "id")
    }

    fn iface(&self) -> IfaceType {
        glib::ObjectExt::property(self.as_ref(), "iface")
    }

    fn name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "name")
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceInfo")
    }
}
