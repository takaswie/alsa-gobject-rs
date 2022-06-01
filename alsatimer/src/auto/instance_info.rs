// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsatimer_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use DeviceInfoFlag;

glib_wrapper! {
    pub struct InstanceInfo(Object<alsatimer_sys::ALSATimerInstanceInfo, alsatimer_sys::ALSATimerInstanceInfoClass, InstanceInfoClass>);

    match fn {
        get_type => || alsatimer_sys::alsatimer_instance_info_get_type(),
    }
}

pub const NONE_INSTANCE_INFO: Option<&InstanceInfo> = None;

pub trait InstanceInfoExt: 'static {
    fn get_property_card_id(&self) -> i32;

    fn get_property_flags(&self) -> DeviceInfoFlag;

    fn get_property_id(&self) -> Option<GString>;

    fn get_property_name(&self) -> Option<GString>;

    fn get_property_resolution(&self) -> u64;

    fn connect_property_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<InstanceInfo>> InstanceInfoExt for O {
    fn get_property_card_id(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"card-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `card-id` getter").unwrap()
        }
    }

    fn get_property_flags(&self) -> DeviceInfoFlag {
        unsafe {
            let mut value = Value::from_type(<DeviceInfoFlag as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"flags\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `flags` getter").unwrap()
        }
    }

    fn get_property_id(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `id` getter")
        }
    }

    fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `name` getter")
        }
    }

    fn get_property_resolution(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"resolution\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `resolution` getter").unwrap()
        }
    }

    fn connect_property_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerInstanceInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<InstanceInfo>
        {
            let f: &F = &*(f as *const F);
            f(&InstanceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::card-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_card_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerInstanceInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<InstanceInfo>
        {
            let f: &F = &*(f as *const F);
            f(&InstanceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_flags_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerInstanceInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<InstanceInfo>
        {
            let f: &F = &*(f as *const F);
            f(&InstanceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerInstanceInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<InstanceInfo>
        {
            let f: &F = &*(f as *const F);
            f(&InstanceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerInstanceInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<InstanceInfo>
        {
            let f: &F = &*(f as *const F);
            f(&InstanceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::resolution\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_resolution_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for InstanceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InstanceInfo")
    }
}
