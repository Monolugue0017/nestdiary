# 栖记 NestDiary

> 隐私优先的本地日记 + 备忘 + 待办三合一桌面应用

数据 100% 存储在本地设备，零账号、零云端、离线可用。

## ✨ 功能特性

### P0 - MVP 核心功能

- **日记撰写编辑器** - 基于 Tiptap 的富文本编辑器，支持 Markdown 快捷输入（H1-H3、粗体、斜体、列表、引用、代码块），停止输入 2 秒自动保存
- **每日日记管理** - 按日期创建/打开日记，前后翻页切换日期，"回到今天"按钮，日期选择器跳转
- **待办清单** - 创建/编辑/删除/完成/优先级（高/中/低），回车连续添加，完成项视觉区分
- **备忘录** - 快速创建文本备忘，支持置顶，按修改时间倒序排列
- **纯本地数据存储** - Web 版使用 IndexedDB，桌面版使用 SQLite + Markdown 文件双存储
- **应用密码锁** - SHA-256 哈希密码保护，空闲自动锁定，错误 3 次延迟重试
- **全文搜索** - 跨日记、备忘、待办全文搜索，防抖 300ms，匹配片段高亮，点击跳转
- **本地备份与导出** - 一键备份为 ZIP，导出单篇/全部 Markdown 文件
- **三模块统一界面** - 侧边栏切换，Ctrl+1/2/3 快捷键，切换不丢失编辑状态

### 技术亮点

- 亮色/暗色主题切换
- 中文优化的界面和日期显示
- 响应式布局，适配不同窗口大小
- 数据可移植，Markdown 文件可在任意编辑器打开

## 🛠 技术栈

| 层级 | 技术 | 说明 |
|------|------|------|
| 应用框架 | Tauri 2.0 | 轻量桌面应用框架，包体积 2-10MB |
| 前端框架 | Vue 3 + TypeScript | Composition API，SFC 单文件组件 |
| UI 组件 | 自定义组件 + SVG 图标 | 无重量级 UI 库依赖 |
| 编辑器 | Tiptap (ProseMirror) | 富文本 + Markdown 编辑 |
| 状态管理 | Pinia | Vue 3 官方推荐状态管理 |
| 路由 | Vue Router 4 | Hash 模式路由 |
| Web 存储 | Dexie.js (IndexedDB) | 浏览器端本地存储 |
| 桌面存储 | SQLite (rusqlite) + Markdown 文件 | 结构化数据 + 可移植正文 |
| 后端语言 | Rust | Tauri 原生，内存安全 |
| 备份打包 | JSZip | 前端 ZIP 打包 |

## 📦 项目结构

```
nestdiary/
├── src/                        # Vue 3 前端源码
│   ├── components/             # 可复用组件
│   │   ├── DiaryEditor.vue    # Tiptap 日记编辑器
│   │   ├── LockScreen.vue     # 密码锁屏
│   │   ├── Sidebar.vue        # 侧边栏导航
│   │   └── TopBar.vue         # 顶部搜索栏
│   ├── views/                  # 页面视图
│   │   ├── DiaryView.vue      # 日记模块
│   │   ├── TodoView.vue       # 待办模块
│   │   ├── MemoView.vue       # 备忘模块
│   │   ├── SearchView.vue     # 搜索页面
│   │   └── SettingsView.vue   # 设置页面
│   ├── db/                     # 数据访问层
│   │   └── database.ts        # Dexie.js IndexedDB 封装
│   ├── stores/                 # Pinia 状态管理
│   │   └── app.ts             # 应用状态（密码锁/主题）
│   ├── utils/                  # 工具函数
│   │   ├── crypto.ts          # SHA-256 密码哈希
│   │   ├── date.ts            # 日期处理
│   │   └── export.ts          # 备份与导出
│   ├── types/                  # TypeScript 类型定义
│   ├── router/                 # Vue Router 配置
│   ├── styles/                 # 全局样式
│   ├── App.vue                 # 根组件
│   └── main.ts                 # 应用入口
├── src-tauri/                  # Tauri + Rust 后端
│   ├── src/
│   │   ├── main.rs            # Rust 入口
│   │   ├── lib.rs             # Tauri 应用配置
│   │   ├── db.rs              # SQLite 数据库管理
│   │   ├── diary.rs           # Markdown 文件操作
│   │   └── commands.rs        # Tauri 命令定义
│   ├── capabilities/          # Tauri 权限配置
│   ├── Cargo.toml             # Rust 依赖
│   ├── tauri.conf.json        # Tauri 应用配置
│   └── build.rs               # 构建脚本
├── index.html                  # HTML 入口
├── package.json                # Node.js 依赖
├── vite.config.ts              # Vite 配置
├── tsconfig.json               # TypeScript 配置
└── README.md                   # 项目文档
```

## 🚀 快速开始

### 环境要求

- Node.js 18+ 
- npm 或 pnpm
- （可选）Rust + Tauri 2.0 CLI 用于构建桌面版

### Web 版开发运行

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 构建生产版本
npm run build

# 预览生产版本
npm run preview
```

访问 http://localhost:1420 即可使用。

### 桌面版构建（需要 Rust）

```bash
# 安装 Rust 工具链
# https://www.rust-lang.org/tools/install

# 安装 Tauri CLI
npm install -D @tauri-apps/cli

# 开发模式运行桌面应用
npm run tauri dev

# 构建桌面安装包
npm run tauri build
```

## 🔒 隐私设计

- **零账号** - 无需注册，打开即用
- **零云端** - 数据永远不离开你的设备
- **离线可用** - 100% 功能在离线状态下可用
- **数据可移植** - 日记以 Markdown 文件存储，可在任意编辑器打开
- **密码保护** - SHA-256 哈希密码，本地验证
- **可备份** - 一键 ZIP 备份，包含所有数据

## 📋 开发路线图

- [x] P0-01 日记撰写编辑器
- [x] P0-02 每日日记管理
- [x] P0-03 待办清单
- [x] P0-04 备忘录
- [x] P0-05 纯本地数据存储
- [x] P0-06 应用密码锁
- [x] P0-07 全文搜索
- [x] P0-08 本地备份与导出
- [x] P0-09 三模块统一界面
- [ ] P1-01 日历视图 + 往年今日
- [ ] P1-02 自然语言待办解析
- [ ] P1-03 图片/附件插入
- [ ] P1-04 标签分类系统
- [ ] P1-05 桌面常驻/快速唤出
- [ ] P1-06 数据统计概览

## 📄 License

MIT License

## 🙏 致谢

- [Tauri](https://tauri.app/) - 轻量级桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Tiptap](https://tiptap.dev/) - 无头富文本编辑器
- [Dexie.js](https://dexie.org/) - IndexedDB 封装库
- [Naive UI](https://www.naiveui.com/) - Vue 3 组件库（参考）
