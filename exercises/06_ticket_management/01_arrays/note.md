# 对比`day as usize` & `match`
## 代码比较
### 用`match`
```rust
impl Weekday {
    fn as_index(&self) -> usize {
        match self {
            Weekday::Monday    => 0,
            Weekday::Tuesday   => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday  => 3,
            Weekday::Friday    => 4,
            Weekday::Saturday  => 5,
            Weekday::Sunday    => 6,
        }
    }
}

pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
    self.temperatures[day.as_index()]
}
```
### [derive(Copy, Clone)] + as usize 版本
```rust
// 增加宏
#[derive(Copy, Clone)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

 pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        self.temperatures[day as usize]
    }
```
## 原因
**原理：**为什么 as usize 能用？
Rust 中，无字段枚举（field-less enum）的每个变体都有一个隐式的判别值（discriminant），默认从 0 开始递增：
| 变体 | 判断值 |
| :--- | :---: |
| Monday | 0 |
| Tuesday | 1 |
|Wednesday|	2 |
|... |	...|
|Sunday|	6|

当枚举实现了 Copy 时，day as usize 就是把判别值直接转为整数。由于 Monday 是第一个变体，判别值恰好是 0，与数组索引完美对齐。

> 拓展：为什么要 Copy？ \
> as 转换会消耗值。如果 Weekday 不是 Copy，day as usize 会 move 掉 day，导致 get_temperature(&self, day: Weekday) 签名下 day 在函数结束后被消耗——虽然这里不影响编译，但 set_temperature 接收的是值类型，用完就没了。加上 Copy 后，as usize 只是复制判别值，day 本身不受影响。

> ❗注意：\
如果哪天把 Weekday 的变体顺序改了，as usize 版本会静默出错——Monday 可能映射到 3，而 match 版本则不受影响。这就是我之前说"手写 match 更稳妥"的原因。不过在当前这个练习中，变体顺序天然就是 0..6，as usize 是完全安全的。





