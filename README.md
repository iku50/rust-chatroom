# rust-chatroom

一个简单的聊天室桌面应用，使用 Rust 全栈编写。使用 WebSocket 协议,学习 Rust 语言的练手项目。

## 项目结构

### 后端

使用 `axum` 框架编写，使用 `tokio` 异步运行时，使用 `Postgres` 库操作 `PostgreSQL` 数据库。

### 前端

使用 `tauri` 框架编写，使用 `Vue` 框架编写前端页面。

### 协议

使用 `WebSocket` 协议进行通信。

## 功能实现

- 用户管理
  - [ ] 用户注册
  - [ ] 用户登录
  - [ ] 用户注销
  - [ ] 用户信息修改
- 聊天室管理
  - [ ] 聊天室创建
  - [ ] 聊天室加入
  - [ ] 聊天室退出
  - [ ] 聊天室信息修改
  - [ ] 聊天室删除
- 聊天室功能
  - [ ] 聊天室内发送消息
  - [ ] 聊天室内发送图片
  - [ ] 聊天室内发送文件

