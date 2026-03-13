# Tauri + Vue + TypeScript

基于 Tauri 2.0 + Vue 3 + TypeScript 的桌面应用程序模板。

## 技术栈

- **前端框架**: Vue 3 (Composition API + `<script setup>`)
- **构建工具**: Vite 6
- **语言**: TypeScript
- **桌面框架**: Tauri 2.0
- **后端语言**: Rust

## 项目结构

```
tauri-app/
├── src/                      # Vue 前端源代码
│   ├── assets/               # 静态资源
│   │   └── vue.svg           # Vue logo
│   ├── components/           # Vue 组件
│   │   └── FloatingBall.vue  # 悬浮球组件
│   ├── App.vue               # 主应用组件
│   ├── main.ts               # 前端入口文件
│   └── vite-env.d.ts         # Vite 类型声明
├── src-tauri/                # Rust 后端源代码
│   ├── capabilities/         # Tauri 权限配置
│   │   └── default.json      # 默认权限
│   ├── gen/                  # Tauri 自动生成的代码
│   │   └── schemas/           # JSON Schema 文件
│   ├── icons/                # 应用图标
│   ├── src/
│   │   ├── lib.rs            # Rust 库入口（核心逻辑）
│   │   └── main.rs           # Rust 程序入口
│   ├── Cargo.toml           # Rust 依赖配置
│   ├── tauri.conf.json       # Tauri 应用配置
│   └── build.rs              # Tauri 构建脚本
├── public/                   # 公共静态资源
│   ├── tauri.svg             # Tauri logo
│   └── vite.svg              # Vite logo
├── index.html                # HTML 入口
├── package.json              # Node 依赖配置
├── tsconfig.json             # TypeScript 配置
├── vite.config.ts            # Vite 构建配置
└── README.md                 # 项目说明
```

## 功能特性

- 桌面悬浮球功能（类似搜狗输入法悬浮球）
- 5个快捷功能按钮：网页搜索、图片搜索、视频搜索、新闻搜索、天气查询
- 点击按钮后弹出内容窗口加载百度相关页面
- 悬浮球始终置顶显示
- Vue 3 前端与 Rust 后端的 IPC 通信
- 支持热重载开发
- 跨平台桌面应用打包

## 悬浮球功能说明

### 悬浮球按钮

| 按钮 | 功能 | 链接 |
|------|------|------|
| 网 | 网页搜索 | https://www.baidu.com |
| 图 | 图片搜索 | https://image.baidu.com |
| 视频 | 视频搜索 | https://v.baidu.com |
| 新 | 新闻搜索 | https://news.baidu.com |
| 天气 | 天气查询 | https://weather.baidu.com |

### 窗口说明

- **悬浮球窗口**: 60x60 像素、圆形、透明背景、始终置顶、无边框
- **内容窗口**: 800x600 像素、可调整大小、用于显示百度搜索页面

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- npm 或 pnpm

### 安装依赖

```bash
npm install
```

### 开发模式

启动开发服务器：

```bash
npm run tauri dev
```

### 构建生产版本

```bash
npm run tauri build
```

## 关键配置

### 窗口配置 (src-tauri/tauri.conf.json)

应用包含两个窗口：
1. `floating-ball` - 悬浮球窗口（始终置顶、透明背景）
2. `content` - 内容展示窗口（用于显示百度搜索页面）

### Rust 后端命令

- `show_content_window` - 显示内容窗口
- `hide_content_window` - 隐藏内容窗口
- `navigate_content_window(url)` - 导航到指定 URL

### 前端调用示例

```typescript
import { invoke } from "@tauri-apps/api/core";

// 导航到指定 URL
await invoke("navigate_content_window", { url: "https://www.baidu.com" });
```

## 推荐开发工具

- [VS Code](https://code.visualstudio.com/)
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) - Vue 语言支持
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) - Tauri 开发支持
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - Rust 语言支持

## 相关文档

- [Vue 3 文档](https://vuejs.org/)
- [Tauri 官方文档](https://tauri.app/)
- [Vite 文档](https://vitejs.dev/)
- [TypeScript 文档](https://www.typescriptlang.org/)
