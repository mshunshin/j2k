// SPDX-License-Identifier: MIT OR Apache-2.0

//! A lossless (5/3) codestream whose encoder stopped some code blocks' coding
//! passes early decodes exactly. Its undecoded low bit-planes are
//! reconstructed by T.800 Equation E-8 (E.1.2.2) with r = 1/2, the midpoint
//! the encoder relied on. See `fixtures/pydicom/README.md`.

use j2k_native::{DecodeSettings, Image};

const CODESTREAM: &[u8] =
    include_bytes!("../fixtures/pydicom/emri_small_jpeg_2k_lossless_frame1.j2c");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/pydicom/emri_small_frame1.u16le");

#[test]
fn early_stopped_reversible_code_blocks_decode_to_the_original() {
    let image = Image::new(CODESTREAM, &DecodeSettings::default()).expect("parse codestream");
    let decoded = image.decode_native().expect("decode codestream");

    assert_eq!(
        (
            decoded.width,
            decoded.height,
            decoded.num_components,
            decoded.bit_depth
        ),
        (64, 64, 1, 16)
    );
    assert_eq!(decoded.data, ORIGINAL);
}
