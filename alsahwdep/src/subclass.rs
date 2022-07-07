// SPDX-License-Identifier: MIT

mod device_common;

pub mod prelude {
    pub use super::device_common::DeviceCommonImpl;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Error, Source},
    libc::*,
};