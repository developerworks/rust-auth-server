# Rust: 认证微服务

##### 开发环境配置(MacOS)

```
# Mysql 客户端库
brew install mysql-connector-c
# Diesel 命令行工具
cargo install diesel_cli
```

##### 过程

- 使用 Email 注册, 邮件接收验证连接
- 点击连接, 用相同的 Email 和密码注册
- 用 Email 和密码登录, 通过验证并接收认证 Cookie

#### 配置 Diesel

在项目根目录中的`.env`文件中设置数据库连接串.

```
echo DATABASE_URL=mysql://root:root@localhost/rust_auth_server > .env
```

创建数据库, 如果数据库不存在, 自动创建, 并在项目根中创建 `migrations` 目录.

```
diesel setup
```

生成移植文件

```
diesel migration generate users
diesel migration generate invitations
```

输出如下:

```
➜  rust-auth-server git:(main) ✗ diesel migration generate users
Creating migrations/2022-10-14-121759_users/up.sql
Creating migrations/2022-10-14-121759_users/down.sql
➜  rust-auth-server git:(main) ✗ diesel migration generate invitations
Creating migrations/2022-10-14-121837_invitations/up.sql
Creating migrations/2022-10-14-121837_invitations/down.sql
```


创建数据库表和 `src/schema.rs` 文件.

```
diesel migration run
```

## 参考连接

- https://stackoverflow.com/questions/72927992/auto-completion-not-working-for-rust-in-module-files-vs-code

## 测试

```
curl --request POST \
  --url http://localhost:3000/api/invitation \
  --header 'content-type: application/json' \
  --data '{"email":"developerworks@163.com"}'

curl --request POST \
  --url http://localhost:3000/api/register/5068a9fc-529d-4f26-93e7-a53bb855e249 \
  --header 'content-type: application/json' \
  --data '{"email":"developerworks@163.com", "password":"root"}'

curl -i --request POST \
  --url http://localhost:3000/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "developerworks@163.com","password":"root"}'
```
## 环境变量

在 `$PROJECT_ROOT/.env` 的环境变量

```
# 数据库连接字符串
DATABASE_URL=mysql://root:root@localhost/rust_auth_server
# SMTP 服务器账号
SMTP_ACCOUNT=""
# SMTP 服务器密码
SMTP_PASSWORD=""
# 发送者
MAIL_FROM=""
# 接受者
MAIL_TO=""
# 生成用户密码散列的加密键
SECRET_KEY="3VFQuA1LkV92OB8QAtcbnmnB4MMcKHP1Aunoe8T5guL"
```

## 监控项目

自动重编译并运行项目

```
cargo watch -x run
```