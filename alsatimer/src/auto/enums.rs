// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

/// A set of enumerations for the class of timer device.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "ALSATimerClass")]
pub enum Class {
    #[doc(alias = "ALSATIMER_CLASS_NONE")]
    None,
    #[doc(alias = "ALSATIMER_CLASS_GLOBAL")]
    Global,
    #[doc(alias = "ALSATIMER_CLASS_CARD")]
    Card,
    #[doc(alias = "ALSATIMER_CLASS_PCM")]
    Pcm,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Class::{}",
            match *self {
                Self::None => "None",
                Self::Global => "Global",
                Self::Card => "Card",
                Self::Pcm => "Pcm",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Class {
    type GlibType = ffi::ALSATimerClass;

    #[inline]
    fn into_glib(self) -> ffi::ALSATimerClass {
        match self {
            Self::None => ffi::ALSATIMER_CLASS_NONE,
            Self::Global => ffi::ALSATIMER_CLASS_GLOBAL,
            Self::Card => ffi::ALSATIMER_CLASS_CARD,
            Self::Pcm => ffi::ALSATIMER_CLASS_PCM,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSATimerClass> for Class {
    #[inline]
    unsafe fn from_glib(value: ffi::ALSATimerClass) -> Self {
        match value {
            ffi::ALSATIMER_CLASS_NONE => Self::None,
            ffi::ALSATIMER_CLASS_GLOBAL => Self::Global,
            ffi::ALSATIMER_CLASS_CARD => Self::Card,
            ffi::ALSATIMER_CLASS_PCM => Self::Pcm,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for Class {
    #[inline]
    #[doc(alias = "alsatimer_class_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::alsatimer_class_get_type()) }
    }
}

impl glib::HasParamSpec for Class {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for Class {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Class {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Class {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Class> for glib::Value {
    #[inline]
    fn from(v: Class) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of enumerations for the type of event.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "ALSATimerEventType")]
pub enum EventType {
    #[doc(alias = "ALSATIMER_EVENT_TYPE_TICK_TIME")]
    TickTime,
    #[doc(alias = "ALSATIMER_EVENT_TYPE_REAL_TIME")]
    RealTime,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EventType::{}",
            match *self {
                Self::TickTime => "TickTime",
                Self::RealTime => "RealTime",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for EventType {
    type GlibType = ffi::ALSATimerEventType;

    #[inline]
    fn into_glib(self) -> ffi::ALSATimerEventType {
        match self {
            Self::TickTime => ffi::ALSATIMER_EVENT_TYPE_TICK_TIME,
            Self::RealTime => ffi::ALSATIMER_EVENT_TYPE_REAL_TIME,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSATimerEventType> for EventType {
    #[inline]
    unsafe fn from_glib(value: ffi::ALSATimerEventType) -> Self {
        match value {
            ffi::ALSATIMER_EVENT_TYPE_TICK_TIME => Self::TickTime,
            ffi::ALSATIMER_EVENT_TYPE_REAL_TIME => Self::RealTime,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for EventType {
    #[inline]
    #[doc(alias = "alsatimer_event_type_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::alsatimer_event_type_get_type()) }
    }
}

impl glib::HasParamSpec for EventType {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for EventType {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for EventType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for EventType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<EventType> for glib::Value {
    #[inline]
    fn from(v: EventType) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of enumerations for the type of real time event.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "ALSATimerRealTimeEventType")]
pub enum RealTimeEventType {
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_RESOLUTION")]
    Resolution,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_TICK")]
    Tick,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_START")]
    Start,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_STOP")]
    Stop,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_CONTINUE")]
    Continue,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_PAUSE")]
    Pause,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_EARLY")]
    Early,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_SUSPEND")]
    Suspend,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_RESUME")]
    Resume,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_MSTART")]
    Mstart,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_MSTOP")]
    Mstop,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_MCONTINUE")]
    Mcontinue,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_MPAUSE")]
    Mpause,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_MSUSPEND")]
    Msuspend,
    #[doc(alias = "ALSATIMER_REAL_TIME_EVENT_TYPE_MRESUME")]
    Mresume,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RealTimeEventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RealTimeEventType::{}",
            match *self {
                Self::Resolution => "Resolution",
                Self::Tick => "Tick",
                Self::Start => "Start",
                Self::Stop => "Stop",
                Self::Continue => "Continue",
                Self::Pause => "Pause",
                Self::Early => "Early",
                Self::Suspend => "Suspend",
                Self::Resume => "Resume",
                Self::Mstart => "Mstart",
                Self::Mstop => "Mstop",
                Self::Mcontinue => "Mcontinue",
                Self::Mpause => "Mpause",
                Self::Msuspend => "Msuspend",
                Self::Mresume => "Mresume",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for RealTimeEventType {
    type GlibType = ffi::ALSATimerRealTimeEventType;

    fn into_glib(self) -> ffi::ALSATimerRealTimeEventType {
        match self {
            Self::Resolution => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_RESOLUTION,
            Self::Tick => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_TICK,
            Self::Start => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_START,
            Self::Stop => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_STOP,
            Self::Continue => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_CONTINUE,
            Self::Pause => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_PAUSE,
            Self::Early => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_EARLY,
            Self::Suspend => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_SUSPEND,
            Self::Resume => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_RESUME,
            Self::Mstart => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MSTART,
            Self::Mstop => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MSTOP,
            Self::Mcontinue => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MCONTINUE,
            Self::Mpause => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MPAUSE,
            Self::Msuspend => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MSUSPEND,
            Self::Mresume => ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MRESUME,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSATimerRealTimeEventType> for RealTimeEventType {
    unsafe fn from_glib(value: ffi::ALSATimerRealTimeEventType) -> Self {
        match value {
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_RESOLUTION => Self::Resolution,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_TICK => Self::Tick,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_START => Self::Start,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_STOP => Self::Stop,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_CONTINUE => Self::Continue,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_PAUSE => Self::Pause,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_EARLY => Self::Early,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_SUSPEND => Self::Suspend,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_RESUME => Self::Resume,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MSTART => Self::Mstart,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MSTOP => Self::Mstop,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MCONTINUE => Self::Mcontinue,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MPAUSE => Self::Mpause,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MSUSPEND => Self::Msuspend,
            ffi::ALSATIMER_REAL_TIME_EVENT_TYPE_MRESUME => Self::Mresume,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for RealTimeEventType {
    #[inline]
    #[doc(alias = "alsatimer_real_time_event_type_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::alsatimer_real_time_event_type_get_type()) }
    }
}

impl glib::HasParamSpec for RealTimeEventType {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for RealTimeEventType {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for RealTimeEventType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for RealTimeEventType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<RealTimeEventType> for glib::Value {
    #[inline]
    fn from(v: RealTimeEventType) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of enumerations for the slave class of timer instance (not timer device).
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "ALSATimerSlaveClass")]
pub enum SlaveClass {
    #[doc(alias = "ALSATIMER_SLAVE_CLASS_NONE")]
    None,
    #[doc(alias = "ALSATIMER_SLAVE_CLASS_APPLICATION")]
    Application,
    #[doc(alias = "ALSATIMER_SLAVE_CLASS_SEQUENCER")]
    Sequencer,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SlaveClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SlaveClass::{}",
            match *self {
                Self::None => "None",
                Self::Application => "Application",
                Self::Sequencer => "Sequencer",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SlaveClass {
    type GlibType = ffi::ALSATimerSlaveClass;

    #[inline]
    fn into_glib(self) -> ffi::ALSATimerSlaveClass {
        match self {
            Self::None => ffi::ALSATIMER_SLAVE_CLASS_NONE,
            Self::Application => ffi::ALSATIMER_SLAVE_CLASS_APPLICATION,
            Self::Sequencer => ffi::ALSATIMER_SLAVE_CLASS_SEQUENCER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSATimerSlaveClass> for SlaveClass {
    #[inline]
    unsafe fn from_glib(value: ffi::ALSATimerSlaveClass) -> Self {
        match value {
            ffi::ALSATIMER_SLAVE_CLASS_NONE => Self::None,
            ffi::ALSATIMER_SLAVE_CLASS_APPLICATION => Self::Application,
            ffi::ALSATIMER_SLAVE_CLASS_SEQUENCER => Self::Sequencer,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for SlaveClass {
    #[inline]
    #[doc(alias = "alsatimer_slave_class_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::alsatimer_slave_class_get_type()) }
    }
}

impl glib::HasParamSpec for SlaveClass {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for SlaveClass {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for SlaveClass {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SlaveClass {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SlaveClass> for glib::Value {
    #[inline]
    fn from(v: SlaveClass) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of enumerations for the kind of global timer device.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "ALSATimerSpecificGlobalDevice")]
pub enum SpecificGlobalDevice {
    #[doc(alias = "ALSATIMER_SPECIFIC_GLOBAL_DEVICE_SYSTEM")]
    System,
    #[doc(alias = "ALSATIMER_SPECIFIC_GLOBAL_DEVICE_HRTIMER")]
    Hrtimer,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SpecificGlobalDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SpecificGlobalDevice::{}",
            match *self {
                Self::System => "System",
                Self::Hrtimer => "Hrtimer",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SpecificGlobalDevice {
    type GlibType = ffi::ALSATimerSpecificGlobalDevice;

    #[inline]
    fn into_glib(self) -> ffi::ALSATimerSpecificGlobalDevice {
        match self {
            Self::System => ffi::ALSATIMER_SPECIFIC_GLOBAL_DEVICE_SYSTEM,
            Self::Hrtimer => ffi::ALSATIMER_SPECIFIC_GLOBAL_DEVICE_HRTIMER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSATimerSpecificGlobalDevice> for SpecificGlobalDevice {
    #[inline]
    unsafe fn from_glib(value: ffi::ALSATimerSpecificGlobalDevice) -> Self {
        match value {
            ffi::ALSATIMER_SPECIFIC_GLOBAL_DEVICE_SYSTEM => Self::System,
            ffi::ALSATIMER_SPECIFIC_GLOBAL_DEVICE_HRTIMER => Self::Hrtimer,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for SpecificGlobalDevice {
    #[inline]
    #[doc(alias = "alsatimer_specific_global_device_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::alsatimer_specific_global_device_get_type()) }
    }
}

impl glib::HasParamSpec for SpecificGlobalDevice {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for SpecificGlobalDevice {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for SpecificGlobalDevice {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SpecificGlobalDevice {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SpecificGlobalDevice> for glib::Value {
    #[inline]
    fn from(v: SpecificGlobalDevice) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of error code for [`glib::Error`][crate::glib::Error] with `ALSATimer.UserInstanceError` domain.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "ALSATimerUserInstanceError")]
pub enum UserInstanceError {
    #[doc(alias = "ALSATIMER_USER_INSTANCE_ERROR_FAILED")]
    Failed,
    #[doc(alias = "ALSATIMER_USER_INSTANCE_ERROR_TIMER_NOT_FOUND")]
    TimerNotFound,
    #[doc(alias = "ALSATIMER_USER_INSTANCE_ERROR_NOT_ATTACHED")]
    NotAttached,
    #[doc(alias = "ALSATIMER_USER_INSTANCE_ERROR_ATTACHED")]
    Attached,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for UserInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UserInstanceError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::TimerNotFound => "TimerNotFound",
                Self::NotAttached => "NotAttached",
                Self::Attached => "Attached",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for UserInstanceError {
    type GlibType = ffi::ALSATimerUserInstanceError;

    #[inline]
    fn into_glib(self) -> ffi::ALSATimerUserInstanceError {
        match self {
            Self::Failed => ffi::ALSATIMER_USER_INSTANCE_ERROR_FAILED,
            Self::TimerNotFound => ffi::ALSATIMER_USER_INSTANCE_ERROR_TIMER_NOT_FOUND,
            Self::NotAttached => ffi::ALSATIMER_USER_INSTANCE_ERROR_NOT_ATTACHED,
            Self::Attached => ffi::ALSATIMER_USER_INSTANCE_ERROR_ATTACHED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSATimerUserInstanceError> for UserInstanceError {
    #[inline]
    unsafe fn from_glib(value: ffi::ALSATimerUserInstanceError) -> Self {
        match value {
            ffi::ALSATIMER_USER_INSTANCE_ERROR_FAILED => Self::Failed,
            ffi::ALSATIMER_USER_INSTANCE_ERROR_TIMER_NOT_FOUND => Self::TimerNotFound,
            ffi::ALSATIMER_USER_INSTANCE_ERROR_NOT_ATTACHED => Self::NotAttached,
            ffi::ALSATIMER_USER_INSTANCE_ERROR_ATTACHED => Self::Attached,
            value => Self::__Unknown(value),
        }
    }
}

impl glib::error::ErrorDomain for UserInstanceError {
    #[inline]
    fn domain() -> glib::Quark {
        unsafe { from_glib(ffi::alsatimer_user_instance_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        match unsafe { from_glib(code) } {
            Self::__Unknown(_) => Some(Self::Failed),
            value => Some(value),
        }
    }
}

impl StaticType for UserInstanceError {
    #[inline]
    #[doc(alias = "alsatimer_user_instance_error_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::alsatimer_user_instance_error_get_type()) }
    }
}

impl glib::HasParamSpec for UserInstanceError {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for UserInstanceError {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for UserInstanceError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for UserInstanceError {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<UserInstanceError> for glib::Value {
    #[inline]
    fn from(v: UserInstanceError) -> Self {
        ToValue::to_value(&v)
    }
}
