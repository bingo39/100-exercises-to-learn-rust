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

    // `&`指针本身大小就是4/8个字节（32/64计算机的最大寻址空间）
    // `size_of::<&type>()`，无论type是什么，打印的只是指针本身大小（最大寻址空间）
    #[test]
    fn size_test(){
        println!("u16的长度 {}",size_of::<&u16>());
        println!("u64的长度 {}",size_of::<&mut u64>());
        println!("u64的长度 {}",size_of::<&u64>());
    }

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(),8);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(),8);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
