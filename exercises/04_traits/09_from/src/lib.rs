// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

// From的作用：From 的作用就是实现类型之间的安全转换；for U 表示"从类型 T 转换成类型 U"的能力。你实现一个 from 方法，接收 T 类型的值，返回 U 类型的值
pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }  
}

fn example() {
    let wrapping: WrappingU32 = 42.into();  //实现了From,就自动获得了into.()
    let wrapping = WrappingU32::from(42);
}
