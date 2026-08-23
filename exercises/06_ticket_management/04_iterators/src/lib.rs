use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
//   You want to *delegate* the iteration to the `Vec<Ticket>` field in `TicketStore`.
//   Look at the standard library documentation for `Vec` to find the right type
//   to return from `into_iter`.


/*
  TODO: 开始搭建我们的工单存储系统！
  第一个任务：在 `TicketStore` 上实现 `IntoIterator`，
  使得可以使用 `for` 循环遍历它包含的所有工单。
  提示：这种情况下你不需要自己去实现 `Iterator` trait。
  你应该把迭代*委托*给 `TicketStore` 中的 `Vec<Ticket>` 字段。
  查阅标准库文档中 `Vec` 的部分，找出 `into_iter` 应该返回的正确类型。
 */

 // 数据结构
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

//TicketStore的接口 
impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

impl IntoIterator for  TicketStore{
    //`into_iter` ：return right type
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        //解释注释部分内容:
        // 实现的是 TicketStore 的迭代，但迭代逻辑得让内部的 Vec 来干——所以必须先访问 self.tickets，再调它的 into_iter
        // 假如：TicketStore::into_iter，就是调用实现的into_iter,即无限调用自己
        self.tickets.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
