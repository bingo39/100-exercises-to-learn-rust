## 关于iter()方法逐项拆解
### 1. &self — 借用，不消费
```rust
pub fn iter(&self) -> ...
```
对比 04 练习的 into_iter(self)：
|   |   |   |
|--|--|--|
| |into_iter(self) |	iter(&self) |
|self 形式|	值（所有权）	|不可变引用
|调用后store 还能用？    |	❌ 被消费	| ✅ 仍然可用

测试正是验证这一点——连续调了两次 store.iter()：
```rust
let tickets: Vec<&Ticket> = store.iter().collect();
let tickets2: Vec<&Ticket> = store.iter().collect();  // ← store 没被吃掉
assert_eq!(tickets, tickets2);
```
### 2. 返回类型 std::slice::Iter<'_, Ticket>
这是 Vec::iter() 的返回类型。拆开看：
```bash
std::slice::Iter<'_, Ticket>
  │       │     │    │
  │       │     │    └─ 迭代产出的元素类型
  │       │     └─ 生命周期参数（绑定到 &self）
  │       └─ 切片上的迭代器结构体
  └─ 标准库 slice 模块
```
关键在那个 '_——它是生命周期的省略写法，等价于：
```rust
pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, Ticket>
```
迭代器借用 self，生命周期与 &self 绑定。只要迭代器活着，TicketStore 就不能被修改或销毁。编译器通过生命周期确保这一点。

### 3. 委托 self.tickets.iter()
和 04 练习一样的思路——自己不造轮子：
```rust
fn iter(&self) -> std::slice::Iter<'_, Ticket> {
    self.tickets.iter()
    //  ^^^^^^^ Vec<Ticket>，调的是 Vec 的 iter
}
```
### 拓展
 > -- 等价于：impl IntoIterator for &TicketStore
iter() 方法和 IntoIterator for &T 是两套不同的接口：

|   |   |   |
|-- |-- |-- |
|接口	|调用方式|	练习|
|iter(&self) 方法	|store.iter()|	05|
IntoIterator for &TicketStore	|(&store).into_iter() 或 for t in &store|	06|

两者效果相同，但 IntoIterator 让 &store 可以直接放进 for 循环，而 iter() 是显式调用。Rust 标准库中 Vec 两者都实现了，所以写 vec.iter() 和 (&vec).into_iter() 效果一样。