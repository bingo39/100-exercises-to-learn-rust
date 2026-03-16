# 两端代码思考与对比
+ 答案①：
```RUST
pub fn title(self) -> String {
self.title
}

Copy code
    pub fn description(self) -\> String {
        self.description
    }

    pub fn status(self) -\> String {
        self.status
    }
```
+ 答案②：
```rust
pub fn title(&self) -> &str {
            &self.title
        }
 
        pub fn description(&self) -> &str {
            &self.description
        }
 
        pub fn status(&self) -> &str {
            &self.status
        }
```
> 两段代码虽然都能通过编译，但答案②才是正确做法
## 对比分析：
| 维度      | 方案一 |  方案二
| ----------- | ----------- | --------|
| 能否编译      | 能       | 能       |
| 能否通过测试   | 能       | 能       |
| 性能      | ❌每次调用都转移所有权 | ✅ 零成本借用 |
| 可用性    | ❌只能调用一次    |✅ 可以多次调用 |
|内存消耗   | ❌消耗对象    | ✅ 不消耗对象 |
|rust惯用法 |  ❌不符合     | ✅ 标准getter模式 |

## 问题演示
+ 为什么方案一可以通过代码测试？
测试代码：
```rust
#[test]
fn title() {
    let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
    assert_eq!(ticket.title(), "A title");
    // 测试到此结束，ticket 被消耗了也无所谓
}
```
每个测试都是独立的，只调用一次 getter，所以代码能通过
但在真实场景中：
```rust
// 使用方案一
let ticket = Ticket::new("标题".into(), "描述".into(), "To-Do".into());
 
let t = ticket.title();  // ← ticket 被消耗（move）
// let d = ticket.description();  // ❌ 编译错误！ticket 已经不存在了
```
```rust
// 使用方案二
let ticket = Ticket::new("标题".into(), "描述".into(), "To-Do".into());
 
let t = ticket.title();        // ✅ 借用
let d = ticket.description();  // ✅ 还能用
let s = ticket.status();       // ✅ 还能用
 
// 甚至可以多次访问同一个字段
for _ in 0..10 {
    println!("{}", ticket.title());  // ✅ 可以循环调用
}
```

## 性能对比
+ 方案一：
```rust
pub fn title(self) -> String {
    self.title  // 转移所有权，整个 Ticket 对象被销毁
}
 
// 调用成本：
// 1. 转移整个 Ticket 结构体（包括 3 个 String）
// 2. 提取 title 字段
// 3. 销毁剩余字段（description 和 status 被 drop）
// 4. 返回 String
```
+ 方案二：
```rust
pub fn title(&self) -> &str {
    &self.title  // 只是返回一个指针
}
 
// 调用成本：
// 1. 传递一个引用（8 字节指针）
// 2. 返回一个切片引用（16 字节：指针 + 长度）
// 零堆分配，零复制
```
+ 测试
```rust
// 方案一
let ticket = create_ticket();
let _ = ticket.title();  // 消耗整个对象
// 成本：~100-200 纳秒（取决于字符串长度）
 
// 方案二
let ticket = create_ticket();
let _ = ticket.title();  // 只是借用
let _ = ticket.title();  // 可以继续用
// 成本：~1-5 纳秒（几乎零成本）
```
## Getter 方法的设计原则
Rust社区的最佳实践：
```Rust
// ✅ 标准 getter 模式
pub fn field(&self) -> &FieldType {
    &self.field
}
 
// ❌ 反模式：消耗对象的 getter
pub fn field(self) -> FieldType {
    self.field
}
 
// ⚠️ 可接受但不推荐：克隆数据
pub fn field(&self) -> FieldType {
    self.field.clone()
}
```
> 补充：什么时候用`self`（消耗对象）
> 只有明确知道`self`会被消耗，在`转换方法`中使用，通常命名`into_xxx`



