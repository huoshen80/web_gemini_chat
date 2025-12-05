# ===== 前端构建阶段 =====
FROM oven/bun:1 AS frontend-builder

WORKDIR /app/frontend

# 复制前端依赖文件
COPY frontend/package.json frontend/bun.lock* ./

# 安装依赖
RUN bun install --frozen-lockfile

# 复制前端源码
COPY frontend/ ./

# 构建前端
RUN bun run build

# ===== 后端构建阶段 =====
FROM rust:1.91.1-slim AS backend-builder

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock* ./

# 创建虚拟 src 以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖（用于缓存）
RUN cargo build --release && rm -rf src

# 复制实际源码
COPY src/ ./src/

# 重新构建应用
RUN touch src/main.rs && cargo build --release

# ===== 运行阶段 =====
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# 从后端构建阶段复制二进制文件
COPY --from=backend-builder /app/target/release/web_chat ./

# 从前端构建阶段复制静态文件
COPY --from=frontend-builder /app/frontend/dist ./static

# 创建非 root 用户
RUN useradd -m -u 1000 appuser && \
    mkdir -p /app/data && \
    chown -R appuser:appuser /app
USER appuser

# 暴露端口
EXPOSE 23333

# 设置环境变量
ENV RUST_LOG=info

# 启动应用
CMD ["./web_chat"]
