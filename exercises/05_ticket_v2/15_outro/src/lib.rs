// TODO: you have something to do in each of the modules in this crate!
mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// and then re-export the public parts of those modules at the root of the crate.
//
// This hides the internal structure of the crate from your users, while still
// allowing you to organize your code however you like.
/*翻译：
 Rust 中一个常见的模式是将代码拆分到多个（私有）模块中，
 然后在 crate 的根层级重新导出这些模块的公共部分。这样既对用户隐藏了 crate 的内部结构，又允许你按照自己的喜好来组织代码。
*/
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// We no longer need to make the fields private!
// Since each field encapsulates its own validation logic, there is no risk of
// a user of `Ticket` modifying the fields in a way that would break the
// invariants of the struct.
//
// Careful though: if you had any invariants that spanned multiple fields, you
// would need to ensure that those invariants are still maintained and go back
// to making the fields private.

/*
 我们不再需要将字段设为私有！
 因为每个字段都封装了自己的验证逻辑，所以不存在`Ticket` 的用户以破坏结构不变量的方式修改字段的风险。
 但要注意：如果你有任何涉及多个字段的不变量约束，你需要确保这些不变量仍然被维护，并回退到将字段设为私有。
*/
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
