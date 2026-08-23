// TODO: Implement the `to_dos` method. It must return a `Vec` of references to the tickets
//  in `TicketStore` with status set to `Status::ToDo`.

/*
要求：
TODO: 实现 `to_dos` 方法。它必须返回一个 Vec，其中包含对 TicketStore 中状态为 Status::ToDo 的工单的引用。

*/
use ticket_fields::{TicketDescription, TicketTitle};

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
    // rust没有new关键字，常用的构造模式构建新类型
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    // 要求：过滤status = Status::ToDo 的工单
    pub fn to_dos(&self) -> Vec<&Ticket>{
        // 管道函数
        self.tickets    //集合
            .iter()     //迭代器  (前面都是Combinators常规引用)
            .filter(|tickets|tickets.status == Status::ToDo)   //过滤条件
            .collect()  //返回Vec,作集合
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn todos() {
        let mut store = TicketStore::new();

        let todo = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,   // <- TODO
        };
        store.add_ticket(todo.clone());

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,     // <- InProgress
        };
        store.add_ticket(ticket);

        let todos: Vec<&Ticket> = store.to_dos();  // Vec<&Ticket>,引用不是clone
        // 筛选条件：len() = 1;对应status=TODO
        assert_eq!(todos.len(), 1); 
        assert_eq!(todos[0], &todo);
    }
}
