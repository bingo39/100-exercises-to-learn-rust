// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
   // todo!()
  if title.is_empty() || title.len()>50{
    Ticket::new(title, description, status).unwrap()
  }else if description.is_empty()|| description.len()>500{
      Ticket::new(
            title,
            "Description not provided".to_string(),
            status,
        )
        .unwrap()
  } else {
      Ticket::new(title, description, status).unwrap()
  }

}
/*
1.简便写法：
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
  let description = if description.is_empty()|| description.len()>500{
          "Description not provided".to_string()  
  }else{
       description 
  }；
Ticket::new(title, description, status).unwrap()
}
2.显示处理
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
  match Ticket::new(title.clone(), description, status.clone()){
    Ok(ticket) => ticket;
    Err(error) => {
         if error.contains("Description") {
            Ticket::new(title, "Description not provided".to_string(), status).unwrap()
    }else{
       panic!("{}", error);
    }
  }
  }；
  补充：
  contains() 是 String 的标准库方法，error是String
  error.contains("Description") 的逻辑就是：错误信息里带 "Description" → 说明是描述的问题 → 用默认值替代；否则 → 是标题的问题 → panic。
*/

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
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
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
