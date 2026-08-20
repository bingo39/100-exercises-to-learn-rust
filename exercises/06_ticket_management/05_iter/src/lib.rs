use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Provide an `iter` method that returns an iterator over `&Ticket` items.
//
// Hint: just like in the previous exercise, you want to delegate the iteration to
//   the `Vec<Ticket>` field in `TicketStore`. Look at the standard library documentation
//   for `Vec` to find the right type to return from `iter`.

/*
TODO: 提供一个 `iter` 方法，返回一个对 `&Ticket` 项的迭代器。
提示：和上一个练习一样，你应该把迭代委托给`TicketStore` 中的 `Vec<Ticket>` 字段（tickets字段才对）
查阅标准库文档中 `Vec` 的部分，找出 `iter` 应该返回的正确类型。

文档：大多数集合都提供 .iter()方法，该方法返回一个迭代器，用于遍历集合的元素引用
 */

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self)->std::slice::Iter<Ticket>{
        self.tickets.iter()
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

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
