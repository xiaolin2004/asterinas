// SPDX-License-Identifier: MPL-2.0

#![allow(unused_variables)]

use super::*;
use crate::{events::IoEvents, fs::inode_handle::FileIo, prelude::*, process::signal::Poller};

pub struct Zero;

impl Device for Zero {
    fn type_(&self) -> DeviceType {
        DeviceType::CharDevice
    }

    fn id(&self) -> DeviceId {
        // Same value with Linux
        DeviceId::new(1, 5)
    }
}

impl FileIo for Zero {
    fn read(&self, writer: &mut VmWriter) -> Result<usize> {
        // TODO: Use more efficient way when need to read a bunch of zeros once.
        let read_len = writer.avail();
        for _ in 0..read_len {
            writer.write_val(&0u8)?;
        }
        Ok(read_len)
    }

    fn write(&self, reader: &mut VmReader) -> Result<usize> {
        Ok(reader.remain())
    }

    fn poll(&self, mask: IoEvents, poller: Option<&mut Poller>) -> IoEvents {
        let events = IoEvents::IN | IoEvents::OUT;
        events & mask
    }
}
