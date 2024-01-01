- `cargo install diesel_cli --no-default-features --features sqlite`

上面的指令出现问题，然后用这个解决： `sudo apt install libsqlite3-dev`

> Diesel是一种用于Rust编程语言的ORM（对象关系映射）框架。ORM框架旨在简化与数据库的交互，允许开发者使用编程语言的对象来表示数据库中的表和数据。Diesel提供了类型安全的查询和操作数据库的功能，它能够在编译时捕获一些常见的数据库错误，从而提高代码的可靠性。Diesel还支持多种数据库引擎，包括但不限于SQLite、MySQL和PostgreSQL。通过使用Diesel，开发者可以更容易地在Rust中构建和管理数据库相关的应用程序。

- 安装sqlite：`sudo apt install sqlite3`

- 生成数据库：`diesel setup --database-url=database.sqlite`

- 生成up, down文件：`diesel migration generate create_products`

- 执行数据库语句：diesel migration run --database-url=database.sqlite