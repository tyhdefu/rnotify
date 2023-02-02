use crate::config::Config;
use crate::destination::routed_destination::{MessageRoutingBehaviour, RoutedDestination};
use crate::destination::message_condition_config::MessageCondition;
use crate::message::Message;
use crate::send_error::{SendError, SendErrors};
use serde::{Serialize, Deserialize};
use crate::send_error::borrowed::SendErrorBorrowed;
use crate::send_error::owned::SendErrorOwned;
use crate::send_error::reported::{ErrorReportSummary, ReportedSendError};

pub struct MessageRouter {
    destinations: Vec<Box<dyn RoutedDestination>>,
}

impl MessageRouter {
    pub fn empty() -> Self {
        Self {
            destinations: vec![],
        }
    }

    pub fn from_config(config: Config) -> Self {
        let destinations = config.take_destinations().into_iter()
            .map(|item| {
                let boxed: Box<dyn RoutedDestination> = Box::new(item);
                boxed
            })
            .collect();

        Self {
            destinations,
        }
    }

    pub fn add_destination(&mut self, destination: Box<dyn RoutedDestination>) {
        self.destinations.push(destination)
    }

    pub fn route<'a>(&self, message: &'a Message) -> Result<usize, SendErrors<'a>> {
        let mut errors: Vec<SendErrorBorrowed<'a>> = vec![];

        let mut sent_to_non_root_dest = false;

        let mut successful = 0;

        fn send_wrap<'a>(destination: &dyn RoutedDestination, message: &'a Message) -> Result<(), SendErrorBorrowed<'a>> {
            destination.send(message).map_err(|err| {
                SendErrorBorrowed::create(err, destination.get_id().to_owned(), message)
            })
        }

        fn send_to_dest<'a>(dest: &dyn RoutedDestination,
                            sent_to_non_root_dest: &mut bool,
                            successful: &mut usize,
                            errors: &mut Vec<SendErrorBorrowed<'a>>,
                            message: &'a Message) {
            match send_wrap(dest, message) {
                Ok(()) => {
                    if !dest.is_root() {
                        *sent_to_non_root_dest = true;
                    }
                    *successful += 1;
                }
                Err(send_err) => errors.push(send_err),
            }
        }

        for dest in self.destinations.iter()
            .filter(|dest| dest.get_routing_type().always_send_messages())
            .filter(|dest| dest.should_receive(message)) {

            send_to_dest(dest.as_ref(), &mut sent_to_non_root_dest, &mut successful, &mut errors, message);
        }

        if !sent_to_non_root_dest {
            // Find a drain.
            for dest in self.destinations.iter()
                .filter(|dest| dest.get_routing_type() == &MessageRoutingBehaviour::Drain)
                .filter(|dest| dest.should_receive(message)) {

                send_to_dest(dest.as_ref(), &mut sent_to_non_root_dest, &mut successful, &mut errors, message);
            }
        }

        if errors.is_empty() {
            return Ok(successful);
        }

        let root_destinations: Vec<_> = self.destinations.iter()
            .filter(|dest| dest.is_root())
            .map(|dest| dest.to_owned())
            .collect();

        let reported_errors = errors.into_iter().map(|error| {
            let report_message = error.create_report_message();
            let mut any_report_success = false;
            let mut report_fails = vec![];

            for root_dest in &root_destinations {
                match root_dest.send(&report_message) {
                    Ok(_) => {
                        any_report_success = true;
                    }
                    Err(send_err) => {
                        let send_err = SendErrorOwned::create(send_err, root_dest.get_id().to_owned(), report_message.clone());
                        report_fails.push(send_err);
                    }
                }
            }

            ReportedSendError::new(error, ErrorReportSummary::new(any_report_success, report_fails))
        }).collect();


        Err(SendErrors::new(message, reported_errors, successful))
    }

}

impl Default for MessageRouter {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RoutingInfo {
    // Whether errors with sending notifications will be reported to this destination.
    #[serde(default)]
    routing_type: MessageRoutingBehaviour,
    #[serde(skip_serializing_if = "Option::is_none")]
    applies_to: Option<MessageCondition>,
}

impl RoutingInfo {
    pub fn of(routing_type: MessageRoutingBehaviour) -> Self {
        Self {
            routing_type,
            applies_to: None
        }
    }

    pub fn root() -> Self {
        Self::of(MessageRoutingBehaviour::Root)
    }

    pub fn get_routing_behaviour(&self) -> &MessageRoutingBehaviour {
        &self.routing_type
    }

    pub fn applies_to(&self, message: &Message) -> bool {
        match &self.applies_to  {
            Some(filter) => filter.matches(message),
            None => true,
        }
    }
}