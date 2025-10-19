# Qi Language Compiler - Quick Start Guide

**Purpose**: Get started with the Qi programming language compiler quickly
**Target Audience**: Chinese-speaking developers new to Qi language
**Prerequisites**: Basic programming knowledge, familiarity with command line

## What is Qi?

Qi (气) is a modern programming language with 100% Chinese keywords. It's designed to make programming more accessible to Chinese-speaking developers while maintaining the performance and capabilities of modern compiled languages.

### Key Features
- **100% Chinese keywords**: All language keywords are in Chinese
- **Multi-platform support**: Compile to Linux, Windows, macOS, and WebAssembly
- **Modern language features**: Functions, types, control flow, error handling
- **Performance**: Compiled to native code with LLVM optimization
- **UTF-8 native**: Full Unicode and Chinese character support

## Installation

### System Requirements
- **Operating System**: Linux, Windows 10+, macOS 10.15+
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Disk Space**: 500MB for compiler installation
- **Tools**: Git, C/C++ compiler (for runtime library)

### Install Qi Compiler

#### Option 1: Download Pre-built Binary (Recommended)
```bash
# Linux
wget https://releases.qi-lang.org/qi-compiler-v1.0.0-linux-x64.tar.gz
tar -xzf qi-compiler-v1.0.0-linux-x64.tar.gz
sudo cp qi-compiler-1.0.0/qi /usr/local/bin/

# Windows
# Download from https://releases.qi-lang.org/qi-compiler-v1.0.0-windows-x64.zip
# Extract and add to PATH

# macOS
brew install qi-lang/qi/qi-compiler
```

#### Option 2: Build from Source
```bash
git clone https://github.com/qi-lang/qi-compiler.git
cd qi-compiler
cargo build --release
sudo cp target/release/qi /usr/local/bin/
```

### Verify Installation
```bash
qi --version
# 输出: Qi 编译器 v1.0.0 (LLVM 15.0.7)
```

## Your First Qi Program

### Hello World
Create a new file called `你好.qi`:

```qi
// 你好世界程序
包 主程序;

函数 整数 主程序入口() {
    变量 问候语 = "你好，Qi世界！";
    打印(问候语);

    变量 数字 = 42;
    打印("幸运数字: {}", 数字);

    返回 0;
}
```

### Compile and Run
```bash
# 编译程序
qi compile 你好.qi --output 你好

# 运行程序
./你好

# 输出:
# 你好，Qi世界！
# 幸运数字: 42
```

## Language Basics

### Basic Data Types
```qi
// 基础数据类型示例
变量 整数年龄 = 25;           // 整数类型
变量 浮点数圆周率 = 3.14159;  // 浮点数类型
变量 布尔是否成年 = 真;       // 布尔类型 (真/假)
变量 字符串姓名 = "张三";     // 字符串类型
变量 字符初始 = 'A';          // 字符类型

// 常量声明
常量 最大重试次数 = 3;
```

### Variables and Constants
```qi
// 可变变量
变量 计数器 = 0;
计数器 = 计数器 + 1;

// 不可变变量
不可变 用户名 = "admin";

// 常量 (编译时确定)
常量 版本号 = "1.0.0";
```

### Operators
```qi
// 算术运算符
变量 结果 = 10 加 5;           // 加法 (也可以用 +)
变量 差值 = 10 减 3;           // 减法 (也可以用 -)
变量 乘积 = 6 乘 7;            // 乘法 (也可以用 *)
变量 商数 = 20 除 4;           // 除法 (也可以用 /)
变量 余数 = 15 取余 4;         // 取余 (也可以用 %)

// 比较运算符
布尔 相等 = (5 等于 5);       // 等于 (也可以用 ==)
布尔 不相等 = (5 不等于 3);    // 不等于 (也可以用 !=)
布尔 大于 = (10 大于 5);       // 大于 (也可以用 >)
布尔 小于等于 = (5 小于等于 5); // 小于等于 (也可以用 <=)

// 逻辑运算符
布尔 与运算 = 真 与 假;        // 逻辑与 (也可以用 &&)
布尔 或运算 = 真 或 假;        // 逻辑或 (也可以用 ||)
布尔 非运算 = 非 真;          // 逻辑非 (也可以用 !)
```

### Control Flow
```qi
// 条件语句
变量 年龄 = 18;

如果 年龄 >= 18 {
    打印("成年人");
} 否则 {
    打印("未成年");
}

// 多重条件
变量 分数 = 85;

如果 分数 >= 90 {
    打印("优秀");
} 否则 如果 分数 >= 80 {
    打印("良好");
} 否则 如果 分数 >= 60 {
    打印("及格");
} 否则 {
    打印("不及格");
}

// 循环语句
// 当循环
变量 计数 = 1;
当 计数 <= 5 {
    打印("计数: {}", 计数);
    计数 = 计数 + 1;
}

// 对于循环 (范围)
对于 i 在 1..5 {
    打印("数字: {}", i);
}

// 对于循环 (数组)
变量 数字列表 = [10, 20, 30, 40, 50];
对于 数字 在 数字列表 {
    打印("值: {}", 数字);
}
```

### Functions
```qi
// 基础函数定义
函数 整数 加法(整数 数字1, 整数 数字2) {
    变量 结果 = 数字1 + 数字2;
    返回 结果;
}

// 调用函数
变量 总和 = 加法(10, 20);
打印("总和: {}", 总和);

// 无返回值函数
函数 空 显示消息(字符串 消息) {
    打印("消息: {}", 消息);
}

显示消息("这是一个测试");

// 带默认参数的函数
函数 整数 幂运算(整数 基础数, 整数 指数 = 2) {
    变量 结果 = 1;
    对于 i 在 1..指数+1 {
        结果 = 结果 * 基础数;
    }
    返回 结果;
}

变量 平方 = 幂运算(5);        // 使用默认指数 2
变量 立方 = 幂运算(5, 3);     // 指定指数 3
```

### Arrays and Data Structures
```qi
// 数组操作
变量 数字数组 = [1, 2, 3, 4, 5];
变量 第一个元素 = 数字数组[0];     // 数组索引从 0 开始
变量 数组长度 = 数字数组.长度();    // 获取数组长度

// 添加元素到数组
数字数组.添加(6);

// 字符串操作
变量 姓名 = "张三";
变量 问候 = "你好，" + 姓名 + "！";

// 字符串格式化
变量 格式化消息 = "姓名: {}, 年龄: {}";
变量 完整消息 = 格式化消息.格式(姓名, 25);
```

### Error Handling
```qi
// 简单错误处理
函数 整数 安全除法(整数 被除数, 整数 除数) {
    如果 除数 等于 0 {
        打印("错误：除数不能为零");
        返回 0;
    }

    返回 被除数 / 除数;
}

// 使用错误处理函数
变量 结果1 = 安全除法(10, 2);   // 正常情况
变量 结果2 = 安全除法(10, 0);   // 错误情况
```

## Advanced Examples

### Calculator Program
```qi
// 计算器程序
包 计算器;

函数 整数 主程序入口() {
    打印("=== Qi 语言计算器 ===");

    变量 继续 = 真;

    当 继续 {
        打印("请输入第一个数字:");
        变量 数字1 = 输入整数();

        打印("请输入运算符 (+, -, *, /):");
        变量 运算符 = 输入字符串();

        打印("请输入第二个数字:");
        变量 数字2 = 输入整数();

        变量 结果;

        匹配 运算符 {
            "+" => 结果 = 数字1 + 数字2,
            "-" => 结果 = 数字1 - 数字2,
            "*" => 结果 = 数字1 * 数字2,
            "/" => 结果 = 数字1 / 数字2,
            _ => {
                打印("错误：不支持的运算符");
                继续;
            }
        }

        打印("结果: {} {} {} = {}", 数字1, 运算符, 数字2, 结果);

        打印("继续计算? (y/n):");
        变量 答案 = 输入字符串();
        如果 答案 不等于 "y" 且 答案 不等于 "Y" {
            继续 = 假;
        }
    }

    打印("感谢使用计算器！");
    返回 0;
}
```

### Simple Game
```qi
// 猜数字游戏
包 游戏;
导入 标准库.随机;

函数 整数 主程序入口() {
    打印("=== 猜数字游戏 ===");
    打印("我想了一个 1 到 100 之间的数字，你能猜出来吗？");

    变量 目标数字 = 随机数(1, 100);
    变量 猜测次数 = 0;
    变量 最大次数 = 10;
    变量 猜对了 = 假;

    当 猜测次数 < 最大次数 且 !猜对了 {
        打印("请输入你的猜测 (1-100):");
        变量 猜测 = 输入整数();
        猜测次数 = 猜测次数 + 1;

        如果 猜测 等于 目标数字 {
            猜对了 = 真;
            打印("恭喜！你猜对了！");
        } 否则 如果 猜测 < 目标数字 {
            打印("太小了！");
        } 否则 {
            打印("太大了！");
        }

        打印("剩余次数: {}", 最大次数 - 猜测次数);
    }

    如果 猜对了 {
        打印("你用了 {} 次猜中了数字 {}！", 猜测次数, 目标数字);
    } 否则 {
        打印("很遗憾，你没有在 {} 次内猜中。正确答案是: {}", 最大次数, 目标数字);
    }

    返回 0;
}
```

## Compilation Options

### Basic Compilation
```bash
# 基础编译
qi compile 程序.qi

# 指定输出文件名
qi compile 程序.qi --output 我的应用

# 指定目标平台
qi compile 程序.qi --target windows
qi compile 程序.qi --target macos
qi compile 程序.qi --target wasm
```

### Optimization Levels
```bash
# 无优化 (快速编译)
qi compile 程序.qi --optimization none

# 基础优化 (默认)
qi compile 程序.qi --optimization basic

# 标准优化
qi compile 程序.qi --optimization standard

# 最大优化 (慢编译，快运行)
qi compile 程序.qi --optimization maximum
```

### Debug Options
```bash
# 包含调试符号
qi compile 程序.qi --debug-symbols

# 启用运行时检查
qi compile 程序.qi --runtime-checks

# 显示详细编译信息
qi compile 程序.qi --verbose

# 将警告视为错误
qi compile 程序.qi --warnings-as-errors
```

### Multi-file Projects
```bash
# 编译多个文件
qi compile 主程序.qi 工具函数.qi 数据结构.qi

# 指定导入路径
qi compile 主程序.qi --import-path ./lib --import-path ./modules

# 使用配置文件
qi compile 主程序.qi --config qi-config.json
```

## Error Handling and Debugging

### Common Error Messages

#### Syntax Errors (语法错误)
```
E0001: 语法错误 - 在第3行第5列发现无效的字符序列
   建议：检查是否使用了有效的中文关键字或标识符

E0002: 语法错误 - 在第5行第20列语句末尾缺少分号
   建议：在表达式末尾添加分号 '；'
```

#### Semantic Errors (语义错误)
```
E0003: 语义错误 - 在第4行第10列使用了未声明的变量 '计数器'
   建议：请先使用 '变量' 关键字声明变量

E0004: 类型错误 - 在第7行第15列表达式类型不匹配
   期望：整数类型，实际：字符串类型
   建议：确保操作数类型一致或进行类型转换
```

### Debugging Tips

1. **使用调试符号编译**
   ```bash
   qi compile 程序.qi --debug-symbols
   ```

2. **启用详细输出**
   ```bash
   qi compile 程序.qi --verbose
   ```

3. **检查语法**
   ```bash
   qi check 程序.qi  # 只检查语法，不生成可执行文件
   ```

4. **格式化代码**
   ```bash
   qi format 程序.qi  # 自动格式化代码
   ```

## Project Structure

### Recommended Directory Layout
```
我的项目/
├── src/                  # 源代码目录
│   ├── main.qi          # 主程序入口
│   ├── utils.qi         # 工具函数
│   ├── data.qi          # 数据结构
│   └── tests/           # 测试文件
│       ├── test_main.qi
│       └── test_utils.qi
├── docs/                 # 文档目录
│   ├── README.md
│   └── API.md
├── build/                # 构建输出目录
├── qi-config.json       # 配置文件
└── README.md
```

### Configuration File (qi-config.json)
```json
{
  "项目名称": "我的Qi项目",
  "版本": "1.0.0",
  "目标平台": "linux",
  "优化级别": "standard",
  "调试符号": false,
  "运行时检查": true,
  "导入路径": ["./src", "./lib"],
  "编译选项": {
    "警告视为错误": false,
    "详细输出": false,
    "并行编译": true
  }
}
```

## Getting Help

### Command Line Help
```bash
# 显示帮助信息
qi --help

qi compile --help
qi check --help
qi format --help
```

### Language Reference
```bash
# 显示支持的关键字
qi language --keywords

# 显示支持的数据类型
qi language --types

# 显示示例代码
qi examples --basic
qi examples --advanced
```

### Community Resources
- **官方网站**: https://qi-lang.org
- **文档**: https://docs.qi-lang.org
- **GitHub**: https://github.com/qi-lang/qi
- **社区论坛**: https://community.qi-lang.org

## Next Steps

1. **Practice basic syntax**: Try writing simple programs with variables, functions, and control flow
2. **Explore standard library**: Learn about built-in functions and data structures
3. **Build a small project**: Apply what you've learned to a real project
4. **Join the community**: Get help and share your creations with other Qi developers

## Common Questions

### Q: Can I mix Chinese and English in my code?
A: Keywords must be in Chinese, but variable names and comments can be in any language supported by UTF-8.

### Q: How do I handle Unicode characters in strings?
A: Qi has native UTF-8 support. Just include Chinese characters directly in string literals.

### Q: Can I call C libraries from Qi?
A: Yes, Qi provides a foreign function interface (FFI) for calling C libraries.

### Q: Is Qi suitable for large projects?
A: Yes, Qi supports modules, packages, and modern programming features suitable for large applications.

### Q: How fast are Qi programs compared to C?
A: Qi compiles to native code with LLVM optimization, typically achieving 80-95% of C performance.

---

**Happy coding with Qi! 🚀**

如果你有任何问题或需要帮助，请访问我们的社区论坛或查看我们的文档。