# rCollection
rust小型项目合集，个人入门
### tinygrep *0.1.0*
查询文件内容，cli，入参，大小写不敏感  
```
cargo run -- how poem.txt
```
### single-thread server
支持get请求
### async server
支持get请求，异步处理
### multi-thread server
基本线程池实现，请求应答
### tinyRedis
set/get命令、服务端客户端、请求缓存
后续可以新增：分片锁+读写锁来优化
这里的测试有点问题，后续研究一下
### miniTokio
简化版异步tokio
通过channel来进行调度、唤醒机制
### dataStructure
```
(1)stack.rs        mem::replace
(2)rc_stack.rs     Rc
(3)queue.rs        裸指针
(4)rc_queue.rs     Rc+RefCell
```
### 笔记
<aside>
👉常用：

as_deref   `Option<Box<Node>> ⇒ Option<&Node>`

as_ref  `Option<Box<Node>> ⇒ Option<&Box<Node>>`

map `fn map<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> U`

and_then `fn and_then<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> Option<U>`


take() `Option 是 Some，它会将 Option 的值取出，并将原 Option 设置为 None`

unwrap()  `对于Option<T>或者Result<T>返回T或者panic`

*mut Node<T> .as_ref() →`Option<&Node<T>>  裸指针在unsafe块中`

</aside>
<aside>
👉Box

Box::into_raw(b:Box<T>) → *mut T    `拿走Box所有权，返回一个裸指针`

Box::from_raw(b: *mut T ) → Box<T>  `将裸指针转回到Box`

</aside>

<aside>
👉RefCell

`RefCell`实现编译期可变、不可变引用共存

into_inner()  `fn into_inner(self)→ T ，消费掉RefCell并返回内部的值`

Ref::map   `Ref::map(node.borrow(),|node:&Node<T>| &node.elem)   ⇒ Ref<T>`

Ref::map_split `允许我们同时持有对结构多个不同部分的引用，并且可以同时使用这两个引用。`


borrow_mut() `let mut balance = self.balance.borrow_mut();  *balance += amount; 针对不可变引用，转变为可变引用，从&RefCell<T>->&mut T ，之后通过*T来取值`

</aside>

<aside>
👉Rc

主要实现1vN的引用情况

Rc::clone() `是通过引用计数，在底层数据之上进行重用`

Rc::as_ref() ⇒ Rc<T> → &T `只是借用`

try_unwrap `尝试获取Rc的完全所有权，只有当Rc的引用计数为1才会成功`

rc.borrow() `//不可变借用内部RefCell`

rc.borrow_mut() `//可变借用内部RefCell`

</aside>