#!/bin/bash

# Qi语言完整编译和运行脚本
# 使用方法: ./compile_and_run.sh <程序文件.qi>

set -e

if [ $# -eq 0 ]; then
    echo "使用方法: $0 <程序文件.qi>"
    exit 1
fi

QI_FILE="$1"
BASENAME=$(basename "$QI_FILE" .qi)
LL_FILE="${BASENAME}.ll"
EXE_FILE="${BASENAME}"

echo "=== Qi语言编译器 ==="
echo "正在编译: $QI_FILE"

# 1. 使用Qi编译器生成LLVM IR
echo "步骤1: 生成LLVM IR..."
cargo run -- compile "$QI_FILE" -o "$LL_FILE"

if [ ! -f "$LL_FILE" ]; then
    echo "错误: LLVM IR文件生成失败"
    exit 1
fi

echo "生成的LLVM IR:"
cat "$LL_FILE"
echo ""

# 2. 修复中文函数名问题（将主函数改为main）
echo "步骤2: 修复函数名..."
sed -i.bak 's/@主()/@main()/g' "$LL_FILE"
sed -i.bak 's/define i64 @主()/define i32 @main()/g' "$LL_FILE"
sed -i.bak 's/ret i64/ret i32/g' "$LL_FILE"

# 3. 使用clang编译为可执行文件
echo "步骤3: 编译为可执行文件..."
clang "$LL_FILE" -o "$EXE_FILE"

if [ ! -f "$EXE_FILE" ]; then
    echo "错误: 可执行文件生成失败"
    exit 1
fi

echo "步骤4: 运行程序..."
echo "========================"
./"$EXE_FILE"
echo "========================"
echo "程序退出码: $?"

# 清理临时文件
rm -f "${LL_FILE}.bak"

echo ""
echo "编译完成! 生成的文件:"
echo "  - LLVM IR: $LL_FILE"
echo "  - 可执行文件: $EXE_FILE"