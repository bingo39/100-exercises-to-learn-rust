// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.
use std::ops::Add;

/* 1.assert_eq! 宏底层就是用 == 运算符比较两个值，而 == 需要 PartialEq trait。没有实现这个 trait，编译器就不知道怎么比较两个 WrappingU32 是否相等
要让自定义类型支持 ==，就必须实现 PartialEq 
2. assert_eq! 在断言失败时需要打印值的内容，所以要求类型必须实现 Debug trait*/
#[derive(Copy, Clone,PartialEq,Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        //等价于：WrappingU32 { value: value }
        /*{ value } 是字段初始化简写。当变量名和字段名相同时，可以省略冒号和值。
        value: value 可以简写为 value。这是 Rust 的语法糖，和 JS 里的对象属性简写类似。 */
        Self { value }
    }
}

impl Add for WrappingU32{
    type Output = Self;
    fn add(self,other:Self) -> self:: WrappingU32{
        WrappingU32{
            value:self.value.wrapping_add(other.value)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        // u32 最大值的,测试里 42 + 31 + 31 + u32::MAX 在普通 u32 运算下会溢出
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
