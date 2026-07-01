#[cfg(feature = "cycle-tracker")]
#[inline(always)]
pub(crate) fn start(name: &str) {
    match name {
        "derivation-batch-brotli-decompress" => start_tag!(derivation_batch_brotli_decompress),
        "derivation-batch-decompress" => start_tag!(derivation_batch_decompress),
        "derivation-batch-payload-decode" => start_tag!(derivation_batch_payload_decode),
        "derivation-batch-rlp-decode" => start_tag!(derivation_batch_rlp_decode),
        "derivation-batch-zlib-decompress" => start_tag!(derivation_batch_zlib_decompress),
        _ => {}
    }
}

#[cfg(feature = "cycle-tracker")]
#[inline(always)]
pub(crate) fn end(name: &str) {
    match name {
        "derivation-batch-brotli-decompress" => end_tag!(derivation_batch_brotli_decompress),
        "derivation-batch-decompress" => end_tag!(derivation_batch_decompress),
        "derivation-batch-payload-decode" => end_tag!(derivation_batch_payload_decode),
        "derivation-batch-rlp-decode" => end_tag!(derivation_batch_rlp_decode),
        "derivation-batch-zlib-decompress" => end_tag!(derivation_batch_zlib_decompress),
        _ => {}
    }
}

#[cfg(feature = "cycle-tracker")]
macro_rules! start_tag {
    ($name:ident) => {{
        ziskos::profile_report_start!($name);
        ziskos::profile_report_steps_start!($name);
    }};
}

#[cfg(feature = "cycle-tracker")]
macro_rules! end_tag {
    ($name:ident) => {{
        ziskos::profile_report_steps_end!($name);
        ziskos::profile_report_end!($name);
    }};
}

#[cfg(feature = "cycle-tracker")]
use {end_tag, start_tag};

#[cfg(not(feature = "cycle-tracker"))]
#[inline(always)]
pub(crate) fn start(_: &str) {}

#[cfg(not(feature = "cycle-tracker"))]
#[inline(always)]
pub(crate) fn end(_: &str) {}
