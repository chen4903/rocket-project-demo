# Brief

Rocket框架入门，CURD项目，步骤如下：

1. [建立路由，完成基础API调用](https://www.bilibili.com/video/BV1QV411H7jB?p=2&vd_source=0cc0401ee122346d6680e90658b0ed1a)
2. [BasicAuth需求完成](https://www.bilibili.com/video/BV1QV411H7jB?p=3&vd_source=0cc0401ee122346d6680e90658b0ed1a)
3. [引入diesel连接数据库](https://www.bilibili.com/video/BV1QV411H7jB?p=4)
4. [完成基本的curd](https://www.bilibili.com/video/BV1QV411H7jB?p=5&vd_source=0cc0401ee122346d6680e90658b0ed1a)
5. [auth重构](https://www.bilibili.com/video/BV1RB4y1N7Zx/?spm_id_from=333.999.0.0&vd_source=0cc0401ee122346d6680e90658b0ed1a)
6. [用repository实现重构](https://www.bilibili.com/video/BV1RB4y1N7Zx?p=2&vd_source=0cc0401ee122346d6680e90658b0ed1a)
7. [错误处理](https://www.bilibili.com/video/BV1RB4y1N7Zx?p=3&vd_source=0cc0401ee122346d6680e90658b0ed1a)

# Usage

学习过程中所需要用到的指令：

安装sqlite：`sudo apt install sqlite3`

- `cargo install diesel_cli --no-default-features --features sqlite`

> 上面的指令出现问题，然后用这个解决： `sudo apt install libsqlite3-dev`
>
> Diesel是一种用于Rust编程语言的ORM（对象关系映射）框架。ORM框架旨在简化与数据库的交互，允许开发者使用编程语言的对象来表示数据库中的表和数据。Diesel提供了类型安全的查询和操作数据库的功能，它能够在编译时捕获一些常见的数据库错误，从而提高代码的可靠性。Diesel还支持多种数据库引擎，包括但不限于SQLite、MySQL和PostgreSQL。通过使用Diesel，开发者可以更容易地在Rust中构建和管理数据库相关的应用程序。

- 生成数据库：`diesel setup --database-url=database.sqlite`

- 生成up, down文件：`diesel migration generate create_products`

- 执行数据库语句：`diesel migration run --database-url=database.sqlite`

实现的效果如下：

![](https://moe.photo/images/2024/01/01/image-20240101181739145.png)