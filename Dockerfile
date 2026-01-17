# 多阶段构建 Docker 镜像

# 构建阶段
FROM rust:1.92-slim as builder

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src

# 构建 release 版本
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/rdiff /usr/local/bin/rdiff

# 创建工作目录
WORKDIR /data

# 设置入口点
ENTRYPOINT ["rdiff"]

# 默认命令
CMD ["--help"]

# 元数据
LABEL maintainer="Your Name <your.email@example.com>"
LABEL description="Powerful CLI diff tool with web visualization"
LABEL version="1.0.0"
