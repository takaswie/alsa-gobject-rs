// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    /// A boxed object to express address in ALSA Sequencer.
    ///
    /// A [`Addr`][crate::Addr] is a boxed object to express address in ALSA Sequencer. The address consists
    /// of two parts; the numeric ID of client and port.
    ///
    /// The object wraps `struct snd_seq_addr` in UAPI of Linux sound subsystem.
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Addr(Boxed<ffi::ALSASeqAddr>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::alsaseq_addr_get_type(), ptr as *mut _) as *mut ffi::ALSASeqAddr,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::alsaseq_addr_get_type(), ptr as *mut _),
        type_ => || ffi::alsaseq_addr_get_type(),
    }
}

impl Addr {
    /// Allocate and return an instance of [`Addr`][crate::Addr].
    /// ## `client_id`
    /// The numeric ID of client to address.
    /// ## `port_id`
    /// The numeric ID of port to address.
    ///
    /// # Returns
    ///
    /// A [`Addr`][crate::Addr].
    #[doc(alias = "alsaseq_addr_new")]
    pub fn new(client_id: u8, port_id: u8) -> Addr {
        unsafe { from_glib_full(ffi::alsaseq_addr_new(client_id, port_id)) }
    }

    #[doc(alias = "alsaseq_addr_equal")]
    fn equal(&self, target: &Addr) -> bool {
        unsafe {
            from_glib(ffi::alsaseq_addr_equal(
                self.to_glib_none().0,
                target.to_glib_none().0,
            ))
        }
    }

    /// Get the numeric ID of client to address.
    ///
    /// # Returns
    ///
    ///
    /// ## `client_id`
    /// The numeric ID of client to address.
    #[doc(alias = "alsaseq_addr_get_client_id")]
    #[doc(alias = "get_client_id")]
    pub fn client_id(&self) -> u8 {
        unsafe {
            let mut client_id = std::mem::MaybeUninit::uninit();
            ffi::alsaseq_addr_get_client_id(self.to_glib_none().0, client_id.as_mut_ptr());
            client_id.assume_init()
        }
    }

    /// Get the numeric ID of port to address.
    ///
    /// # Returns
    ///
    ///
    /// ## `port_id`
    /// The numeric ID of port to address.
    #[doc(alias = "alsaseq_addr_get_port_id")]
    #[doc(alias = "get_port_id")]
    pub fn port_id(&self) -> u8 {
        unsafe {
            let mut port_id = std::mem::MaybeUninit::uninit();
            ffi::alsaseq_addr_get_port_id(self.to_glib_none().0, port_id.as_mut_ptr());
            port_id.assume_init()
        }
    }
}

impl PartialEq for Addr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Addr {}

unsafe impl Send for Addr {}
