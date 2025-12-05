# Web Chat - RAG & Gemini 驱动的即时通讯应用

这是一个基于 **Rust (Actix Web)** 和 **React (TypeScript + Vite)** 开发的全栈实时聊天应用。它集成了 Google Gemini 系列模型，利用 **RAG (检索增强生成)** 技术，让 AI 拥有长期的对话记忆能力。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/backend-Rust-orange.svg)
![React](https://img.shields.io/badge/frontend-React-61DAFB.svg)
![Docker](https://img.shields.io/badge/deployment-Docker-2496ED.svg)

## ✨ 核心特性

- **🧠 长期记忆 (RAG)**: 系统会自动将对话内容向量化并存入 SQLite 数据库。当您提问时，它会检索相关的历史对话作为上下文，让 AI "记得" 您说过的话。
- **🤖 多模型支持**:
    - **Gemini 2.0 Flash**: 极速响应，适合日常快速问答。
    - **Gemini 2.5 Flash**: 增强版，支持更长的上下文处理。
    - **Gemini 2.5 Pro**: 强大的推理模型，支持 **Thinking (深度思考)**，擅长处理复杂逻辑。
- **💬 实时通讯**: 基于 WebSocket 的低延迟双向通信。
- **📂 文件上下文**: 支持上传文本文件，AI 可以基于文件内容进行回答。
- **🐳 Docker 部署**: 开箱即用，数据持久化存储。

## 🛠️ 技术栈

### 后端 (Backend)
- **Framework**: [Actix Web](https://actix.rs/) - 高性能、异步的 Rust Web 框架。
- **Database**: [Rusqlite](https://github.com/rusqlite/rusqlite) (SQLite) - 轻量级嵌入式数据库，用于存储消息记录和向量数据。
- **AI Integration**: [Reqwest](https://github.com/seanmonstar/reqwest) - 用于调用 Google Gemini API。
- **Vector Search**: 内置简单的余弦相似度计算，实现本地向量检索。

### 前端 (Frontend)
- **Framework**: [React 19](https://react.dev/) + TypeScript.
- **Build Tool**: [Vite](https://vitejs.dev/) - 极速的开发与构建体验。
- **Styling**: [UnoCSS](https://unocss.dev/) - 原子化 CSS 引擎。
- **State Management**: 自定义 Hook (`useWebSocket`) 管理实时状态。

## 🚀 快速开始

### 前置要求
- [Docker](https://www.docker.com/) & Docker Compose
- Google Gemini API Key ([获取地址](https://aistudio.google.com/app/apikey))

### 1. 配置环境变量
复制示例配置文件并填入您的 API Key：
```bash
cp .env.example .env
```
在 `.env` 文件中设置：
```env
GEMINI_API_KEY=your_api_key_here
```

### 2. 使用 Docker 启动 (推荐)
```bash
docker-compose up --build -d
```
启动后访问: [http://localhost:23333](http://localhost:23333)

> **注意**: 聊天记录会自动保存在项目根目录下的 `data/web_chat.db` 文件中，即使重启容器也不会丢失。

### 3. 本地开发运行 (可选)

**后端**:
```bash
# 确保已安装 Rust 工具链
cargo run
```

**前端**:
```bash
cd frontend
bun install # 或 npm install
bun run dev # 或 npm run dev
```

## 🔧 手动部署 (非 Docker)

如果你不想使用 Docker，也可以手动构建并运行生产环境版本。

### 前置要求
- **Rust**: 1.83 或更高版本
- **Node.js**: 18+ 或 **Bun**: 1.0+

### 步骤 1: 构建前端
```bash
cd frontend
# 安装依赖
bun install # 或 npm install

# 构建生产版本
bun run build # 或 npm run build
```
构建完成后，会生成 `frontend/dist` 目录。

### 步骤 2: 准备静态文件
回到项目根目录，将前端构建产物复制到 `static` 目录（后端默认从这里读取）：
```bash
# 在项目根目录执行
# Linux/Mac
cp -r frontend/dist static

# Windows (PowerShell)
Copy-Item -Recurse frontend/dist static
```

### 步骤 3: 编译并运行后端
```bash
# 编译发布版本 (Release Mode)
cargo build --release

# 运行后端
# Linux/Mac
./target/release/web_chat

# Windows
.\target\release\web_chat.exe
```
程序启动后，访问 `http://localhost:23333` 即可使用。

> **提示**: 生产环境运行时，同样需要确保 `.env` 文件存在或已设置 `GEMINI_API_KEY` 环境变量。数据库文件默认生成在 `data/web_chat.db`。

## 📂 项目结构

```
.
├── data/               # [自动生成] 数据库持久化目录
├── src/                # Rust 后端
│   ├── handlers/       # 请求处理 (WebSocket, Upload)
│   ├── models/         # 数据结构定义
│   └── services/       # 核心业务 (Gemini API, RAG, Memory)
├── frontend/           # React 前端
│   ├── src/components  # UI 组件
│   └── src/hooks       # 状态逻辑
├── Dockerfile          # 多阶段构建文件
└── docker-compose.yml  # 容器编排配置
```

## 📄 许可证
MIT License