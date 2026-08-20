use core::error;

use crate::TicketNewError::{DescriptionEerror, TitleError};

/*练习要求：
把错误类型从 String 升级为枚举 TicketNewError——这正是上次 contains("Description") 那种尴尬写法的解决方案。 */
// TODO: Use two variants, one for a title error and one for a description error.
//   Each variant should contain a string with the explanation of what went wrong exactly.
//   You'll have to update the implementation of `Ticket::new` as well.
#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionEerror(String)
}

// TODO: `easy_ticket` should panic when the title is invalid, using he error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    //todo!()
    match Ticket::new(title.clone(), description, status.clone()){
        Ok(ticket)=>ticket,
        Err(TicketNewError::TitleError(msg))=>panic!("{}",msg),
        Err(TicketNewError::DescriptionEerror(msg1))=>{
            Ticket::new(
                title,//common::valid_title()
                 "Description not provided".to_string(), 
                 status,
                ).unwrap()
        }
    }
}
/*
只调用一次  Ticket::new ：
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    let description = if description.is_empty() || description.len() > 500 {
        "Description not provided".to_string()
    } else {
        description
    };
    match Ticket::new(title, description, status) {
        Ok(ticket) => ticket,
        Err(TicketNewError::TitleError(msg)) => panic!("{msg}"),
        Err(TicketNewError::DescriptionError(_)) => unreachable!(),
    }
}
    不需要 clone，只调一次 new，逻辑也更清晰。避免所有权问题的最佳策略永远是重构代码让它不发生
*/

#[derive(Debug, PartialEq)]
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
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        //Error要求TicketNewError类型，而不是String
        if title.is_empty() {
            return Err(TitleError("Title cannot be empty".to_string()));
        }
        if title.len() > 50 {
            return Err(TitleError("Title cannot be longer than 50 bytes".to_string()));
        }
        if description.is_empty() {
            return Err(DescriptionEerror("Description cannot be empty".to_string()));
        }
        if description.len() > 500 {
            return Err(DescriptionEerror("Description cannot be longer than 500 bytes".to_string()));
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
