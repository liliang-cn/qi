**「纯中文关键字 + 可编译」语言规范**真正落地，做出一份**完整的编译器设计文档**。

你问的「用 Rust 还是 C」是非常关键的设计决策，我先给你一个非常简洁的比较 👇

| 项目           | Rust 🦀                            | C 🧰                            |
| -------------- | ---------------------------------- | ------------------------------- |
| 安全性         | 内存安全（无需手动管理）           | 需手动管理内存，容易出错        |
| 并发           | 有原生的 `std::thread` 和安全封装  | pthread 或 clone，自行封装      |
| LLVM 绑定      | 官方 `inkwell` crate 非常成熟      | 直接使用 `llvm-c`，但手写代码多 |
| 编译器结构适配 | 非常适合编译器项目                 | 可行但代码量较大                |
| 生态           | inkwell、chumsky、pest（parser）等 | flex/bison、LLVM-C API          |

✅ 建议：
如果你目标是 **现代、安全、高效** → 用 **Rust**。
如果你目标是 **极简、接近底层控制** → 用 **C**。

👉 下面我给你的是：

> 📄 **Qi 语言（.qi）编译器完整设计文档 v1.0**
> 🦀 采用 **Rust + LLVM（inkwell）** 实现
> 🈶 100%中文关键字前端
> 🈚 英文关键字暴露给用户
> 🧠 完整词法、语法、AST、IR 生成、运行时、并发支持、包管理机制

---

# 📜 **Qi 语言编译器设计文档 v1.0**

## 1. 概述

**Qi 语言**是一门使用 100% 中文关键字的现代编程语言，目标是：

- ✅ 语法简洁、无英文关键字
- 🧠 对标 C / Rust 的性能和底层控制力
- 🧵 内建并发模型（线程/进程/协程）
- 📦 内置包系统和标准库
- ⚡️ 编译到 LLVM IR，然后生成原生机器码

编译器项目代号：`qi`
文件扩展名：`.qi`
构建产物：`.o`、`.qi.ll`、二进制

---

## 2. 项目结构

```
qi-compiler/
 ├─ Cargo.toml               # Rust 项目配置
 ├─ src/
 │   ├─ main.rs              # CLI 入口
 │   ├─ lexer.rs             # 词法分析
 │   ├─ parser.rs            # 语法分析（AST）
 │   ├─ ast.rs               # 抽象语法树定义
 │   ├─ codegen.rs           # LLVM IR 生成
 │   ├─ runtime.rs           # 运行时库绑定
 │   ├─ stdlib/              # Qi 标准库
 │   └─ package.rs           # 包管理器
 ├─ runtime/
 │   ├─ qirt.c               # 运行时C底层（线程/内存）
 │   └─ qirt.h
 ├─ stdlib/
 │   ├─ 数学.qi
 │   ├─ 字符串.qi
 │   └─ 线程.qi
 ├─ examples/
 │   └─ helloworld.qi
 └─ target/
```

---

## 3. 语言关键字表

（与之前一致）
✅ 100% 中文关键字，无英文保留字。
包括：`结构体`、`函数`、`如果`、`否则`、`线程`、`包`、`导入`、`导出` 等等。

---

## 4. 词法设计（Lexer）

### 4.1 主要 Token 类型

| Token 类型 | 说明                    | 示例          |     |         |
| ---------- | ----------------------- | ------------- | --- | ------- |
| 关键字     | `函数`、`如果`、`包` 等 | `函数`        |     |         |
| 标识符     | 变量/函数/包名          | `用户` `姓名` |     |         |
| 字符串     | 双引号字符串            | `"你好"`      |     |         |
| 数字       | 整数/浮点数             | `123` `3.14`  |     |         |
| 运算符     | `+ - \* / == != > < &&  |               | `   | `a + b` |
| 分隔符     | `; , { } ( )`           | `函数 f() {}` |     |         |

### 4.2 Lexer 使用 pest

Rust 中通过正则 + `pest` 定义 `.pest` 语法文件：

```rust
规则
  = { 关键字 | 标识符 | 数字 | 字符串 | 运算符 | 分隔符 }
关键字
  = { "函数" | "结构体" | "如果" | "否则" | ... }
标识符
  = @{ (['\u4e00'..'\u9fa5'] | ['a'..'z' 'A'..'Z'] | ['0'..'9'] )+ }
数字
  = @{ ['0'..'9']+ ( "." ['0'..'9']+ )? }
字符串
  = _{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
```

---

## 5. 语法设计（Parser）

我们定义一套上下文无关语法（类似 C）：

```
程序           ::= (包定义 | 导入语句 | 声明)* 主函数?
包定义         ::= "包" 标识符 ";"
导入语句       ::= "导入" 标识符 ";"
声明           ::= 函数定义 | 结构体定义 | 变量声明
函数定义       ::= "函数" 类型 标识符 "(" 参数列表? ")" 语句块
结构体定义     ::= "结构体" 标识符 "{" (类型 标识符 ";")* "}"
语句块         ::= "{" 语句* "}"
语句           ::= 变量声明 ";" | 表达式 ";" | 控制语句
控制语句       ::= 如果语句 | 循环语句 | 返回语句
如果语句       ::= "如果" "(" 表达式 ")" 语句块 ("否则" 语句块)?
```

---

## 6. AST（抽象语法树）

使用 Rust 定义结构体：

```rust
pub enum 节点 {
    程序(Vec<节点>),
    包定义(String),
    导入(String),
    函数定义 {
        返回类型: 类型节点,
        名称: String,
        参数: Vec<(类型节点, String)>,
        体: Box<节点>,
    },
    结构体定义 {
        名称: String,
        字段: Vec<(类型节点, String)>,
    },
    语句(Vec<节点>),
    表达式(表达式节点),
    ...
}
```

---

## 7. LLVM IR 生成（Codegen）

使用 inkwell 生成 IR。

例如 `函数 整数 加法(整数 a, 整数 b)` → IR:

```llvm
define i32 @加法(i32 %a, i32 %b) {
entry:
  %addtmp = add i32 %a, %b
  ret i32 %addtmp
}
```

Rust 代码：

```rust
fn 生成函数(&mut self, 函数: &节点) {
    let fn_type = self.context.i32_type().fn_type(&[i32.into(), i32.into()], false);
    let function = self.module.add_function("加法", fn_type, None);
    let builder = self.context.create_builder();
    let entry = self.context.append_basic_block(function, "entry");
    builder.position_at_end(entry);
    let a = function.get_nth_param(0).unwrap().into_int_value();
    let b = function.get_nth_param(1).unwrap().into_int_value();
    let sum = builder.build_int_add(a, b, "addtmp");
    builder.build_return(Some(&sum));
}
```

---

## 8. 运行时库（Runtime）

运行时使用 C 编写，Rust FFI 调用：

```c
// qirt.h
#ifndef QIRT_H
#define QIRT_H

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

typedef void* (*qi_thread_func)(void*);

void* qi_thread_spawn(qi_thread_func func, void* arg);
void  qi_thread_join(void* handle);

#endif
```

```c
// qirt.c
#include "qirt.h"

void* qi_thread_spawn(qi_thread_func func, void* arg) {
    pthread_t* t = malloc(sizeof(pthread_t));
    pthread_create(t, NULL, func, arg);
    return t;
}

void qi_thread_join(void* handle) {
    pthread_t* t = (pthread_t*)handle;
    pthread_join(*t, NULL);
    free(t);
}
```

---

## 9. 标准库（Qi 标准库）

```
stdlib/
 ├─ 数学.qi
 ├─ 字符串.qi
 ├─ 线程.qi
```

例子：`线程.qi`

```qi
包 标准库.线程;

导出 函数 线程* 创建线程(函数 空 * f, 空* 参数) {
    返回 运行时.创建线程(f, 参数);
}

导出 函数 空 等待(线程* t) {
    运行时.等待线程(t);
}
```

---

## 10. 包管理系统

- 每个项目根目录包含 `qimod.json`
- 格式：

```json
{
  "名称": "示例项目",
  "版本": "0.1.0",
  "依赖": {
    "数学": "1.0",
    "线程": "1.0"
  }
}
```

- 编译器自动解析依赖，下载包，加入编译路径。

---

## 11. 并发支持

- ✅ `线程` → pthread 封装
- ✅ `进程` → fork 封装
- ✅ `协程` → ucontext 或 libco 封装

语法：

```qi
线程* t = 创建线程(工作函数, 空);
等待(t);
```

---

## 12. 编译器 CLI 设计

```
qi [命令] [参数]

命令：
  build         编译项目
  run           编译并运行
  fmt           格式化代码
  get           安装包
  repl          交互式解释器
```

示例：

```bash
qi build main.qi
qi run main.qi
qi fmt src/
```

---

## 13. Hello World 示例

```qi
包 主程序;
导入 标准库.打印;

函数 整数 主函数() {
    打印("你好，Qi语言！");
    返回 0;
}
```

编译 & 运行：

```bash
qi build main.qi -o main
./main
```

输出：

```
你好，Qi语言！
```

---

## 14. 后续拓展方向

- 🧠 JIT 模式（使用 LLVM JIT）
- 📦 包中心（npm 类似）
- 🧾 类型推导 + 泛型
- 🧭 并发模型升级（async/await）
- 🌐 WebAssembly 目标（WASM 后端）

---

## ✅ 总结

| 模块                 | 技术栈                     |
| -------------------- | -------------------------- |
| 前端（Lexer/Parser） | Rust + pest/chumsky        |
| AST / IR             | Rust + inkwell (LLVM)      |
| 后端                 | LLVM IR → 原生代码         |
| 并发运行时           | C + pthread + FFI          |
| 包系统               | JSON + 本地缓存 + 远程拉取 |
| 标准库               | `.qi` + 运行时接口         |

---

1. 📐 `parser.rs` 的完整代码骨架（Rust）
2. 🧠 `codegen.rs` LLVM 生成函数
3. 🧰 runtime C 库实现全套（含协程）
