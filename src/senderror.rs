use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::{DestinationConfig, Message};

/// A struct containing information about what parts succeeded and what parts
/// failed about [send_message] attempt.
#[derive(Debug)]
pub struct SendErrors {
    root_destinations: Vec<DestinationConfig>,
    original_message: Message,
    errors: Vec<DestinationSendErrorInternal>,
}

impl SendErrors {

    pub fn new(root_destinations: Vec<DestinationConfig>,
               message: Message,
               errors: Vec<(DestinationConfig, usize, Box<dyn Error>, HashMap<usize, Box<dyn Error>>)>) -> Self {

        let errors = errors.into_iter().map(|(dest, config_file_index, err, failed_indices)| {
            DestinationSendErrorInternal {
                dest,
                err,
                config_file_index,
                failed_root_indices: failed_indices
            }
        }).collect();
        Self {
            root_destinations,
            original_message: message,
            errors,
        }
    }

    /// Get the message that caused errors when sent.
    pub fn get_message(&self) -> &Message {
        &self.original_message
    }
}

impl Display for SendErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-----")?;
        writeln!(f, "Summary:")?;
        writeln!(f, "Failed to send to {} destinations", self.errors.len())?;
        writeln!(f, "Message: {:?}", self.original_message)?;
        let any_root_fails = self.errors.iter().any(|err| err.dest.is_root() || !err.failed_root_indices.is_empty());
        if any_root_fails {
            writeln!(f, "Some errors failed to be reported to root destinations.")?;
            writeln!(f, "Root destination index reference:")?;
            for (i, dest) in self.root_destinations.iter().enumerate() {
                writeln!(f, "- {}: {:?}", i, dest)?;
            }
        }
        else {
            writeln!(f, "All errors were successfully reported all root destinations")?;
        }
        for error in &self.errors {
            writeln!(f, "--")?;
            writeln!(f, "Failed to send a message to {:?}", error.dest)?;
            writeln!(f, "... which is index {} in the config", error.config_file_index)?;
            writeln!(f, "Due to error: {:?}", error.err)?;
            if error.failed_root_indices.is_empty() {
                writeln!(f, "This was correctly reported to all root destinations")?;
            }
            else {
                writeln!(f, "Failed to report this error to the following destinations:")?;
                for (i, err) in &error.failed_root_indices {
                    writeln!(f, "- index {}, due to error: {}", i, err)?;
                }
            }
        }
        writeln!(f, "-----")
    }
}

#[derive(Debug)]
struct DestinationSendErrorInternal {
    err: Box<dyn Error>,
    dest: DestinationConfig,
    config_file_index: usize,
    failed_root_indices: HashMap<usize, Box<dyn Error>>,
}

pub struct DestinationSendErrorIterator<'a> {
    i: usize,
    obj: &'a SendErrors,
}

impl<'a> Iterator for DestinationSendErrorIterator<'a> {
    type Item = DestinationSendError<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(internal) = self.obj.errors.get(self.i) {
            self.i += 1;
            let mut suc = vec![];
            let mut fail = vec![];

            for (i, root_dest) in self.obj.root_destinations.iter().enumerate() {
                if let Some(err) = internal.failed_root_indices.get(&i) {
                    fail.push((root_dest, err));
                }
                else {
                    suc.push(root_dest);
                }
            }

            return Some(DestinationSendError {
                err: &internal.err,
                dest: &internal.dest,
                failed_roots: fail,
                successful_roots: suc,
                config_file_index: internal.config_file_index
            });
        }
        None
    }
}

impl<'a> IntoIterator for &'a SendErrors {
    type Item = DestinationSendError<'a>;
    type IntoIter = DestinationSendErrorIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DestinationSendErrorIterator {
            i: 0,
            obj: &self,
        }
    }
}

pub struct DestinationSendError<'a> {
    err: &'a Box<dyn Error>,
    dest: &'a DestinationConfig,
    config_file_index: usize,
    successful_roots: Vec<&'a DestinationConfig>,
    failed_roots: Vec<(&'a DestinationConfig, &'a Box<dyn Error>)>,
}

impl<'a> DestinationSendError<'a> {

    /// The original error that caused the problem.
    pub fn get_error(&self) -> &'a Box<dyn Error> {
        &self.err
    }

    /// The destination that produced the error
    pub fn get_destination(&self) -> &'a DestinationConfig {
        &self.dest
    }

    /// The index in the config file that the destination comes from (may be useful if you have many similar destinations)
    pub fn get_destination_config_file_index(&self) -> usize {
        self.config_file_index
    }

    /// The root destination where this error was successfully reported
    pub fn get_successful_root_destinations(&self) -> &Vec<&'a DestinationConfig> {
        &self.successful_roots
    }

    /// The root destinations where this error failed to be reported
    /// If there are any present here, there is a more significant error.
    pub fn get_failed_root_destinations(&self) -> &Vec<(&'a DestinationConfig, &'a Box<dyn Error>)> {
        &self.failed_roots
    }
}