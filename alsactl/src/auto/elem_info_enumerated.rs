// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ElemInfoCommon;
use crate::ElemInfoSingleArray;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    /// An object to express information for enumerated type of element.
    ///
    /// A `GObject::Object` derived object class for enumerated type of element.
    ///
    /// The object wraps `struct snd_ctl_elem_info` in UAPI of Linux sound subsystem.
    ///
    /// # Implements
    ///
    /// [`ElemInfoEnumeratedExt`][trait@crate::prelude::ElemInfoEnumeratedExt], [`ElemInfoCommonExt`][trait@crate::prelude::ElemInfoCommonExt], [`ElemInfoSingleArrayExt`][trait@crate::prelude::ElemInfoSingleArrayExt]
    #[doc(alias = "ALSACtlElemInfoEnumerated")]
    pub struct ElemInfoEnumerated(Object<ffi::ALSACtlElemInfoEnumerated, ffi::ALSACtlElemInfoEnumeratedClass>) @implements ElemInfoCommon, ElemInfoSingleArray;

    match fn {
        type_ => || ffi::alsactl_elem_info_enumerated_get_type(),
    }
}

impl ElemInfoEnumerated {
    pub const NONE: Option<&'static ElemInfoEnumerated> = None;

    /// Allocate and return an instance of [`ElemInfoEnumerated`][crate::ElemInfoEnumerated].
    ///
    /// # Returns
    ///
    /// An instance of [`ElemInfoEnumerated`][crate::ElemInfoEnumerated].
    #[doc(alias = "alsactl_elem_info_enumerated_new")]
    pub fn new() -> ElemInfoEnumerated {
        unsafe { from_glib_full(ffi::alsactl_elem_info_enumerated_new()) }
    }
}

impl Default for ElemInfoEnumerated {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all [`struct@ElemInfoEnumerated`] methods.
///
/// # Implementors
///
/// [`ElemInfoEnumerated`][struct@crate::ElemInfoEnumerated]
pub trait ElemInfoEnumeratedExt: 'static {
    /// The list of indexed labels for the element. There is limitation that:
    ///
    ///  - The length of label including terminator should be within 64 bytes.
    ///  - The total length of labels including terminators should be within (64 * 1024) bytes.
    fn labels(&self) -> Vec<glib::GString>;

    /// The list of indexed labels for the element. There is limitation that:
    ///
    ///  - The length of label including terminator should be within 64 bytes.
    ///  - The total length of labels including terminators should be within (64 * 1024) bytes.
    fn set_labels(&self, labels: &[&str]);

    #[doc(alias = "labels")]
    fn connect_labels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ElemInfoEnumerated>> ElemInfoEnumeratedExt for O {
    fn labels(&self) -> Vec<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "labels")
    }

    fn set_labels(&self, labels: &[&str]) {
        glib::ObjectExt::set_property(self.as_ref(), "labels", &labels)
    }

    fn connect_labels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_labels_trampoline<
            P: IsA<ElemInfoEnumerated>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSACtlElemInfoEnumerated,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ElemInfoEnumerated::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::labels\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_labels_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ElemInfoEnumerated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ElemInfoEnumerated")
    }
}
