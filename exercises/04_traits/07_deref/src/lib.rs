// TODO: whenever `title` and `description` are returned via their accessor methods, they
//   should be normalized—i.e. leading and trailing whitespace should be removed.
//   There is a method in Rust's standard library that can help with this, but you won't
//   find it in the documentation for `String`.
//   Can you figure out where it is defined and how to use it?

/** trim()：用于去除字符串首尾的空白字符，定义在str上，但由于 String 实现了 Deref<Target = str>，
 * 所以可以在 String 的引用上直接调用 str 的方法。这就是 deref coercion 的威力。*/
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {

    pub fn title(&self) -> &str {
        self.title.trim()
    }

    pub fn description(&self) -> &str {
        self.description.trim()
    }
}
/**
 * 直接在 String 类型的 self.title 上调用了 trim() 方法，虽然 trim() 是 str 的方法而不是 String 的方法。
 * 这就是 deref coercion 在起作用：编译器看到 String 没有 trim() 方法，
 * 就会查看 String 实现的 Deref trait，发现 Target = str，然后去 str 上找 trim() 方法，找到了就自动转换
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
