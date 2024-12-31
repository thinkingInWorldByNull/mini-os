
**编译**

整个编译过程包含二步：编译内核以及将内核打包成启动镜像

```shell
cd kernel
cargo build --target x86_64-unknown-none

cd ..
CARGO_BUILD_PRINT_SCRIPTS=1 cargo build
cargo run
```
