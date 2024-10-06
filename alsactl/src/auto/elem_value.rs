// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ElemId};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    /// A GObject-derived object to express the container of array for values specific to element type.
    ///
    /// A [`ElemValue`][crate::ElemValue] includes several types of array for values specific to element type. The
    /// arrays shares the same internal storage, thus the user application should decide the type of
    /// array according to the type of element when accessing to the array. The object is used for the
    /// call of [`CardExt::write_elem_value()`][crate::prelude::CardExt::write_elem_value()] and [`CardExtManual::read_elem_value()`][crate::prelude::CardExtManual::read_elem_value()].
    ///
    /// The object wraps `struct snd_ctl_elem_value` in UAPI of Linux sound subsystem.
    ///
    /// ## Properties
    ///
    ///
    /// #### `elem-id`
    ///  The identifier of element.
    ///
    /// Readable
    ///
    /// # Implements
    ///
    /// [`ElemValueExt`][trait@crate::prelude::ElemValueExt], [`ElemValueExtManual`][trait@crate::prelude::ElemValueExtManual]
    #[doc(alias = "ALSACtlElemValue")]
    pub struct ElemValue(Object<ffi::ALSACtlElemValue, ffi::ALSACtlElemValueClass>);

    match fn {
        type_ => || ffi::alsactl_elem_value_get_type(),
    }
}

impl ElemValue {
    pub const NONE: Option<&'static ElemValue> = None;

    /// Allocate and return an instance of [`ElemValue`][crate::ElemValue].
    ///
    /// # Returns
    ///
    /// An instance of [`ElemValue`][crate::ElemValue].
    #[doc(alias = "alsactl_elem_value_new")]
    pub fn new() -> ElemValue {
        unsafe { from_glib_full(ffi::alsactl_elem_value_new()) }
    }
}

impl Default for ElemValue {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ElemValue>> Sealed for T {}
}

/// Trait containing the part of [`struct@ElemValue`] methods.
///
/// # Implementors
///
/// [`ElemValue`][struct@crate::ElemValue]
pub trait ElemValueExt: IsA<ElemValue> + sealed::Sealed + 'static {
    #[doc(alias = "alsactl_elem_value_equal")]
    fn equal(&self, target: &impl IsA<ElemValue>) -> bool {
        unsafe {
            from_glib(ffi::alsactl_elem_value_equal(
                const_override(self.as_ref().to_glib_none().0),
                target.as_ref().to_glib_none().0,
            ))
        }
    }

    /// Copy the array into internal storage for [`ElemType`][crate::ElemType].BYTES element.
    /// ## `values`
    /// The array for 8 bit unsigned integer values.
    #[doc(alias = "alsactl_elem_value_set_bytes")]
    fn set_bytes(&self, values: &[u8]) {
        let value_count = values.len() as _;
        unsafe {
            ffi::alsactl_elem_value_set_bytes(
                self.as_ref().to_glib_none().0,
                values.to_glib_none().0,
                value_count,
            );
        }
    }

    /// Copy the array into internal storage for [`ElemType`][crate::ElemType].ENUMERATED element.
    /// ## `values`
    /// The array for enumeration index values.
    #[doc(alias = "alsactl_elem_value_set_enum")]
    fn set_enum(&self, values: &[u32]) {
        let value_count = values.len() as _;
        unsafe {
            ffi::alsactl_elem_value_set_enum(
                self.as_ref().to_glib_none().0,
                values.to_glib_none().0,
                value_count,
            );
        }
    }

    /// Copy the channel status bits into internal storage for [`ElemType`][crate::ElemType].IEC60958 element.
    /// ## `status`
    /// The array of byte data for channel status bits of IEC 60958.
    #[doc(alias = "alsactl_elem_value_set_iec60958_channel_status")]
    fn set_iec60958_channel_status(&self, status: &[u8]) {
        let length = status.len() as _;
        unsafe {
            ffi::alsactl_elem_value_set_iec60958_channel_status(
                self.as_ref().to_glib_none().0,
                status.to_glib_none().0,
                length,
            );
        }
    }

    /// Copy the user data bits into internal storage for [`ElemType`][crate::ElemType].IEC60958 element.
    /// ## `data`
    /// The array of byte data for user data bits of IEC 60958.
    #[doc(alias = "alsactl_elem_value_set_iec60958_user_data")]
    fn set_iec60958_user_data(&self, data: &[u8]) {
        let length = data.len() as _;
        unsafe {
            ffi::alsactl_elem_value_set_iec60958_user_data(
                self.as_ref().to_glib_none().0,
                data.to_glib_none().0,
                length,
            );
        }
    }

    /// Copy the array into internal storage for [`ElemType`][crate::ElemType].INTEGER element.
    /// ## `values`
    /// The array for 32 bit signed integer values.
    #[doc(alias = "alsactl_elem_value_set_int")]
    fn set_int(&self, values: &[i32]) {
        let value_count = values.len() as _;
        unsafe {
            ffi::alsactl_elem_value_set_int(
                self.as_ref().to_glib_none().0,
                values.to_glib_none().0,
                value_count,
            );
        }
    }

    /// Copy the array into internal storage for [`ElemType`][crate::ElemType].INTEGER64 element.
    /// ## `values`
    /// The array for 64 bit signed integer values.
    #[doc(alias = "alsactl_elem_value_set_int64")]
    fn set_int64(&self, values: &[i64]) {
        let value_count = values.len() as _;
        unsafe {
            ffi::alsactl_elem_value_set_int64(
                self.as_ref().to_glib_none().0,
                values.to_glib_none().0,
                value_count,
            );
        }
    }

    /// The identifier of element.
    #[doc(alias = "elem-id")]
    fn elem_id(&self) -> Option<ElemId> {
        ObjectExt::property(self.as_ref(), "elem-id")
    }

    #[doc(alias = "elem-id")]
    fn connect_elem_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_elem_id_trampoline<P: IsA<ElemValue>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSACtlElemValue,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ElemValue::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::elem-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_elem_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ElemValue>> ElemValueExt for O {}
