// TODO: Implement the `Error` trait for `TicketNewError` using `thiserror`.
//   We've changed the enum variants to be more specific, thus removing the need for storing
//   a `String` field into each variant.
//   You'll also have to add `thiserror` as a dependency in the `Cargo.toml` file.

use thiserror::Error;
#[derive(Error,Debug)]
enum TicketNewError {
    //thiserror 把手写的 Display + Error 实现压缩成一行注解
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}
/*
补充：
如果要实现 #[error("固定字符串")]
例如：#[derive(Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),     // 0号字段 = String
 
    #[error("{0}")]
    DescriptionError(String), // 0号字段 = String
}
需要    
变体得携带数据，即枚举要携带数据*/
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

// 实现TicketNewError trait
impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
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
    fn title_cannot_be_empty() {
        let err = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be empty");
    }

    #[test]
    fn description_cannot_be_empty() {
        let err = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Description cannot be empty");
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let err = Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn description_cannot_be_too_long() {
        let err = Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Description cannot be longer than 500 bytes"
        );
    }
}
