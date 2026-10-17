Lossless JPEG 2000 fixture whose encoder stopped some code-blocks' coding
passes early.

`emri_small_jpeg_2k_lossless_frame1.j2c` is frame 1 (zero-based) of
`emri_small_jpeg_2k_lossless.dcm` from [pydicom-data](https://github.com/pydicom/pydicom-data)
(commit `abc42b90985fb6cf385aa4af766d2c9c94a257a4`; file SHA-256
`b2b4063359a08ed3b0afa9f4e4f72f84af79e5116515b446d9a30da9dc7f1888`). It is
that frame's encapsulated fragment, byte for byte, including the trailing
`00` padding after EOC. It is a 64 × 64 single-component reversible 5/3
codestream with SIZ precision 16 (unsigned), 5 decomposition levels and
64 × 64 code blocks. Its COM marker reads "Created by GDCM/OpenJPEG
version 2.0".

The LL and coarsest LH code blocks stop before their last coding passes:
for example, the LL block codes 39 of its 43 possible passes. T.800
E.1.2.2 (Equation E-8) reconstructs such coefficients at
q ± r·2^(Mb−Nb), with the reconstruction parameter r chosen by the decoder
(commonly 1/2). This encoder dropped passes whose bits equal the r = 1/2
midpoint, so a decoder that uses r = 0 decodes this frame
up to 2 levels low in 3,336 of its 4,096 samples.

`emri_small_frame1.u16le` holds the expected samples: frame 1 of
`emri_small.dcm` from the same pydicom-data commit (file SHA-256
`151233ec63f64ebb63b979df51aa827cd612a53422c073f6ef341770c7bc9a56`), the
uncompressed original of the same series, as row-major little-endian
`u16`. No JPEG 2000 decoder was involved in producing it; OpenJPEG 2.5.4
decodes the codestream to the same bytes.

pydicom-data is MIT licensed; its license is retained in
`LICENSE.pydicom-data`.
