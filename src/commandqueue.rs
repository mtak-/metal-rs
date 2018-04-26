// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use objc_foundation::{NSString, INSString};

pub enum MTLCommandQueue {}

foreign_obj_type! {
    type CType = MTLCommandQueue;
    pub struct CommandQueue;
    pub struct CommandQueueRef;
}

impl CommandQueueRef {
    pub fn label(&self) -> &str {
        unsafe {
            let label: &NSString = msg_send![self, label];
            label.as_str()
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = NSString::from_str(label);
            msg_send![self, setLabel:nslabel]
        }
    }

    pub fn new_command_buffer(&self) -> &CommandBufferRef {
        unsafe {
            msg_send![self, commandBuffer]
        }
    }

    pub fn new_command_buffer_with_unretained_references(&self) -> &CommandBufferRef {
        unsafe {
            msg_send![self, commandBufferWithUnretainedReferences]
        }
    }

    pub fn device(&self) -> &DeviceRef {
        unsafe {
            msg_send![self, device]
        }
    }
}
