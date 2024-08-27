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