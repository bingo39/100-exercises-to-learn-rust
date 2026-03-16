> 练习04_visibility中，为什么"TODO"要求注释掉那两行代码以为了让编译能进行，或者说那两行代码留下来的意义是什么

**为什么要注释掉这两行？**
这个练习的核心是学习 可见性（visibility） 和 封装（encapsulation）。代码故意写了两段"不应该编译通过"的代码，目的是：
1.第一行错误代码（行56）：
```rust
assert_eq!(ticket.description, "A description");
//         ^^^^^^^^^^^^^^^^^^
//         试图直接访问私有字段
```
**目的:**
+ 当你把` Ticket `结构体设为 `pub` 后，虽然可以在外部使用它
+ 但它的字段默认还是私有的，不能直接访问
+ <font color=red>封装</font>：外部代码不能绕过构造函数直接操作内部数据

2. **第二行错误代码（行70-74）：**
```rust
let ticket = Ticket {
    title: "A title".into(),
    description: "A description".into(),
    status: "To-Do".into(),
};
// 试图用字面量语法直接构造 Ticket
```
**目的：**
+ 即使你能访问` Ticket `类型，也不能绕过` new `方法直接创建实例
+ 这确保了所有` Ticket `实例都经过了` new` 里的验证逻辑
+ 保证了数据一致性：不可能创建出非法的` Ticket`