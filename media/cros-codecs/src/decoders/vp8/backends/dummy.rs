// Copyright 2022 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains a dummy backend whose only purpose is to let the decoder
// run so we can test it in isolation.

use std::cell::RefCell;
use std::rc::Rc;

use crate::decoders::vp8::backends::StatelessDecoderBackend;
use crate::decoders::vp8::decoder::Decoder;
use crate::decoders::vp8::parser::Header;
use crate::decoders::vp8::parser::MbLfAdjustments;
use crate::decoders::vp8::parser::Segmentation;
use crate::decoders::BlockingMode;
use crate::utils::dummy::*;

impl StatelessDecoderBackend for Backend {
    fn new_sequence(&mut self, _: &crate::decoders::vp8::parser::Header) -> super::Result<()> {
        Ok(())
    }

    fn submit_picture(
        &mut self,
        _: &Header,
        _: Option<&Self::Handle>,
        _: Option<&Self::Handle>,
        _: Option<&Self::Handle>,
        _: &[u8],
        _: &Segmentation,
        _: &MbLfAdjustments,
        _: u64,
        _: BlockingMode,
    ) -> super::Result<Self::Handle> {
        Ok(Handle {
            handle: Rc::new(RefCell::new(BackendHandle)),
        })
    }

    #[cfg(test)]
    fn get_test_params(&self) -> &dyn std::any::Any {
        // There are no test parameters for the dummy backend.
        unimplemented!()
    }
}

impl Decoder<Handle> {
    // Creates a new instance of the decoder using the dummy backend.
    pub fn new_dummy(blocking_mode: BlockingMode) -> anyhow::Result<Self> {
        Self::new(Box::new(Backend::new()), blocking_mode)
    }
}
