// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
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
use Addr;
use EventTimestampMode;
use PortAttrFlag;
use PortCapFlag;

glib_wrapper! {
    pub struct PortInfo(Object<alsaseq_sys::ALSASeqPortInfo, alsaseq_sys::ALSASeqPortInfoClass, PortInfoClass>);

    match fn {
        get_type => || alsaseq_sys::alsaseq_port_info_get_type(),
    }
}

impl PortInfo {
    pub fn new() -> PortInfo {
        unsafe {
            from_glib_full(alsaseq_sys::alsaseq_port_info_new())
        }
    }
}

impl Default for PortInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PORT_INFO: Option<&PortInfo> = None;

pub trait PortInfoExt: 'static {
    fn get_property_addr(&self) -> Option<Addr>;

    fn get_property_attrs(&self) -> PortAttrFlag;

    fn set_property_attrs(&self, attrs: PortAttrFlag);

    fn get_property_caps(&self) -> PortCapFlag;

    fn set_property_caps(&self, caps: PortCapFlag);

    fn get_property_midi_channels(&self) -> i32;

    fn set_property_midi_channels(&self, midi_channels: i32);

    fn get_property_midi_voices(&self) -> i32;

    fn set_property_midi_voices(&self, midi_voices: i32);

    fn get_property_name(&self) -> Option<GString>;

    fn set_property_name(&self, name: Option<&str>);

    fn get_property_queue_id(&self) -> u8;

    fn set_property_queue_id(&self, queue_id: u8);

    fn get_property_read_users(&self) -> i32;

    fn get_property_synth_voices(&self) -> i32;

    fn set_property_synth_voices(&self, synth_voices: i32);

    fn get_property_timestamp_mode(&self) -> EventTimestampMode;

    fn set_property_timestamp_mode(&self, timestamp_mode: EventTimestampMode);

    fn get_property_timestamp_overwrite(&self) -> bool;

    fn set_property_timestamp_overwrite(&self, timestamp_overwrite: bool);

    fn get_property_write_users(&self) -> i32;

    fn connect_property_attrs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_midi_channels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_midi_voices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_read_users_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_synth_voices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timestamp_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timestamp_overwrite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_write_users_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PortInfo>> PortInfoExt for O {
    fn get_property_addr(&self) -> Option<Addr> {
        unsafe {
            let mut value = Value::from_type(<Addr as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"addr\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `addr` getter")
        }
    }

    fn get_property_attrs(&self) -> PortAttrFlag {
        unsafe {
            let mut value = Value::from_type(<PortAttrFlag as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"attrs\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `attrs` getter").unwrap()
        }
    }

    fn set_property_attrs(&self, attrs: PortAttrFlag) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"attrs\0".as_ptr() as *const _, Value::from(&attrs).to_glib_none().0);
        }
    }

    fn get_property_caps(&self) -> PortCapFlag {
        unsafe {
            let mut value = Value::from_type(<PortCapFlag as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"caps\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `caps` getter").unwrap()
        }
    }

    fn set_property_caps(&self, caps: PortCapFlag) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"caps\0".as_ptr() as *const _, Value::from(&caps).to_glib_none().0);
        }
    }

    fn get_property_midi_channels(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"midi-channels\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `midi-channels` getter").unwrap()
        }
    }

    fn set_property_midi_channels(&self, midi_channels: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"midi-channels\0".as_ptr() as *const _, Value::from(&midi_channels).to_glib_none().0);
        }
    }

    fn get_property_midi_voices(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"midi-voices\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `midi-voices` getter").unwrap()
        }
    }

    fn set_property_midi_voices(&self, midi_voices: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"midi-voices\0".as_ptr() as *const _, Value::from(&midi_voices).to_glib_none().0);
        }
    }

    fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `name` getter")
        }
    }

    fn set_property_name(&self, name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"name\0".as_ptr() as *const _, Value::from(name).to_glib_none().0);
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

    fn get_property_read_users(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"read-users\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `read-users` getter").unwrap()
        }
    }

    fn get_property_synth_voices(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"synth-voices\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `synth-voices` getter").unwrap()
        }
    }

    fn set_property_synth_voices(&self, synth_voices: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"synth-voices\0".as_ptr() as *const _, Value::from(&synth_voices).to_glib_none().0);
        }
    }

    fn get_property_timestamp_mode(&self) -> EventTimestampMode {
        unsafe {
            let mut value = Value::from_type(<EventTimestampMode as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"timestamp-mode\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `timestamp-mode` getter").unwrap()
        }
    }

    fn set_property_timestamp_mode(&self, timestamp_mode: EventTimestampMode) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"timestamp-mode\0".as_ptr() as *const _, Value::from(&timestamp_mode).to_glib_none().0);
        }
    }

    fn get_property_timestamp_overwrite(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"timestamp-overwrite\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `timestamp-overwrite` getter").unwrap()
        }
    }

    fn set_property_timestamp_overwrite(&self, timestamp_overwrite: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"timestamp-overwrite\0".as_ptr() as *const _, Value::from(&timestamp_overwrite).to_glib_none().0);
        }
    }

    fn get_property_write_users(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"write-users\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `write-users` getter").unwrap()
        }
    }

    fn connect_property_attrs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_attrs_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::attrs\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_attrs_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_caps_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_midi_channels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_midi_channels_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::midi-channels\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_midi_channels_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_midi_voices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_midi_voices_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::midi-voices\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_midi_voices_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::queue-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_queue_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_read_users_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_read_users_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::read-users\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_read_users_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_synth_voices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_synth_voices_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::synth-voices\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_synth_voices_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_timestamp_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timestamp_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::timestamp-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_timestamp_mode_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_timestamp_overwrite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timestamp_overwrite_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::timestamp-overwrite\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_timestamp_overwrite_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_write_users_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_write_users_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqPortInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<PortInfo>
        {
            let f: &F = &*(f as *const F);
            f(&PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::write-users\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_write_users_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for PortInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PortInfo")
    }
}
