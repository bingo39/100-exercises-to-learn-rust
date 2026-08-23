use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Implement the `IntoIterator` trait for `&TicketStore` so that the test compiles and passes.
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

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

// <'a> :	声明一个生命周期参数 'a
// &'a TicketStore:	对 TicketStore 的引用，生命周期为 'a
impl<'a> IntoIterator for &'a TicketStore {
    // 每次迭代产出一个 &'a Ticket——对 Ticket 的引用，生命周期与 &'a TicketStore 绑定
    type Item = &'a Ticket;
    // 这是 Vec::iter() 的返回类型。Iter<'a, T> 的 'a 表示：这个迭代器产出的引用，至少能活 'a 这么久。
    type IntoIter = std::slice::Iter<'a,Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        //注意这里的 self 类型是 &'a TicketStore（不是 TicketStore），所以不会消费所有权——和 05 的 iter(&self) 本质相同。
        self.tickets.iter()
    }
}
/*
核心约束：Iter 不能比 &TicketStore 活得更久。'a 把它们绑在一起，编译器据此确保不会出现悬垂引用
*/

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
        let tickets2: Vec<&Ticket> = (&store).into_iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
