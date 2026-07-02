# 对比“match”和“switch-case”
> ‼`match` 比 `switch-case` 强得多。简单对比：

### 相似处

都是拿一个值去跟多个分支做匹配，命中就执行对应逻辑：

```java
// Java switch
switch (status) {
    case DONE: return true;
    case IN_PROGRESS: return false;
}
```

```rust
// Rust match
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

### 关键区别——match 更强的地方

**1. 穷举性：编译器逼你处理所有情况**

`switch` 漏了 case 不会报错，`match` 漏一个变体直接编译失败。你不会再踩"忘了处理某个分支"的坑。

**2. 模式匹配：不只是比值，还能解构**

```rust
match message {
    Message::Move { x, y } => move_to(x, y),  // 直接解构出字段
    Message::Write(text) => println!("{}", text), // 提取变体携带的数据
    Message::Quit => quit(),
}
```

`switch` 只能比标量值，`match` 能拆解嵌套结构。这是本质差异。

**3. 没有穿透（fall-through）**

`switch` 不加 `break` 会穿透到下一个 case，这是无数 bug 的来源。`match` 每个分支天然隔离，不存在穿透问题。

**4. 是表达式，有返回值**

```rust
let is_done = match self {
    Status::Done => true,
    Status::InProgress | Status::ToDo => false,
};
```

`switch` 是语句，`match` 是表达式——可以直接赋值。

### 一句话总结

`switch-case` 是 `match` 的远房穷亲戚。`match` = `switch` + 穷举检查 + 模式解构 + 表达式返回 + 无穿透。把它当成加强版 switch 来理解没问题，但别低估了它。

更多关于 `match` 和模式匹配：[枚举错误处理](12-error-handling-with-enums)