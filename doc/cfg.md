
### cfg配置结构

#### 需要实现的功能

1. 全局变量
2. 局部变量
3. 可执行func
4. config
5. 启动服务
6. 数据表



-----------------


### 配置格式


#### 全局变量

```
[[global.var]]
db_path = "./db"
```

#### 局部变量

```
[[local.var]]
b = 2
```

#### 表

```

[table.user]
type = "file"
path = "./path"

[table.user]
type = {file = true, path = "./path"}
key = "int"
value = {user = "string", age = "int", name = "string"}

```

#### 服务

```
[server.web]
start = "web.start()"
stop = "web.stop()"
restart = "web.restart()"

[server.db]
start = "db.start()"
stop = "db.stop()"
restart = "db.restart()"
```

#### config

```
[config.p2p]
port = 3303
type = "tcp"

[config.web]
port = 80

[config.db]
path = "$db_path"
allow_2pc = true
fun = """fn test() {
    println!(\"test func\");
}"""

```

#### fun

```
[[func]]
fn1 = """fn test() {
    println!(\"test func\");
}"""

```

#### res

```
[res.web.www.xx.com]
root = "./html"
index = ["index.html", "index.htm"]

[res.web.192.168.1.1]
root = "./html"
index = ["index.html", "index.htm"]

[res.web.*]
root = "./html"
index = ["index.html", "index.htm"]

```

#### port

```
[port.http."user/login"]
fun = login

```