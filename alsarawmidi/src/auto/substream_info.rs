// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{StreamDirection, StreamPairInfoFlag};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    /// A GObject-derived object to express information of substream.
    ///
    /// A [`SubstreamInfo`][crate::SubstreamInfo] is a GObject-derived object to express information of substream
    /// attached to the pair of streams. The call of [`StreamPairExt::substream_info()`][crate::prelude::StreamPairExt::substream_info()] or
    /// [`StreamPairExt::substream_info()`][crate::prelude::StreamPairExt::substream_info()] return the instance of object.
    ///
    /// The object wraps `struct snd_rawmidi_info` in UAPI of Linux sound subsystem.
    ///
    /// ## Properties
    ///
    ///
    /// #### `card-id`
    ///  The numeric identifier of sound card.
    ///
    /// Readable
    ///
    ///
    /// #### `device-id`
    ///  The numeric identifier of rawmidi device.
    ///
    /// Readable
    ///
    ///
    /// #### `direction`
    ///  The direction of stream, one of [`StreamDirection`][crate::StreamDirection].
    ///
    /// Readable
    ///
    ///
    /// #### `flags`
    ///  The information flags of rawmidi device with flags of [`StreamPairInfoFlag`][crate::StreamPairInfoFlag].
    ///
    /// Readable
    ///
    ///
    /// #### `id`
    ///  The string identifier of rawmidi device.
    ///
    /// Readable
    ///
    ///
    /// #### `name`
    ///  The name of rawmidi device.
    ///
    /// Readable
    ///
    ///
    /// #### `subdevice-id`
    ///  The numeric identifier of subdevice or rawmidi device.
    ///
    /// Readable
    ///
    ///
    /// #### `subdevice-name`
    ///  The name of subdevice for the direction and the [`subdevice-id`][struct@crate::SubstreamInfo#subdevice-id].
    ///
    /// Readable
    ///
    ///
    /// #### `subdevices-avail`
    ///  The current number of available subdevices on the rawmidi device for the direction.
    ///
    /// Readable
    ///
    ///
    /// #### `subdevices-count`
    ///  Readable
    ///
    /// # Implements
    ///
    /// [`SubstreamInfoExt`][trait@crate::prelude::SubstreamInfoExt]
    #[doc(alias = "ALSARawmidiSubstreamInfo")]
    pub struct SubstreamInfo(Object<ffi::ALSARawmidiSubstreamInfo, ffi::ALSARawmidiSubstreamInfoClass>);

    match fn {
        type_ => || ffi::alsarawmidi_substream_info_get_type(),
    }
}

impl SubstreamInfo {
    pub const NONE: Option<&'static SubstreamInfo> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::SubstreamInfo>> Sealed for T {}
}

/// Trait containing all [`struct@SubstreamInfo`] methods.
///
/// # Implementors
///
/// [`SubstreamInfo`][struct@crate::SubstreamInfo]
pub trait SubstreamInfoExt: IsA<SubstreamInfo> + sealed::Sealed + 'static {
    /// The numeric identifier of sound card.
    #[doc(alias = "card-id")]
    fn card_id(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "card-id")
    }

    /// The numeric identifier of rawmidi device.
    #[doc(alias = "device-id")]
    fn device_id(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "device-id")
    }

    /// The direction of stream, one of [`StreamDirection`][crate::StreamDirection].
    fn direction(&self) -> StreamDirection {
        ObjectExt::property(self.as_ref(), "direction")
    }

    /// The information flags of rawmidi device with flags of [`StreamPairInfoFlag`][crate::StreamPairInfoFlag].
    fn flags(&self) -> StreamPairInfoFlag {
        ObjectExt::property(self.as_ref(), "flags")
    }

    /// The string identifier of rawmidi device.
    fn id(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "id")
    }

    /// The name of rawmidi device.
    fn name(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "name")
    }

    /// The numeric identifier of subdevice or rawmidi device.
    #[doc(alias = "subdevice-id")]
    fn subdevice_id(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "subdevice-id")
    }

    /// The name of subdevice for the direction and the [`subdevice-id`][struct@crate::SubstreamInfo#subdevice-id].
    #[doc(alias = "subdevice-name")]
    fn subdevice_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "subdevice-name")
    }

    /// The current number of available subdevices on the rawmidi device for the direction.
    #[doc(alias = "subdevices-avail")]
    fn subdevices_avail(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "subdevices-avail")
    }

    #[doc(alias = "subdevices-count")]
    fn subdevices_count(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "subdevices-count")
    }

    #[doc(alias = "card-id")]
    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<
            P: IsA<SubstreamInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::card-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_card_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "device-id")]
    fn connect_device_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_device_id_trampoline<
            P: IsA<SubstreamInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::device-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_device_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "direction")]
    fn connect_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<
            P: IsA<SubstreamInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "flags")]
    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P: IsA<SubstreamInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<SubstreamInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<SubstreamInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "subdevice-id")]
    fn connect_subdevice_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subdevice_id_trampoline<
            P: IsA<SubstreamInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subdevice-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_subdevice_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "subdevice-name")]
    fn connect_subdevice_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subdevice_name_trampoline<
            P: IsA<SubstreamInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subdevice-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_subdevice_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "subdevices-avail")]
    fn connect_subdevices_avail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subdevices_avail_trampoline<
            P: IsA<SubstreamInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subdevices-avail\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_subdevices_avail_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "subdevices-count")]
    fn connect_subdevices_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subdevices_count_trampoline<
            P: IsA<SubstreamInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subdevices-count\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_subdevices_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<SubstreamInfo>> SubstreamInfoExt for O {}
