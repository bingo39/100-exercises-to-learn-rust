# 随手记
## 解释 pub fn new() -> Self
```rust
pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }
```

|   |   |
|-- |-- |
|部分|	含义|
|pub fn new()|	公开的关联函数，Rust 中约定用作"构造器"|
|-> Self	|   返回类型是所在 impl 块的类型，即 TicketStore|
|Self { ... }|	构造 TicketStore 实例，等价于 TicketStore { ... }|
|tickets: Vec::new()|	将 tickets 字段初始化为一个空的 Vec|

## 总结
> Rust 中最常见的构造器模式——没有 new 关键字，用约定俗成的 ::new() 函数代替

## 拓展：
`Vec::new` 等价于 `TicketStore { tickets: vec![] }`

## Combinators
> Iterators 功能很强大，有可以用来以各种方式转换、过滤和组合迭代器的方法；这些方法用在一起就是Combinators
## 闭包
> 语法：|args| body 语法定义，其中 args 是参数， body 是函数体
```rust
 .filter(|tickets|tickets.status == Status::ToDo)    // filter就是闭包函数，里面就是闭包本身
 ```
## 拓展👀
📕 [Combinators可用的组合函数文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)

