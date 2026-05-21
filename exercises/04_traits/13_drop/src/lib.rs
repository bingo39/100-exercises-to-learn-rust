// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.
// 实现一个"炸弹”类型：它被销毁时如果没被拆除就会panic
// 目的是理解 Drop trait 的工作机制，以及 Drop bomb 这种编程模式的用途
/*
1.先说 Drop 触发。当一个值离开作用域时，Rust 会自动调用它的 drop 方法。这是确定性的，
不像 Java 的 finalize 由垃圾回收器不确定地调用。
2.panic 本身和 Drop 没有必然关系，Drop 里通常做的是释放资源、清理内存这类事情。
通过在Drop trait中放入panic的目的只是为了在调用到drop时候，自然而然触发painc,从而达到“是否拆除炸弹的目的”（简单看待成if，bool的实现就好理解练习的目的）
 */
pub struct DropBomb{
    defused : bool
}

impl DropBomb {
    pub fn new()-> Self{
        DropBomb{defused:false}
    }

    pub fn defuse(&mut self){
        self.defused = true;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused{
            panic!("DropBomb was not defused")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 测试标记了#[should_panic],说明测试期望发生panic
    /*创建了 DropBomb 但什么都没做，当 bomb 离开作用域被销毁时，应该 panic。就像炸弹一样 */
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
