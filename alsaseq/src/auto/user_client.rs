// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ClientInfo;
use crate::ClientPool;
use crate::EventCntr;
use crate::PortInfo;
use crate::QueueInfo;
use crate::QueueTempo;
use crate::QueueTimer;
use crate::RemoveFilter;
use crate::SubscribeData;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "ALSASeqUserClient")]
    pub struct UserClient(Object<ffi::ALSASeqUserClient, ffi::ALSASeqUserClientClass>);

    match fn {
        type_ => || ffi::alsaseq_user_client_get_type(),
    }
}

impl UserClient {
    pub const NONE: Option<&'static UserClient> = None;

    #[doc(alias = "alsaseq_user_client_new")]
    pub fn new() -> UserClient {
        unsafe { from_glib_full(ffi::alsaseq_user_client_new()) }
    }
}

impl Default for UserClient {
    fn default() -> Self {
        Self::new()
    }
}

pub trait UserClientExt: 'static {
    #[doc(alias = "alsaseq_user_client_create_source")]
    fn create_source(&self) -> Result<glib::Source, glib::Error>;

    #[doc(alias = "alsaseq_user_client_delete_port")]
    fn delete_port(&self, port_id: u8) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_delete_queue")]
    fn delete_queue(&self, queue_id: u8) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_get_queue_tempo")]
    #[doc(alias = "get_queue_tempo")]
    fn queue_tempo(&self, queue_id: u8) -> Result<QueueTempo, glib::Error>;

    #[doc(alias = "alsaseq_user_client_get_queue_timer")]
    #[doc(alias = "get_queue_timer")]
    fn queue_timer(&self, queue_id: u8) -> Result<QueueTimer, glib::Error>;

    #[doc(alias = "alsaseq_user_client_get_queue_usage")]
    #[doc(alias = "get_queue_usage")]
    fn queue_usage(&self, queue_id: u8) -> Result<bool, glib::Error>;

    #[doc(alias = "alsaseq_user_client_open")]
    fn open(&self, open_flag: i32) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_operate_subscription")]
    fn operate_subscription(
        &self,
        subs_data: &impl IsA<SubscribeData>,
        establish: bool,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_remove_events")]
    fn remove_events(&self, filter: &mut RemoveFilter) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_schedule_event")]
    fn schedule_event(
        &self,
        ev_cntr: &impl IsA<EventCntr>,
        count: usize,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_set_info")]
    fn set_info(&self, client_info: &impl IsA<ClientInfo>) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_set_pool")]
    fn set_pool(&self, client_pool: &impl IsA<ClientPool>) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_set_queue_tempo")]
    fn set_queue_tempo(
        &self,
        queue_id: u8,
        queue_tempo: &impl IsA<QueueTempo>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_set_queue_timer")]
    fn set_queue_timer(
        &self,
        queue_id: u8,
        queue_timer: &impl IsA<QueueTimer>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_set_queue_usage")]
    fn set_queue_usage(&self, queue_id: u8, use_: bool) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_update_port")]
    fn update_port(&self, port_info: &impl IsA<PortInfo>, port_id: u8) -> Result<(), glib::Error>;

    #[doc(alias = "alsaseq_user_client_update_queue")]
    fn update_queue(&self, queue_info: &impl IsA<QueueInfo>) -> Result<(), glib::Error>;

    #[doc(alias = "client-id")]
    fn client_id(&self) -> u8;

    #[doc(alias = "handle-event")]
    fn connect_handle_event<F: Fn(&Self, &EventCntr) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "client-id")]
    fn connect_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UserClient>> UserClientExt for O {
    fn create_source(&self) -> Result<glib::Source, glib::Error> {
        unsafe {
            let mut gsrc = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_create_source(
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
            let _ = ffi::alsaseq_user_client_delete_port(
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
            let _ = ffi::alsaseq_user_client_delete_queue(
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

    fn queue_tempo(&self, queue_id: u8) -> Result<QueueTempo, glib::Error> {
        unsafe {
            let mut queue_tempo = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_get_queue_tempo(
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

    fn queue_timer(&self, queue_id: u8) -> Result<QueueTimer, glib::Error> {
        unsafe {
            let mut queue_timer = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_get_queue_timer(
                self.as_ref().to_glib_none().0,
                queue_id,
                &mut queue_timer,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(queue_timer))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn queue_usage(&self, queue_id: u8) -> Result<bool, glib::Error> {
        unsafe {
            let mut use_ = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_get_queue_usage(
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
            let _ = ffi::alsaseq_user_client_open(
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

    fn operate_subscription(
        &self,
        subs_data: &impl IsA<SubscribeData>,
        establish: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_operate_subscription(
                self.as_ref().to_glib_none().0,
                subs_data.as_ref().to_glib_none().0,
                establish.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_events(&self, filter: &mut RemoveFilter) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_remove_events(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none_mut().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn schedule_event(
        &self,
        ev_cntr: &impl IsA<EventCntr>,
        count: usize,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_schedule_event(
                self.as_ref().to_glib_none().0,
                ev_cntr.as_ref().to_glib_none().0,
                count,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_info(&self, client_info: &impl IsA<ClientInfo>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_set_info(
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

    fn set_pool(&self, client_pool: &impl IsA<ClientPool>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_set_pool(
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

    fn set_queue_tempo(
        &self,
        queue_id: u8,
        queue_tempo: &impl IsA<QueueTempo>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_set_queue_tempo(
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

    fn set_queue_timer(
        &self,
        queue_id: u8,
        queue_timer: &impl IsA<QueueTimer>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_set_queue_timer(
                self.as_ref().to_glib_none().0,
                queue_id,
                queue_timer.as_ref().to_glib_none().0,
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
            let _ = ffi::alsaseq_user_client_set_queue_usage(
                self.as_ref().to_glib_none().0,
                queue_id,
                use_.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn update_port(&self, port_info: &impl IsA<PortInfo>, port_id: u8) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_update_port(
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

    fn update_queue(&self, queue_info: &impl IsA<QueueInfo>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::alsaseq_user_client_update_queue(
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

    fn client_id(&self) -> u8 {
        glib::ObjectExt::property(self.as_ref(), "client-id")
    }

    fn connect_handle_event<F: Fn(&Self, &EventCntr) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn handle_event_trampoline<
            P: IsA<UserClient>,
            F: Fn(&P, &EventCntr) + 'static,
        >(
            this: *mut ffi::ALSASeqUserClient,
            ev_cntr: *mut ffi::ALSASeqEventCntr,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                UserClient::from_glib_borrow(this).unsafe_cast_ref(),
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

    fn connect_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_id_trampoline<
            P: IsA<UserClient>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqUserClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UserClient::from_glib_borrow(this).unsafe_cast_ref())
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
        f.write_str("UserClient")
    }
}
