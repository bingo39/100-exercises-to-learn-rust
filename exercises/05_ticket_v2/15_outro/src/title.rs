// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

/*
TODO: 为 `TicketTitle` 类型实现 `TryFrom<String>` 和 `TryFrom<&str>`
要求标题不为空且不超过 50 字节。
同时实现使测试通过的必要 trait。
*/

use thiserror::Error;
#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

#[derive(Debug,Error)]
pub enum TicketTitleError  {
    #[error("The title cannot be empty")]
    CannotEmpty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong
}


impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty(){
           return Err(TicketTitleError::CannotEmpty);
        }else if value.len() > 50{
            return Err(TicketTitleError::TooLong);
        }else{
            Ok(TicketTitle(value))
        }
    }
}

impl TryFrom <&str> for TicketTitle{
    type Error =  TicketTitleError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TicketTitle::try_from(value.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
