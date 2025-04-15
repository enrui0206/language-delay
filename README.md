1. 项目概述
1.1 项目目标
•	功能目标： 
o	提供一个支持语言发育迟缓的在线平台，帮助家长了解相关知识、进行自我筛查、参与社区讨论，并获取专业资源。
o	支持用户认证（注册、登录、退出），为后续功能（如视频上传和分析）提供权限控制。
o	预留扩展接口，支持未来引入 AI 功能（如视频连线、视频分析）。
•	技术目标： 
o	使用 Rust 语言开发后端（Actix Web 框架），确保高性能和内存安全。
o	通过学习 Rust 开发过程，提升对 Rust 语言的掌握。
o	实现前后端分离，为未来开发手机端 App 提供 API 支持。
1.2 技术栈
•	后端：Rust (Actix Web 框架)
•	前端：HTML, CSS, JavaScript (Bootstrap)
•	数据库：SQLite (通过 Diesel ORM)
•	模板引擎：Tera
•	用户认证：JWT (使用 jsonwebtoken 和 bcrypt)
•	依赖管理：Cargo (Rust 包管理器)
1.3 项目结构
text
CollapseWrapCopy
language_delay_rust/
│
├── src/
│   ├── main.rs              # 主入口文件，初始化服务器和中间件
│   ├── models.rs            # 数据库模型，定义数据结构
│   ├── schema.rs            # 数据库模式，由 Diesel 自动生成
│   ├── routes.rs            # 路由和视图逻辑，处理 HTTP 请求
│   ├── auth.rs              # 用户认证逻辑，处理 JWT 和密码加密
│   └── templates/           # HTML 模板，用于页面渲染
│       ├── base.html        # 基础模板，定义通用布局
│       ├── index.html       # 首页模板
│       ├── education.html   # 科普教育页面模板
│       ├── screening.html   # 自我筛查页面模板
│       ├── community.html   # 社区支持页面模板
│       ├── resources.html   # 资源页面模板
│       ├── login.html       # 登录页面模板
│       ├── register.html    # 注册页面模板
│
├── static/
│   ├── css/
│   │   └── styles.css       # 自定义 CSS 样式
│   └── js/
│       └── screening.js     # 自我筛查页面的 JavaScript 逻辑
│
├── migrations/              # 数据库迁移文件，存储 SQL 脚本
│   └── ...
│
├── Cargo.toml               # Rust 项目配置文件，管理依赖
└── diesel.toml              # Diesel 配置文件，定义模式生成路径
