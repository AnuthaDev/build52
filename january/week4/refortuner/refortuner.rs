// SPDX-License-Identifier: GPL-2.0

//! Rust refortuner device module.
//!
//! Creates a misc device at /dev/refortuner that returns a random fortune when read.

use core::pin::Pin;

use kernel::{
    c_str,
    device::Device,
    fs::{File, Kiocb},
    iov::IovIterDest,
    miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration},
    new_mutex,
    prelude::*,
    sync::{aref::ARef, Mutex},
};

module! {
    type: RustMiscDeviceModule,
    name: "refortuner",
    authors: ["build52"],
    description: "A fortune telling kernel module",
    license: "GPL",
}

mod fortune_list;

#[pin_data]
struct RustMiscDeviceModule {
    #[pin]
    _miscdev: MiscDeviceRegistration<RustMiscDevice>,
}

impl kernel::InPlaceModule for RustMiscDeviceModule {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("Initialising Refortuner Device\n");

        let options = MiscDeviceOptions {
            name: c_str!("refortuner"),
        };

        try_pin_init!(Self {
            _miscdev <- MiscDeviceRegistration::register(options),
        })
    }
}

struct Inner {
    fortune: KVVec<u8>,
}

#[pin_data(PinnedDrop)]
struct RustMiscDevice {
    #[pin]
    inner: Mutex<Inner>,
    dev: ARef<Device>,
}

#[vtable]
impl MiscDevice for RustMiscDevice {
    type Ptr = Pin<KBox<Self>>;

    fn open(_file: &File, misc: &MiscDeviceRegistration<Self>) -> Result<Pin<KBox<Self>>> {
        let dev = ARef::from(misc.device());

        dev_info!(dev, "Opening Refortuner Device\n");

        // Generate a random index to select a quote
        let random_num = unsafe { kernel::bindings::get_random_u32() };
        let index = (random_num as usize) % fortune_list::FORTUNES.len();
        let selected_fortune = fortune_list::FORTUNES[index];

        let mut fortune = KVVec::new();
        fortune.extend_from_slice(selected_fortune.as_bytes(), GFP_KERNEL)?;

        KBox::try_pin_init(
            try_pin_init! {
                RustMiscDevice {
                    inner <- new_mutex!(Inner {
                        fortune: fortune,
                    }),
                    dev: dev,
                }
            },
            GFP_KERNEL,
        )
    }

    fn read_iter(mut kiocb: Kiocb<'_, Self::Ptr>, iov: &mut IovIterDest<'_>) -> Result<usize> {
        let me = kiocb.file();
        dev_info!(me.dev, "Reading from Refortuner Device\n");

        let inner = me.inner.lock();
        let read = iov.simple_read_from_buffer(kiocb.ki_pos_mut(), &inner.fortune)?;

        Ok(read)
    }
}

#[pinned_drop]
impl PinnedDrop for RustMiscDevice {
    fn drop(self: Pin<&mut Self>) {
        dev_info!(self.dev, "Closing Refortuner Device\n");
    }
}
