# Rust: 认证微服务

##### 开发环境配置(MacOS)

```
# Mysql 客户端库
brew install mysql-connector-c
# Diesel 命令行工具
cargo install diesel_cli
```

##### 时间流

- 使用 Email 注册, 邮件接收验证连接
- 点击连接, 用相同的 Email 和密码注册
- 用 Email 和密码登录, 通过验证并接收认证 Cookie

#### Setting diesel for project

set database connection string in `.env` file in the project root.
```
echo DATABASE_URL=mysql://root:root@localhost/rust_auth_server > .env
```

Create the database(If it is not exists, auto create it), and create `migrations` directory in the project root.

```
diesel setup
```


Generate migration skelton files of table
```
diesel migration generate users
diesel migration generate invitations
```

Output as follows:
```
➜  rust-auth-server git:(main) ✗ diesel migration generate users
Creating migrations/2022-10-14-121759_users/up.sql
Creating migrations/2022-10-14-121759_users/down.sql
➜  rust-auth-server git:(main) ✗ diesel migration generate invitations
Creating migrations/2022-10-14-121837_invitations/up.sql
Creating migrations/2022-10-14-121837_invitations/down.sql
```

Create database tables and `src/schema.rs` file.

```
diesel migration run
```


## Reference

- https://stackoverflow.com/questions/72927992/auto-completion-not-working-for-rust-in-module-files-vs-code


## Test

```
curl --request POST \
  --url http://localhost:3000/api/invitation \
  --header 'content-type: application/json' \
  --data '{"email":"developerworks@163.com"}'


curl --request POST \
  --url http://localhost:3000/api/register/c5d567a5-1594-48a8-bcd4-2edf195c0b1f \
  --header 'content-type: application/json' \
  --data '{"email":"developerworks@163.com", "password":"root"}'


curl -i --request POST \
  --url http://localhost:3000/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "developerworks@163.com","password":"root"}'
```