// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//TODO:让字符串和'String'能转换成'Status'类型
//  The parsing should be case-insensitive.
// 解析应该不区分大小写

use std::fmt;
use std::convert::TryFrom;

#[derive(Debug)]
enum ParseStatusError {
    //#[error("Invalid status:'{0}'")]
   InvalidInput(String)
}
impl fmt::Display for ParseStatusError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ParseStatusError::InvalidInput(s)=>{
                write!(f,"输入无效的status:'{}'",s)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

//实现Stirng转换为status类型
impl TryFrom<String> for Status{
    type Error = ParseStatusError;

    fn try_from(value:String) -> Result<Self,Self::Error>{
        match value.to_lowercase().as_str(){
            // to_lowercase:解析不区分大小写；as_str:转变为字符串
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" =>Ok(Status::Done),
            _ => Err(ParseStatusError::InvalidInput(value)),
        }
    }
}

// 实现字符串转换为Status类型
impl TryFrom<&str> for Status{
    type Error = ParseStatusError;

    fn try_from(value:&str) -> Result<Self,Self::Error>{
        // match value.to_lowercase().as_str(){
        //     "ToDO" => Ok(Status::ToDo),
        //     "InProgress" => Ok(Status::InProgress),
        //     "Done" =>Ok(Done),
        //     _ => Err(ParseStatusError::InvalidInput(value.to_string())),
        // }

        // 复用 String 的实现
        Status::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDo".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
