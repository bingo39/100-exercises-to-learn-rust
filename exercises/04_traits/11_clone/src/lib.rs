// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let t=ticket.clone();
    //ticket 的所有权被使用了两次;
    /*第一，ticket 作为元组的第一个元素被使用，ticket 的所有权被移入了元组。
     * 第二，ticket.summary() 又用了一次 ticket。导致报错 */
    (ticket, t.summary())
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}

