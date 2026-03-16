pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    // 打印测试用例
    #[test]
    fn print_test(){
        println!("size of 'String': {}",size_of::<String>());
        println!("size of 'Tiet': {}",size_of::<Ticket>());
        println!("size of '&str': {}",size_of::<&str>());
        println!("size of 'u64': {}",size_of::<u64>());
    }

    #[test]
    fn string_size() {
        // 一般编程语言‘String’占用24～32
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        // Ticket的strct有三个属性，都是String类型，占用24*3
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
