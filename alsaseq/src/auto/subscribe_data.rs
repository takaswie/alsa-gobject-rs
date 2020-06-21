// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Addr;
use PortSubscribeFlag;

glib_wrapper! {
    pub struct SubscribeData(Object<alsaseq_sys::ALSASeqSubscribeData, alsaseq_sys::ALSASeqSubscribeDataClass, SubscribeDataClass>);

    match fn {
        get_type => || alsaseq_sys::alsaseq_subscribe_data_get_type(),
    }
}

impl SubscribeData {
    pub fn new() -> SubscribeData {
        unsafe {
            from_glib_full(alsaseq_sys::alsaseq_subscribe_data_new())
        }
    }
}

impl Default for SubscribeData {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SUBSCRIBE_DATA: Option<&SubscribeData> = None;

pub trait SubscribeDataExt: 'static {
    fn get_property_dest(&self) -> Option<Addr>;

    fn set_property_dest(&self, dest: Option<&Addr>);

    fn get_property_flag(&self) -> PortSubscribeFlag;

    fn set_property_flag(&self, flag: PortSubscribeFlag);

    fn get_property_queue_id(&self) -> u8;

    fn set_property_queue_id(&self, queue_id: u8);

    fn get_property_sender(&self) -> Option<Addr>;

    fn set_property_sender(&self, sender: Option<&Addr>);

    fn connect_property_dest_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sender_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SubscribeData>> SubscribeDataExt for O {
    fn get_property_dest(&self) -> Option<Addr> {
        unsafe {
            let mut value = Value::from_type(<Addr as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"dest\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `dest` getter")
        }
    }

    fn set_property_dest(&self, dest: Option<&Addr>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"dest\0".as_ptr() as *const _, Value::from(dest).to_glib_none().0);
        }
    }

    fn get_property_flag(&self) -> PortSubscribeFlag {
        unsafe {
            let mut value = Value::from_type(<PortSubscribeFlag as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"flag\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `flag` getter").unwrap()
        }
    }

    fn set_property_flag(&self, flag: PortSubscribeFlag) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"flag\0".as_ptr() as *const _, Value::from(&flag).to_glib_none().0);
        }
    }

    fn get_property_queue_id(&self) -> u8 {
        unsafe {
            let mut value = Value::from_type(<u8 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"queue-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `queue-id` getter").unwrap()
        }
    }

    fn set_property_queue_id(&self, queue_id: u8) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"queue-id\0".as_ptr() as *const _, Value::from(&queue_id).to_glib_none().0);
        }
    }

    fn get_property_sender(&self) -> Option<Addr> {
        unsafe {
            let mut value = Value::from_type(<Addr as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"sender\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `sender` getter")
        }
    }

    fn set_property_sender(&self, sender: Option<&Addr>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"sender\0".as_ptr() as *const _, Value::from(sender).to_glib_none().0);
        }
    }

    fn connect_property_dest_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dest_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqSubscribeData, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubscribeData>
        {
            let f: &F = &*(f as *const F);
            f(&SubscribeData::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::dest\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_dest_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_flag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flag_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqSubscribeData, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubscribeData>
        {
            let f: &F = &*(f as *const F);
            f(&SubscribeData::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::flag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_flag_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqSubscribeData, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubscribeData>
        {
            let f: &F = &*(f as *const F);
            f(&SubscribeData::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::queue-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_queue_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_sender_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sender_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqSubscribeData, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubscribeData>
        {
            let f: &F = &*(f as *const F);
            f(&SubscribeData::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::sender\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_sender_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for SubscribeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SubscribeData")
    }
}
