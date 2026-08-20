# 概述
## 学习目标
+ 栈分配数组
    + `Vec`, 可增长的数组类型
+ `Iterator` 和 `IntoIterator` ，用于遍历集合
+ `Slices` 切片（ &[T] ），用于处理集合的一部分
+ `Lifetimes`生命周期，用于描述引用的有效时长
+ `HashMap` 和 `BTreeMap` ，两种键值数据结构
+ `Eq` 和 `Hash` ，用于在 HashMap 中比较键
+ `Ord` 和 `PartialOrd` ，用于与 BTreeMap 合作
+ `Index` 和 `IndexMut` ，用于访问集合中的元素


#	练习	核心知识点	简述
00	intro	热身	填空完成欢迎字符串
01	arrays	数组	用定长数组存储一周温度数据，通过 Weekday 枚举索引
02	vec	Vec 动态数组	用 Vec 做斐波那契数列的记忆化计算
03	resizing	Vec 扩容机制	理解 Vec 超出容量后的自动扩容策略
04	iterators	IntoIterator trait	为 TicketStore 实现 IntoIterator，支持 for 循环遍历
05	iter	iter() 方法	提供 iter() 返回 &Ticket 引用的迭代器
06	lifetimes	生命周期	为 &TicketStore 实现 IntoIterator，处理引用的生命周期
07	combinators	迭代器组合器	用 .filter() 等组合器筛选 Status::ToDo 的工单
08	impl_trait	impl Trait 语法	返回 impl Iterator 类型的惰性迭代器
09	impl_trait_2	impl Trait vs 泛型	将 impl Trait 参数改为显式泛型参数，理解两者区别
10	slices	切片 (&[T])	定义函数接收 &[u32] 切片引用并求和
11	mutable_slices	可变切片 (&mut [T])	对可变切片原地修改（平方运算）
12	two_states	TicketDraft / Ticket 双状态	引入 TicketId 和 TicketDraft，add_ticket 返回唯一 ID
13	index	Index trait	实现 Index<TicketId>，支持 store[id] 语法访问
14	index_mut	IndexMut trait	实现 IndexMut<TicketId>，支持 &mut store[id] 修改工单状态
15	hashmap	HashMap	将底层存储从 Vec 换成 HashMap<TicketId, Ticket>，需实现 Hash/Eq
16	btreemap	BTreeMap	将存储换成 BTreeMap，实现有序迭代的 IntoIterator

三大阶段解读
阶段一：集合基础（01-03）
从固定大小的数组到动态 Vec，再到理解 Vec 的扩容行为。这是 Rust 集合操作的入门。
01_arrays 用枚举做数组索引是个巧妙的设计——把 Weekday 映射到 [Option<i32>; 7]。

阶段二：迭代器体系（04-09）
这是本组的核心重头戏。TicketStore 从一个简单的 Vec<Ticket> 包装器开始，逐步获得迭代能力：

先实现 IntoIterator（拥有所有权遍历） → `04_iterators`
再实现 iter()（借用遍历） → `05_iter`
再为 &TicketStore 实现 IntoIterator（生命周期的实战） → `06_lifetimes`
用组合器做过滤 → `07_combinators`
用 impl Trait 返回惰性迭代器 → `08_impl_trait`
理解 impl Trait 与泛型参数的等价转换 → `09_impl_trait_2`
阶段三：真实系统构建（10-16）
开始构建一个真正可用的 Ticket 管理系统：

切片作为函数参数的通用抽象 → `10_slices / 11_mutable_slices`
引入 TicketDraft（草稿）和 Ticket（正式工单）的双状态设计模式 → `12_two_states`
通过 Index/IndexMut 让 store[id] 语法生效 → `13_index / 14_index_mut`
最终用 HashMap 和 BTreeMap 替换 Vec 做底层存储 → `15_hashmap / 16_btreemap`
关键设计亮点
渐进式架构演进：TicketStore 的内部存储从 Vec → HashMap → BTreeMap 逐步替换,同一接口不同底层的权衡。
**TicketDraft vs Ticket：**
`12_two_states` 引入的类型状态模式（typestate pattern）——草稿没有 ID 和状态，提交后才变成正式工单。这是 Rust 中非常惯用的封装手法。
impl Trait 的双面性：
`08_impl_trait` 用它做返回值（隐藏迭代器具体类型），
`09_impl_trait_2` 把它改写成泛型参数——让你体会两者的等价与差异。