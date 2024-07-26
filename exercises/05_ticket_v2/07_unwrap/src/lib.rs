use core::fmt;
use std::error::Error;

// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(ticket) => ticket,
        Err(TicketError::EmptyDescription) | Err(TicketError::DescriptionTooLong) => {
            Ticket::new(title, "Description not provided".into(), status).unwrap()
        }
        Err(error) => panic!("{}", error),
    }
}

#[derive(Debug)]
enum TicketError {
    EmptyTitle,
    TitleTooLong,
    EmptyDescription,
    DescriptionTooLong,
}

impl fmt::Display for TicketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TicketError::EmptyTitle => "Title cannot be empty",
                TicketError::TitleTooLong => "Title cannot be longer than 50 bytes",
                TicketError::EmptyDescription => "Description cannot be empty",
                TicketError::DescriptionTooLong => "Description cannot be longer than 500 bytes",
            }
        )
    }
}

impl Error for TicketError {}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, TicketError> {
        if title.is_empty() {
            return Err(TicketError::EmptyTitle);
        }
        if title.len() > 50 {
            return Err(TicketError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketError::EmptyDescription);
        }
        if description.len() > 500 {
            return Err(TicketError::DescriptionTooLong);
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
