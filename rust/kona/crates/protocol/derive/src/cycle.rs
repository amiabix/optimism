#[cfg(feature = "cycle-tracker")]
#[inline(always)]
pub(crate) fn start(name: &str) {
    match name {
        "derivation-attributes-build" => start_tag!(derivation_attributes_build),
        "derivation-attributes-load-batch" => start_tag!(derivation_attributes_load_batch),
        "derivation-batch-add-validity-check" => start_tag!(derivation_batch_add_validity_check),
        "derivation-batch-derive" => start_tag!(derivation_batch_derive),
        "derivation-batch-queue-input" => start_tag!(derivation_batch_queue_input),
        "derivation-batch-queue-span-expand" => start_tag!(derivation_batch_queue_span_expand),
        "derivation-batch-stream-input" => start_tag!(derivation_batch_stream_input),
        "derivation-batch-validity-check" => start_tag!(derivation_batch_validity_check),
        "derivation-blob-decode" => start_tag!(derivation_blob_decode),
        "derivation-blob-fetch" => start_tag!(derivation_blob_fetch),
        "derivation-blob-fill" => start_tag!(derivation_blob_fill),
        "derivation-blob-tx-filter" => start_tag!(derivation_blob_tx_filter),
        "derivation-channel-batch-decode" => start_tag!(derivation_channel_batch_decode),
        "derivation-channel-data" => start_tag!(derivation_channel_data),
        "derivation-channel-decompress" => start_tag!(derivation_channel_decompress),
        "derivation-channel-reader-set" => start_tag!(derivation_channel_reader_set),
        "derivation-frame-data" => start_tag!(derivation_frame_data),
        "derivation-frame-parse" => start_tag!(derivation_frame_parse),
        "derivation-frame-prune" => start_tag!(derivation_frame_prune),
        "derivation-l1-block-transactions" => start_tag!(derivation_l1_block_transactions),
        "derivation-l1-retrieval" => start_tag!(derivation_l1_retrieval),
        "derivation-span-expand" => start_tag!(derivation_span_expand),
        "derivation-span-next-batch" => start_tag!(derivation_span_next_batch),
        "derivation-span-prefix-check" => start_tag!(derivation_span_prefix_check),
        _ => {}
    }
}

#[cfg(feature = "cycle-tracker")]
#[inline(always)]
pub(crate) fn end(name: &str) {
    match name {
        "derivation-attributes-build" => end_tag!(derivation_attributes_build),
        "derivation-attributes-load-batch" => end_tag!(derivation_attributes_load_batch),
        "derivation-batch-add-validity-check" => end_tag!(derivation_batch_add_validity_check),
        "derivation-batch-derive" => end_tag!(derivation_batch_derive),
        "derivation-batch-queue-input" => end_tag!(derivation_batch_queue_input),
        "derivation-batch-queue-span-expand" => end_tag!(derivation_batch_queue_span_expand),
        "derivation-batch-stream-input" => end_tag!(derivation_batch_stream_input),
        "derivation-batch-validity-check" => end_tag!(derivation_batch_validity_check),
        "derivation-blob-decode" => end_tag!(derivation_blob_decode),
        "derivation-blob-fetch" => end_tag!(derivation_blob_fetch),
        "derivation-blob-fill" => end_tag!(derivation_blob_fill),
        "derivation-blob-tx-filter" => end_tag!(derivation_blob_tx_filter),
        "derivation-channel-batch-decode" => end_tag!(derivation_channel_batch_decode),
        "derivation-channel-data" => end_tag!(derivation_channel_data),
        "derivation-channel-decompress" => end_tag!(derivation_channel_decompress),
        "derivation-channel-reader-set" => end_tag!(derivation_channel_reader_set),
        "derivation-frame-data" => end_tag!(derivation_frame_data),
        "derivation-frame-parse" => end_tag!(derivation_frame_parse),
        "derivation-frame-prune" => end_tag!(derivation_frame_prune),
        "derivation-l1-block-transactions" => end_tag!(derivation_l1_block_transactions),
        "derivation-l1-retrieval" => end_tag!(derivation_l1_retrieval),
        "derivation-span-expand" => end_tag!(derivation_span_expand),
        "derivation-span-next-batch" => end_tag!(derivation_span_next_batch),
        "derivation-span-prefix-check" => end_tag!(derivation_span_prefix_check),
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
