#!/bin/sh

# 默认不进行清理
CLEAN=false
RUN=false

# 解析命令行参数
for arg in "$@"; do
  case $arg in
    --clean | -c)
      CLEAN=true
      shift
      ;;
    --run | -r)
      RUN=true
      shift
      ;;
    --help | -h)
      echo "Usage: $0 [-clean] [-run]"
            echo "--clean or: clean"
            echo "--run: run the image default only build, Note! must install qemu"
            exit 0
            ;;
    -*|--*)
      echo "Unknown option $arg"
      exit 1
      ;;
    *)
      # 非选项参数
      break
      ;;
  esac
done


######################### build kernel start
cd kernel
if [ "$CLEAN" = true ]; then
    echo "Cleaning main project..."
    cargo clean
fi
cargo build --target x86_64-unknown-none
######################### build kernel end


######################### build elf / bios image start
cd ../
cargo build

if [ "$RUN" = true ]; then
    echo "Run the image..."
    cargo run
fi
######################### build elf / bios image end