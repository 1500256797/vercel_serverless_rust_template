## 0x1 配置环境

安装 vercel 命令行工具

```
yarn global add vercel
```

安装 cross 交叉编译工具

macOS m1 交叉编译到 x86_64 linux
教程：https://github.com/chinedufn/cross-compile-rust-from-mac-to-linux

```
 rustup target add x86_64-unknown-linux-gnu

brew install x86_64-unknown-linux-gnu


```
