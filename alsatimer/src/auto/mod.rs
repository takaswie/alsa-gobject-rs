// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod device_info;
pub use self::device_info::{DeviceInfo, NONE_DEVICE_INFO};
pub use self::device_info::DeviceInfoExt;

mod device_params;
pub use self::device_params::{DeviceParams, NONE_DEVICE_PARAMS};
pub use self::device_params::DeviceParamsExt;

mod device_status;
pub use self::device_status::{DeviceStatus, NONE_DEVICE_STATUS};
pub use self::device_status::DeviceStatusExt;

mod instance_info;
pub use self::instance_info::{InstanceInfo, NONE_INSTANCE_INFO};
pub use self::instance_info::InstanceInfoExt;

mod instance_params;
pub use self::instance_params::{InstanceParams, NONE_INSTANCE_PARAMS};
pub use self::instance_params::InstanceParamsExt;

mod instance_status;
pub use self::instance_status::{InstanceStatus, NONE_INSTANCE_STATUS};
pub use self::instance_status::InstanceStatusExt;

mod user_instance;
pub use self::user_instance::{UserInstance, NONE_USER_INSTANCE};
pub use self::user_instance::UserInstanceExt;

mod device_id;
pub use self::device_id::DeviceId;

mod event_data_tick;
pub use self::event_data_tick::EventDataTick;

mod event_data_tstamp;
pub use self::event_data_tstamp::EventDataTstamp;

mod enums;
pub use self::enums::Class;
pub use self::enums::EventDataType;
pub use self::enums::EventType;
pub use self::enums::SlaveClass;
pub use self::enums::SpecificGlobalDevice;
pub use self::enums::UserInstanceError;

mod flags;
pub use self::flags::DeviceInfoFlag;
pub use self::flags::InstanceParamFlag;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::DeviceInfoExt;
    pub use super::DeviceParamsExt;
    pub use super::DeviceStatusExt;
    pub use super::InstanceInfoExt;
    pub use super::InstanceParamsExt;
    pub use super::InstanceStatusExt;
    pub use super::UserInstanceExt;
}
