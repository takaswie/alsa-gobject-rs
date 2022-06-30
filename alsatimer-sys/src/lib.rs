// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]

extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate libc;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type ALSATimerClass = c_int;
pub const ALSATIMER_CLASS_NONE: ALSATimerClass = -1;
pub const ALSATIMER_CLASS_GLOBAL: ALSATimerClass = 1;
pub const ALSATIMER_CLASS_CARD: ALSATimerClass = 2;
pub const ALSATIMER_CLASS_PCM: ALSATimerClass = 3;

pub type ALSATimerEventType = c_int;
pub const ALSATIMER_EVENT_TYPE_TICK_TIME: ALSATimerEventType = 0;
pub const ALSATIMER_EVENT_TYPE_REAL_TIME: ALSATimerEventType = 1;

pub type ALSATimerRealTimeEventType = c_int;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_RESOLUTION: ALSATimerRealTimeEventType = 0;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_TICK: ALSATimerRealTimeEventType = 1;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_START: ALSATimerRealTimeEventType = 2;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_STOP: ALSATimerRealTimeEventType = 3;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_CONTINUE: ALSATimerRealTimeEventType = 4;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_PAUSE: ALSATimerRealTimeEventType = 5;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_EARLY: ALSATimerRealTimeEventType = 6;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_SUSPEND: ALSATimerRealTimeEventType = 7;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_RESUME: ALSATimerRealTimeEventType = 8;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_MSTART: ALSATimerRealTimeEventType = 12;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_MSTOP: ALSATimerRealTimeEventType = 13;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_MCONTINUE: ALSATimerRealTimeEventType = 14;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_MPAUSE: ALSATimerRealTimeEventType = 15;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_MSUSPEND: ALSATimerRealTimeEventType = 17;
pub const ALSATIMER_REAL_TIME_EVENT_TYPE_MRESUME: ALSATimerRealTimeEventType = 18;

pub type ALSATimerSlaveClass = c_int;
pub const ALSATIMER_SLAVE_CLASS_NONE: ALSATimerSlaveClass = 0;
pub const ALSATIMER_SLAVE_CLASS_APPLICATION: ALSATimerSlaveClass = 1;
pub const ALSATIMER_SLAVE_CLASS_SEQUENCER: ALSATimerSlaveClass = 2;

pub type ALSATimerSpecificGlobalDevice = c_int;
pub const ALSATIMER_SPECIFIC_GLOBAL_DEVICE_SYSTEM: ALSATimerSpecificGlobalDevice = 0;
pub const ALSATIMER_SPECIFIC_GLOBAL_DEVICE_HRTIMER: ALSATimerSpecificGlobalDevice = 3;

pub type ALSATimerUserInstanceError = c_int;
pub const ALSATIMER_USER_INSTANCE_ERROR_FAILED: ALSATimerUserInstanceError = 0;
pub const ALSATIMER_USER_INSTANCE_ERROR_TIMER_NOT_FOUND: ALSATimerUserInstanceError = 1;
pub const ALSATIMER_USER_INSTANCE_ERROR_NOT_ATTACHED: ALSATimerUserInstanceError = 2;
pub const ALSATIMER_USER_INSTANCE_ERROR_ATTACHED: ALSATimerUserInstanceError = 3;

// Flags
pub type ALSATimerDeviceInfoFlag = c_uint;
pub const ALSATIMER_DEVICE_INFO_FLAG_SLAVE: ALSATimerDeviceInfoFlag = 1;

pub type ALSATimerInstanceParamFlag = c_uint;
pub const ALSATIMER_INSTANCE_PARAM_FLAG_AUTO: ALSATimerInstanceParamFlag = 1;
pub const ALSATIMER_INSTANCE_PARAM_FLAG_EXCLUSIVE: ALSATimerInstanceParamFlag = 2;
pub const ALSATIMER_INSTANCE_PARAM_FLAG_EARLY_EVENT: ALSATimerInstanceParamFlag = 4;

// Records
#[repr(C)]
pub struct ALSATimerDeviceId(c_void);

impl ::std::fmt::Debug for ALSATimerDeviceId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerDeviceId @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerDeviceInfoClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSATimerDeviceInfoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSATimerDeviceInfoClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerDeviceParamsClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSATimerDeviceParamsClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSATimerDeviceParamsClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerDeviceStatusClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSATimerDeviceStatusClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSATimerDeviceStatusClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerInstanceInfoClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSATimerInstanceInfoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSATimerInstanceInfoClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerInstanceParamsClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSATimerInstanceParamsClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSATimerInstanceParamsClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerInstanceStatusClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for ALSATimerInstanceStatusClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSATimerInstanceStatusClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .finish()
    }
}

#[repr(C)]
pub struct ALSATimerRealTimeEvent(c_void);

impl ::std::fmt::Debug for ALSATimerRealTimeEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerRealTimeEvent @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct ALSATimerTickTimeEvent(c_void);

impl ::std::fmt::Debug for ALSATimerTickTimeEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerTickTimeEvent @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerUserInstanceClass {
    pub parent_class: gobject::GObjectClass,
    pub handle_tick_time_event:
        Option<unsafe extern "C" fn(*mut ALSATimerUserInstance, *const ALSATimerTickTimeEvent)>,
    pub handle_real_time_event:
        Option<unsafe extern "C" fn(*mut ALSATimerUserInstance, *const ALSATimerRealTimeEvent)>,
    pub handle_disconnection: Option<unsafe extern "C" fn(*mut ALSATimerUserInstance)>,
}

impl ::std::fmt::Debug for ALSATimerUserInstanceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "ALSATimerUserInstanceClass @ {:?}",
            self as *const _
        ))
        .field("parent_class", &self.parent_class)
        .field("handle_tick_time_event", &self.handle_tick_time_event)
        .field("handle_real_time_event", &self.handle_real_time_event)
        .field("handle_disconnection", &self.handle_disconnection)
        .finish()
    }
}

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerDeviceInfo {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for ALSATimerDeviceInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerDeviceInfo @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerDeviceParams {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for ALSATimerDeviceParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerDeviceParams @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerDeviceStatus {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for ALSATimerDeviceStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerDeviceStatus @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerInstanceInfo {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for ALSATimerInstanceInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerInstanceInfo @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerInstanceParams {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for ALSATimerInstanceParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerInstanceParams @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerInstanceStatus {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for ALSATimerInstanceStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerInstanceStatus @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALSATimerUserInstance {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for ALSATimerUserInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ALSATimerUserInstance @ {:?}", self as *const _))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

extern "C" {

    //=========================================================================
    // ALSATimerClass
    //=========================================================================
    pub fn alsatimer_class_get_type() -> GType;

    //=========================================================================
    // ALSATimerEventType
    //=========================================================================
    pub fn alsatimer_event_type_get_type() -> GType;

    //=========================================================================
    // ALSATimerRealTimeEventType
    //=========================================================================
    pub fn alsatimer_real_time_event_type_get_type() -> GType;

    //=========================================================================
    // ALSATimerSlaveClass
    //=========================================================================
    pub fn alsatimer_slave_class_get_type() -> GType;

    //=========================================================================
    // ALSATimerSpecificGlobalDevice
    //=========================================================================
    pub fn alsatimer_specific_global_device_get_type() -> GType;

    //=========================================================================
    // ALSATimerUserInstanceError
    //=========================================================================
    pub fn alsatimer_user_instance_error_get_type() -> GType;
    pub fn alsatimer_user_instance_error_quark() -> glib::GQuark;

    //=========================================================================
    // ALSATimerDeviceInfoFlag
    //=========================================================================
    pub fn alsatimer_device_info_flag_get_type() -> GType;

    //=========================================================================
    // ALSATimerInstanceParamFlag
    //=========================================================================
    pub fn alsatimer_instance_param_flag_get_type() -> GType;

    //=========================================================================
    // ALSATimerDeviceId
    //=========================================================================
    pub fn alsatimer_device_id_get_type() -> GType;
    pub fn alsatimer_device_id_new(
        class: ALSATimerClass,
        card_id: c_int,
        device_id: c_int,
        subdevice_id: c_int,
    ) -> *mut ALSATimerDeviceId;
    pub fn alsatimer_device_id_get_card_id(self_: *const ALSATimerDeviceId, card_id: *mut c_int);
    pub fn alsatimer_device_id_get_class(
        self_: *const ALSATimerDeviceId,
        class: *mut ALSATimerClass,
    );
    pub fn alsatimer_device_id_get_device_id(
        self_: *const ALSATimerDeviceId,
        device_id: *mut c_int,
    );
    pub fn alsatimer_device_id_get_subdevice_id(
        self_: *const ALSATimerDeviceId,
        subdevice_id: *mut c_int,
    );

    //=========================================================================
    // ALSATimerRealTimeEvent
    //=========================================================================
    pub fn alsatimer_real_time_event_get_type() -> GType;
    pub fn alsatimer_real_time_event_get_event(
        self_: *const ALSATimerRealTimeEvent,
        event: *mut ALSATimerRealTimeEventType,
    );
    pub fn alsatimer_real_time_event_get_time(
        self_: *const ALSATimerRealTimeEvent,
        real_time: *const *mut [i64; 2],
    );
    pub fn alsatimer_real_time_event_get_val(
        self_: *const ALSATimerRealTimeEvent,
        val: *mut c_uint,
    );

    //=========================================================================
    // ALSATimerTickTimeEvent
    //=========================================================================
    pub fn alsatimer_tick_time_event_get_type() -> GType;
    pub fn alsatimer_tick_time_event_get_count(
        self_: *const ALSATimerTickTimeEvent,
        count: *mut c_uint,
    );
    pub fn alsatimer_tick_time_event_get_resolution(
        self_: *const ALSATimerTickTimeEvent,
        resolution: *mut c_uint,
    );

    //=========================================================================
    // ALSATimerDeviceInfo
    //=========================================================================
    pub fn alsatimer_device_info_get_type() -> GType;

    //=========================================================================
    // ALSATimerDeviceParams
    //=========================================================================
    pub fn alsatimer_device_params_get_type() -> GType;
    pub fn alsatimer_device_params_new() -> *mut ALSATimerDeviceParams;

    //=========================================================================
    // ALSATimerDeviceStatus
    //=========================================================================
    pub fn alsatimer_device_status_get_type() -> GType;
    pub fn alsatimer_device_status_new() -> *mut ALSATimerDeviceStatus;

    //=========================================================================
    // ALSATimerInstanceInfo
    //=========================================================================
    pub fn alsatimer_instance_info_get_type() -> GType;

    //=========================================================================
    // ALSATimerInstanceParams
    //=========================================================================
    pub fn alsatimer_instance_params_get_type() -> GType;
    pub fn alsatimer_instance_params_new() -> *mut ALSATimerInstanceParams;
    pub fn alsatimer_instance_params_get_event_filter(
        self_: *mut ALSATimerInstanceParams,
        entries: *mut *mut ALSATimerRealTimeEventType,
        entry_count: *mut size_t,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_instance_params_set_event_filter(
        self_: *mut ALSATimerInstanceParams,
        entries: *const ALSATimerRealTimeEventType,
        entry_count: size_t,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // ALSATimerInstanceStatus
    //=========================================================================
    pub fn alsatimer_instance_status_get_type() -> GType;
    pub fn alsatimer_instance_status_new() -> *mut ALSATimerInstanceStatus;
    pub fn alsatimer_instance_status_get_time(
        self_: *mut ALSATimerInstanceStatus,
        real_time: *mut *const [i64; 2],
    );

    //=========================================================================
    // ALSATimerUserInstance
    //=========================================================================
    pub fn alsatimer_user_instance_get_type() -> GType;
    pub fn alsatimer_user_instance_new() -> *mut ALSATimerUserInstance;
    pub fn alsatimer_user_instance_attach(
        self_: *mut ALSATimerUserInstance,
        device_id: *mut ALSATimerDeviceId,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_attach_as_slave(
        self_: *mut ALSATimerUserInstance,
        slave_class: ALSATimerSlaveClass,
        slave_id: c_int,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_choose_event_type(
        self_: *mut ALSATimerUserInstance,
        event_type: ALSATimerEventType,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_continue(
        self_: *mut ALSATimerUserInstance,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_create_source(
        self_: *mut ALSATimerUserInstance,
        gsrc: *mut *mut glib::GSource,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_get_info(
        self_: *mut ALSATimerUserInstance,
        instance_info: *mut *mut ALSATimerInstanceInfo,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_get_protocol_version(
        self_: *mut ALSATimerUserInstance,
        proto_ver_triplet: *mut *const [u16; 3],
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_get_status(
        self_: *mut ALSATimerUserInstance,
        instance_status: *const *mut ALSATimerInstanceStatus,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_open(
        self_: *mut ALSATimerUserInstance,
        open_flag: c_int,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_pause(
        self_: *mut ALSATimerUserInstance,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_set_params(
        self_: *mut ALSATimerUserInstance,
        instance_params: *const *mut ALSATimerInstanceParams,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_start(
        self_: *mut ALSATimerUserInstance,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_user_instance_stop(
        self_: *mut ALSATimerUserInstance,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn alsatimer_get_device_id_list(
        entries: *mut *mut glib::GList,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_get_device_info(
        device_id: *mut ALSATimerDeviceId,
        device_info: *mut *mut ALSATimerDeviceInfo,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_get_device_status(
        device_id: *mut ALSATimerDeviceId,
        device_status: *const *mut ALSATimerDeviceStatus,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_get_devnode(
        devnode: *mut *mut c_char,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_get_real_time_clock_id(
        clock_id: *mut c_int,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_get_sysname(
        sysname: *mut *mut c_char,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn alsatimer_set_device_params(
        device_id: *mut ALSATimerDeviceId,
        device_params: *const ALSATimerDeviceParams,
        error: *mut *mut glib::GError,
    ) -> gboolean;

}
