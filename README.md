## V2EX自动签到程序

支持的配置文件格式有：`toml`、`yaml`，请将配置文件命名为`config.toml`或`config.yaml`放置在与程序同级目录即可。

### 配置

签到程序需要依赖V2EX的用户Cookie，您必须在配置文件中配置Cookie，以`toml`为例

```toml
cookie = 'PB3_"2|****1:5***"'
```

如果您需要在签到失败后通过邮箱提醒，还可配置SMTP，以`toml`为例

```toml
[email_config]
smtp_url = "smtp.163.com"
smtp_user = "xxxx@163.com"
smtp_pass = "Ldff35tcg12"
notify_from = "V2EX自动签到程序 <xxxx@163.com>"
notify_to = "尊贵的V2EX自动签到用户 <zzzz@qq.com>"
```

配合`cron`可实现每日自动签到