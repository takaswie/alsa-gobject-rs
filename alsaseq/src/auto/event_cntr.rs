// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;
use Addr;
use EventDataConnect;
use EventDataCtl;
use EventDataNote;
use EventDataQueue;
use EventDataResult;
use EventLengthMode;
use EventPriorityMode;
use EventTimeMode;
use EventTimestampMode;
use EventType;
use Tstamp;

glib_wrapper! {
    pub struct EventCntr(Object<alsaseq_sys::ALSASeqEventCntr, alsaseq_sys::ALSASeqEventCntrClass, EventCntrClass>);

    match fn {
        get_type => || alsaseq_sys::alsaseq_event_cntr_get_type(),
    }
}

impl EventCntr {
    pub fn new(count: u32) -> Result<EventCntr, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsaseq_sys::alsaseq_event_cntr_new(count, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_EVENT_CNTR: Option<&EventCntr> = None;

pub trait EventCntrExt: 'static {
    fn calculate_pool_consumption(&self, count: usize) -> Result<usize, glib::Error>;

    fn count_events(&self) -> usize;

    fn get_event_type(&self, index: usize) -> Result<EventType, glib::Error>;

    fn get_length_mode(&self, index: usize) -> Result<EventLengthMode, glib::Error>;

    fn get_priority_mode(&self, index: usize) -> Result<EventPriorityMode, glib::Error>;

    fn get_queue_id(&self, index: usize) -> Result<u8, glib::Error>;

    fn get_tag(&self, index: usize) -> Result<i8, glib::Error>;

    fn get_time_mode(&self, index: usize) -> Result<EventTimeMode, glib::Error>;

    fn get_tstamp_mode(&self, index: usize) -> Result<EventTimestampMode, glib::Error>;

    fn set_addr_data(&self, index: usize, data: &Addr) -> Result<(), glib::Error>;

    fn set_blob_data(&self, index: usize, data: &[u8]) -> Result<(), glib::Error>;

    fn set_connect_data(&self, index: usize, data: &EventDataConnect) -> Result<(), glib::Error>;

    fn set_ctl_data(&self, index: usize, data: &EventDataCtl) -> Result<(), glib::Error>;

    fn set_dst(&self, index: usize, dst: &Addr) -> Result<(), glib::Error>;

    fn set_event_type(&self, index: usize, ev_type: EventType) -> Result<(), glib::Error>;

    fn set_note_data(&self, index: usize, data: &EventDataNote) -> Result<(), glib::Error>;

    fn set_priority_mode(&self, index: usize, mode: EventPriorityMode) -> Result<(), glib::Error>;

    fn set_queue_data(&self, index: usize, data: &EventDataQueue) -> Result<(), glib::Error>;

    fn set_queue_id(&self, index: usize, queue_id: u8) -> Result<(), glib::Error>;

    fn set_result_data(&self, index: usize, data: &EventDataResult) -> Result<(), glib::Error>;

    fn set_src(&self, index: usize, src: &Addr) -> Result<(), glib::Error>;

    fn set_tag(&self, index: usize, tag: i8) -> Result<(), glib::Error>;

    fn set_time_mode(&self, index: usize, mode: EventTimeMode) -> Result<(), glib::Error>;

    fn set_tstamp(&self, index: usize, tstamp: &Tstamp) -> Result<(), glib::Error>;

    fn set_tstamp_data(&self, index: usize, data: &Tstamp) -> Result<(), glib::Error>;

    fn set_tstamp_mode(&self, index: usize, mode: EventTimestampMode) -> Result<(), glib::Error>;
}

impl<O: IsA<EventCntr>> EventCntrExt for O {
    fn calculate_pool_consumption(&self, count: usize) -> Result<usize, glib::Error> {
        unsafe {
            let mut cells = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_calculate_pool_consumption(
                self.as_ref().to_glib_none().0,
                count,
                cells.as_mut_ptr(),
                &mut error,
            );
            let cells = cells.assume_init();
            if error.is_null() {
                Ok(cells)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn count_events(&self) -> usize {
        unsafe {
            let mut count = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_cntr_count_events(
                self.as_ref().to_glib_none().0,
                count.as_mut_ptr(),
            );
            let count = count.assume_init();
            count
        }
    }

    fn get_event_type(&self, index: usize) -> Result<EventType, glib::Error> {
        unsafe {
            let mut ev_type = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_get_event_type(
                self.as_ref().to_glib_none().0,
                index,
                ev_type.as_mut_ptr(),
                &mut error,
            );
            let ev_type = ev_type.assume_init();
            if error.is_null() {
                Ok(from_glib(ev_type))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_length_mode(&self, index: usize) -> Result<EventLengthMode, glib::Error> {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_get_length_mode(
                self.as_ref().to_glib_none().0,
                index,
                mode.as_mut_ptr(),
                &mut error,
            );
            let mode = mode.assume_init();
            if error.is_null() {
                Ok(from_glib(mode))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_priority_mode(&self, index: usize) -> Result<EventPriorityMode, glib::Error> {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_get_priority_mode(
                self.as_ref().to_glib_none().0,
                index,
                mode.as_mut_ptr(),
                &mut error,
            );
            let mode = mode.assume_init();
            if error.is_null() {
                Ok(from_glib(mode))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_queue_id(&self, index: usize) -> Result<u8, glib::Error> {
        unsafe {
            let mut queue_id = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_get_queue_id(
                self.as_ref().to_glib_none().0,
                index,
                queue_id.as_mut_ptr(),
                &mut error,
            );
            let queue_id = queue_id.assume_init();
            if error.is_null() {
                Ok(queue_id)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_tag(&self, index: usize) -> Result<i8, glib::Error> {
        unsafe {
            let mut tag = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_get_tag(
                self.as_ref().to_glib_none().0,
                index,
                tag.as_mut_ptr(),
                &mut error,
            );
            let tag = tag.assume_init();
            if error.is_null() {
                Ok(tag)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_time_mode(&self, index: usize) -> Result<EventTimeMode, glib::Error> {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_get_time_mode(
                self.as_ref().to_glib_none().0,
                index,
                mode.as_mut_ptr(),
                &mut error,
            );
            let mode = mode.assume_init();
            if error.is_null() {
                Ok(from_glib(mode))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_tstamp_mode(&self, index: usize) -> Result<EventTimestampMode, glib::Error> {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_get_tstamp_mode(
                self.as_ref().to_glib_none().0,
                index,
                mode.as_mut_ptr(),
                &mut error,
            );
            let mode = mode.assume_init();
            if error.is_null() {
                Ok(from_glib(mode))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_addr_data(&self, index: usize, data: &Addr) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_addr_data(
                self.as_ref().to_glib_none().0,
                index,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_blob_data(&self, index: usize, data: &[u8]) -> Result<(), glib::Error> {
        let size = data.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_blob_data(
                self.as_ref().to_glib_none().0,
                index,
                data.to_glib_none().0,
                size,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_connect_data(&self, index: usize, data: &EventDataConnect) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_connect_data(
                self.as_ref().to_glib_none().0,
                index,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_ctl_data(&self, index: usize, data: &EventDataCtl) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_ctl_data(
                self.as_ref().to_glib_none().0,
                index,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_dst(&self, index: usize, dst: &Addr) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_dst(
                self.as_ref().to_glib_none().0,
                index,
                dst.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_event_type(&self, index: usize, ev_type: EventType) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_event_type(
                self.as_ref().to_glib_none().0,
                index,
                ev_type.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_note_data(&self, index: usize, data: &EventDataNote) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_note_data(
                self.as_ref().to_glib_none().0,
                index,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_priority_mode(&self, index: usize, mode: EventPriorityMode) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_priority_mode(
                self.as_ref().to_glib_none().0,
                index,
                mode.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_queue_data(&self, index: usize, data: &EventDataQueue) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_queue_data(
                self.as_ref().to_glib_none().0,
                index,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_queue_id(&self, index: usize, queue_id: u8) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_queue_id(
                self.as_ref().to_glib_none().0,
                index,
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

    fn set_result_data(&self, index: usize, data: &EventDataResult) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_result_data(
                self.as_ref().to_glib_none().0,
                index,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_src(&self, index: usize, src: &Addr) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_src(
                self.as_ref().to_glib_none().0,
                index,
                src.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_tag(&self, index: usize, tag: i8) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_tag(
                self.as_ref().to_glib_none().0,
                index,
                tag,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_time_mode(&self, index: usize, mode: EventTimeMode) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_time_mode(
                self.as_ref().to_glib_none().0,
                index,
                mode.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_tstamp(&self, index: usize, tstamp: &Tstamp) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_tstamp(
                self.as_ref().to_glib_none().0,
                index,
                tstamp.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_tstamp_data(&self, index: usize, data: &Tstamp) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_tstamp_data(
                self.as_ref().to_glib_none().0,
                index,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_tstamp_mode(&self, index: usize, mode: EventTimestampMode) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsaseq_sys::alsaseq_event_cntr_set_tstamp_mode(
                self.as_ref().to_glib_none().0,
                index,
                mode.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for EventCntr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventCntr")
    }
}
