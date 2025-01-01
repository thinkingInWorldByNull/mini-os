
**先决条件**

- 安装`qemu`
- 安装必要的rust编译工具

```shell
rustup override set nightly
rustup component add rust-src
rustup component add llvm-tools-preview
```

**编译**

```shell
sh build_utils.sh --clean --build
```

**运行**

>❗️必须要先安装`qemu`模拟器

```shell
cargo run
```