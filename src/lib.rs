//! FFI bindings to sasquatch's LZMA adaptive library
//!
//! This crate provides Rust bindings to the LZMA adaptive decompression
//! functionality from the sasquatch project, which implements brute-force
//! parameter discovery for LZMA compressed data in SquashFS v3 filesystems.

use std::os::raw::{c_int, c_ulong};

extern "C" {
    /// Decompress LZMA data with specific parameters
    ///
    /// This function is from sasquatch's LZMA adaptive implementation and supports
    /// brute-force parameter discovery for LZMA compressed data blocks.
    ///
    /// # Parameters
    /// - `dest`: Output buffer for decompressed data
    /// - `dest_len`: Pointer to size of output buffer, updated with actual decompressed size
    /// - `source`: Input compressed data
    /// - `source_len`: Size of input data
    /// - `lc`: Literal context bits (0-4)
    /// - `lp`: Literal position bits (0-4)
    /// - `pb`: Position bits (0-4)
    /// - `dictionary_size`: LZMA dictionary size (or 0 for default 8MB)
    /// - `offset`: Offset into source data where LZMA data begins
    ///
    /// # Returns
    /// 0 on success (Z_OK), non-zero error code on failure
    pub fn lzmaspec_uncompress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
        lc: c_int,
        lp: c_int,
        pb: c_int,
        dictionary_size: c_int,
        offset: c_int,
    ) -> c_int;
}

/// Safe wrapper around lzmaspec_uncompress
///
/// # Parameters
/// - `source`: Input compressed data
/// - `lc`: Literal context bits (0-4)
/// - `lp`: Literal position bits (0-4)
/// - `pb`: Position bits (0-4)
/// - `dictionary_size`: LZMA dictionary size (0 for default)
/// - `offset`: Offset into source data where LZMA data begins
/// - `max_output_size`: Maximum expected output size
///
/// # Returns
/// `Ok(Vec<u8>)` with decompressed data on success, `Err(i32)` with error code on failure
pub fn decompress_lzma(
    source: &[u8],
    lc: u32,
    lp: u32,
    pb: u32,
    dictionary_size: u32,
    offset: usize,
    max_output_size: usize,
) -> Result<Vec<u8>, i32> {
    if offset >= source.len() {
        return Err(-1); // Invalid offset
    }

    let mut output = vec![0u8; max_output_size];
    let mut dest_len = max_output_size as c_ulong;

    let result = unsafe {
        lzmaspec_uncompress(
            output.as_mut_ptr(),
            &mut dest_len,
            source.as_ptr(),
            source.len() as c_ulong,
            lc as c_int,
            lp as c_int,
            pb as c_int,
            dictionary_size as c_int,
            offset as c_int,
        )
    };

    if result == 0 && dest_len > 0 {
        output.truncate(dest_len as usize);
        Ok(output)
    } else {
        Err(result)
    }
}
