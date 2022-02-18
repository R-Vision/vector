use super::prelude::{error_stage, error_type};
use metrics::counter;
use vector_core::internal_event::InternalEvent;

#[derive(Debug)]
pub(crate) struct KeyValueParserError {
    pub(crate) key: String,
    pub(crate) error: crate::types::Error,
}

impl InternalEvent for KeyValueParserError {
    fn emit_logs(&self) {
        error!(
            message = "Event failed to parse as key/value.",
            key = %self.key,
            error = %self.error,
            error_type = error_type::PARSER_FAILED,
            stage = error_stage::PROCESSING,
            internal_log_rate_secs = 30
        )
    }

    fn emit_metrics(&self) {
        counter!(
            "component_errors_total", 1,
            "error" => self.error.to_string(),
            "error_type" => error_type::PARSER_FAILED,
            "stage" => error_stage::PROCESSING,
            "key" => self.key.clone(),
        );
        // deprecated
        counter!(
            "processing_errors_total", 1,
            "error_type" => "failed_parse",
        );
    }
}

#[derive(Debug)]
pub(self) struct KeyValueMultipleSplitResults {
    pub(super) pair: String,
}
