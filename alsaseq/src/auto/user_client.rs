// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
use glib;
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
use std::mem;
use std::mem::transmute;
use std::ptr;
use ClientInfo;
use ClientPool;
use Event;
use EventCntr;
use PortInfo;
use QueueInfo;
use QueueTempo;
use RemoveFilter;
use SubscribeData;

glib_wrapper! {
    pub struct UserClient(Object<alsaseq_sys::ALSASeqUserClient, alsaseq_sys::ALSASeqUserClientClass, UserClientClass>);

    match fn {
        get_type => || alsaseq_sys::alsaseq_user_client_get_type(),
    }
}

impl UserClient {
    pub fn new() -> UserClient {
        unsafe { from_glib_full(alsaseq_sys::alsaseq_user_client_new()) }
    }
}

impl Default for UserClient {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_USER_CLIENT: Option<&UserClient> = None;

pub trait UserClientExt: 'static {
    fn create_source(&self) -> Result<glib::Source, glib::Error>;

    fn delete_port(&self, port_id: u8) -> Result<(), glib::Error>;

    fn delete_queue(&self, queue_id: u8) -> Result<(), glib::Error>;

    fn get_queue_tempo(&self, queue_id: u8) -> Result<QueueTempo, glib::Error>;

    fn get_queue_usage(&self, queue_id: u8) -> Result<bool, glib::Error>;

    fn open(&self, open_flag: i32) -> Result<(), glib::Error>;

    fn operate_subscription<P: IsA<SubscribeData>>(
        &self,
        subs_data: &P,
        establish: bool,
    ) -> Result<(), glib::Error>;

    fn remove_events<P: IsA<RemoveFilter>>(&self, filter: &P) -> Result<(), glib::Error>;

    fn schedule_event(&self, event: &Event) -> Result<(), glib::Error>;

    fn set_info<P: IsA<ClientInfo>>(&self, client_info: &P) -> Result<(), glib::Error>;

    fn set_pool<P: IsA<ClientPool>>(&self, client_pool: &P) -> Result<(), glib::Error>;

    fn set_queue_tempo<P: IsA<QueueTempo>>(
        &self,
        queue_id: u8,
        queue_tempo: &P,
    ) -> Result<(), glib::Error>;

    fn set_queue_usage(&self, queue_id: u8, use_: bool) -> Result<(), glib::Error>;

    fn update_port<P: IsA<PortInfo>>(&self, port_info: &P, port_id: u8) -> Result<(), glib::Error>;

    fn update_queue<P: IsA<QueueInfo>>(&self, queue_info: &P) -> Result<(), glib::Error>;

    fn get_property_client_id(&self) -> u8;

    fn connect_handle_event<F: Fn(&Self, &EventCntr) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UserClient>> UserClientExt for O {
    fn create_source(&self) -> Result<glib::Source, glib::Error> {
        unsafe {
            let mut gsrc = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_create_source(
                self.as_ref().to_glib_none().0,
                &mut gsrc,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(gsrc))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn delete_port(&self, port_id: u8) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_delete_port(
                self.as_ref().to_glib_none().0,
                port_id,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn delete_queue(&self, queue_id: u8) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_delete_queue(
                self.as_ref().to_glib_none().0,
                queue_id,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_queue_tempo(&self, queue_id: u8) -> Result<QueueTempo, glib::Error> {
        unsafe {
            let mut queue_tempo = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_get_queue_tempo(
                self.as_ref().to_glib_none().0,
                queue_id,
                &mut queue_tempo,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(queue_tempo))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_queue_usage(&self, queue_id: u8) -> Result<bool, glib::Error> {
        unsafe {
            let mut use_ = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_get_queue_usage(
                self.as_ref().to_glib_none().0,
                queue_id,
                use_.as_mut_ptr(),
                &mut error,
            );
            let use_ = use_.assume_init();
            if error.is_null() {
                Ok(from_glib(use_))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn open(&self, open_flag: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_open(
                self.as_ref().to_glib_none().0,
                open_flag,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn operate_subscription<P: IsA<SubscribeData>>(
        &self,
        subs_data: &P,
        establish: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_operate_subscription(
                self.as_ref().to_glib_none().0,
                subs_data.as_ref().to_glib_none().0,
                establish.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_events<P: IsA<RemoveFilter>>(&self, filter: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_remove_events(
                self.as_ref().to_glib_none().0,
                filter.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn schedule_event(&self, event: &Event) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_schedule_event(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_info<P: IsA<ClientInfo>>(&self, client_info: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_set_info(
                self.as_ref().to_glib_none().0,
                client_info.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_pool<P: IsA<ClientPool>>(&self, client_pool: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_set_pool(
                self.as_ref().to_glib_none().0,
                client_pool.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_queue_tempo<P: IsA<QueueTempo>>(
        &self,
        queue_id: u8,
        queue_tempo: &P,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_set_queue_tempo(
                self.as_ref().to_glib_none().0,
                queue_id,
                queue_tempo.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_queue_usage(&self, queue_id: u8, use_: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_set_queue_usage(
                self.as_ref().to_glib_none().0,
                queue_id,
                use_.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn update_port<P: IsA<PortInfo>>(&self, port_info: &P, port_id: u8) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_update_port(
                self.as_ref().to_glib_none().0,
                port_info.as_ref().to_glib_none().0,
                port_id,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn update_queue<P: IsA<QueueInfo>>(&self, queue_info: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_user_client_update_queue(
                self.as_ref().to_glib_none().0,
                queue_info.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_property_client_id(&self) -> u8 {
        unsafe {
            let mut value = Value::from_type(<u8 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"client-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `client-id` getter")
                .unwrap()
        }
    }

    fn connect_handle_event<F: Fn(&Self, &EventCntr) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn handle_event_trampoline<P, F: Fn(&P, &EventCntr) + 'static>(
            this: *mut alsaseq_sys::ALSASeqUserClient,
            ev_cntr: *mut alsaseq_sys::ALSASeqEventCntr,
            f: glib_sys::gpointer,
        ) where
            P: IsA<UserClient>,
        {
            let f: &F = &*(f as *const F);
            f(
                &UserClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ev_cntr),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"handle-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    handle_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut alsaseq_sys::ALSASeqUserClient,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<UserClient>,
        {
            let f: &F = &*(f as *const F);
            f(&UserClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::client-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_client_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for UserClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserClient")
    }
}
