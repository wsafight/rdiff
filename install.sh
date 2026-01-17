#!/bin/bash
# rdiff 一键安装脚本
# 支持 macOS 和 Linux

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# GitHub 仓库信息
GITHUB_REPO="wsafight/rdiff"
BINARY_NAME="rdiff"

# 打印带颜色的消息
print_info() {
    echo -e "${CYAN}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# 检测操作系统和架构
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case "$os" in
        darwin)
            os="macos"
            ;;
        linux)
            os="linux"
            ;;
        *)
            print_error "不支持的操作系统: $os"
            exit 1
            ;;
    esac

    case "$arch" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        arm64|aarch64)
            arch="aarch64"
            ;;
        *)
            print_error "不支持的架构: $arch"
            exit 1
            ;;
    esac

    echo "${os}-${arch}"
}

# 获取最新版本号
get_latest_version() {
    local version=$(curl -fsSL "https://api.github.com/repos/${GITHUB_REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$version" ]; then
        print_warning "无法获取最新版本，使用 latest"
        echo "latest"
    else
        echo "$version"
    fi
}

# 下载并安装
install_rdiff() {
    local platform=$(detect_platform)
    local version=$(get_latest_version)

    print_info "检测到平台: ${platform}"
    print_info "目标版本: ${version}"

    local base_url="https://github.com/${GITHUB_REPO}/releases/${version}/download"
    local asset_name="${BINARY_NAME}-${platform}.tar.gz"
    local download_url="${base_url}/${asset_name}"

    echo ""
    print_info "开始安装 rdiff..."

    # 创建临时目录
    local tmp_dir=$(mktemp -d)
    cd "$tmp_dir"

    # 下载
    print_info "正在下载 ${asset_name}..."
    if command -v curl >/dev/null 2>&1; then
        if curl -fsSL "$download_url" -o rdiff.tar.gz; then
            print_success "下载完成"
        else
            print_error "下载失败"
            print_info "请检查网络连接或手动下载: ${download_url}"
            exit 1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if wget -q "$download_url" -O rdiff.tar.gz; then
            print_success "下载完成"
        else
            print_error "下载失败"
            exit 1
        fi
    else
        print_error "需要 curl 或 wget 来下载文件"
        exit 1
    fi

    # 解压
    print_info "正在解压..."
    tar -xzf rdiff.tar.gz
    print_success "解压完成"

    # 选择安装目录
    local install_dir="/usr/local/bin"
    if [ ! -w "$install_dir" ]; then
        print_warning "需要管理员权限安装到 ${install_dir}"
        sudo mv rdiff "$install_dir/${BINARY_NAME}"
        sudo chmod +x "$install_dir/${BINARY_NAME}"
    else
        mv rdiff "$install_dir/${BINARY_NAME}"
        chmod +x "$install_dir/${BINARY_NAME}"
    fi

    print_success "已安装到 ${install_dir}/${BINARY_NAME}"

    # 清理
    cd - > /dev/null
    rm -rf "$tmp_dir"

    # 验证安装
    echo ""
    if command -v rdiff >/dev/null 2>&1; then
        print_success "安装成功！"
        echo ""
        print_info "版本信息:"
        rdiff --version
        echo ""
        echo "试试这些命令:"
        echo "  rdiff --help"
        echo "  rdiff file1.txt file2.txt"
        echo "  rdiff file1.txt file2.txt --web"
    else
        print_warning "安装完成，但 rdiff 未在 PATH 中"
        print_info "请将 ${install_dir} 添加到您的 PATH 环境变量"
    fi
}

# 主函数
main() {
    echo "╔════════════════════════════════════════╗"
    echo "║   rdiff 安装脚本                       ║"
    echo "║   Powerful CLI Diff Tool              ║"
    echo "╚════════════════════════════════════════╝"
    echo ""

    # 检查依赖
    if ! command -v tar >/dev/null 2>&1; then
        print_error "需要 tar 命令"
        exit 1
    fi

    install_rdiff

    echo ""
    print_info "项目主页: https://github.com/${GITHUB_REPO}"
    echo ""
}

main
