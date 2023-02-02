use std::error::Error;
use crate::message::Message;
use crate::send_error::borrowed::SendErrorBorrowed;
use crate::send_error::owned::SendErrorOwned;
use crate::send_error::SendError;

#[derive(Debug)]
pub struct ReportedSendError<'a> {
    send_err: SendErrorBorrowed<'a>,
    error_report_summary: ErrorReportSummary,
}

impl<'a> ReportedSendError<'a> {
    pub fn new(send_err: SendErrorBorrowed<'a>, error_report_summary: ErrorReportSummary) -> Self {
        Self {
            send_err,
            error_report_summary,
        }
    }

    pub fn get_report_summary(&self) -> &ErrorReportSummary {
        &self.error_report_summary
    }
}

impl<'a> SendError for ReportedSendError<'a> {
    fn get_error(&self) -> &Box<dyn Error> {
        &self.send_err.get_error()
    }

    fn get_failed_destination_id(&self) -> &str {
        &self.send_err.get_failed_destination_id()
    }

    fn get_failed_message(&self) -> &Message {
        self.send_err.get_failed_message()
    }
}

#[derive(Default, Debug)]
pub struct ErrorReportSummary {
    /// Whether the error was reported to any error-receiving destination
    reported: bool,
    /// Destinations that a report was attempted but failed.
    error_report_failures: Vec<SendErrorOwned>
}

impl ErrorReportSummary {
    pub fn new(reported: bool, error_report_failures: Vec<SendErrorOwned>) -> Self {
        Self {
            reported,
            error_report_failures,
        }
    }

    pub fn was_reported(&self) -> bool {
        self.reported
    }

    pub fn get_report_failures(&self) -> &Vec<SendErrorOwned> {
        &self.error_report_failures
    }
}