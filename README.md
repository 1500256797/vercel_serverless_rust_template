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

## 本地启动

## 线上部署

```
vercel build && vercel deploy --prebuilt
```

参数如下：

```
~/Desktop/vercel_serverless_rust_template[main~] npx vercel build && npx  vercel deploy --prebuilt
Vercel CLI 39.1.2
? No Project Settings found locally. Run `vercel pull` for retrieving them? yes
? Set up “~/Desktop/vercel_serverless_rust_template”? yes
? Which scope should contain your project? 1500256797's projects
? Link to existing project? no
? What’s your project’s name? vercel-serverless-rust-template
? In which directory is your code located? ./
Local settings detected in vercel.json:
No framework detected. Default Project Settings:
- Build Command: `npm run vercel-build` or `npm run build`
- Development Command: None
- Install Command: `yarn install`, `pnpm install`, `npm install`, or `bun install`
- Output Directory: `public` if it exists, or `.`
? Want to modify these settings? no
```
