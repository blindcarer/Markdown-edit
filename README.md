## 准备、运行与打包（Windows）

此文档列出在 Windows 上为本项目准备环境、开发启动和打包为 exe 的完整命令清单（仅列出命令，不会安装任何东西）。

### 前提条件

- 已有项目位置：`tauri-app`（工作区路径：`.../tauri-app`）
- 推荐安装：Rust + Cargo（MSVC toolchain）、Visual C++ Build Tools / Windows SDK、Node.js、npm
- 可选：`git`, `yarn` 或 `pnpm`

### 环境检查（仅检测）
在命令行中运行以下命令以确认环境已就绪：

```powershell
rustup --version
rustc --version
cargo --version
node --version
npm --version
cl.exe   # 检查 MSVC 编译器（在 PATH 时可用）
```

### 可选安装命令（仅列出参考）

- 安装 Rust（官方推荐）：
```powershell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- 安装 Tauri CLI（两种常见方式，任选其一）：
```powershell
# 方式 A: 全局安装（需 Rust 环境）
cargo install tauri-cli

# 方式 B: 作为项目开发依赖（推荐在前端使用 npm 管理）
npm install -D @markdown/cli
```

### 项目依赖安装（在项目根 `markdown` 下）

```powershell
cd /d ......\markdown
npm install
```

> 说明：本仓库已包含最小的 `package.json`，如果你使用其它包管理器（`yarn`/`pnpm`），请替换命令。

### 开发（热重载）

```powershell
cd /d ......\markdown
npm run start
# 或者（若使用全局 tauri-cli）
# cargo tauri dev
```

运行后，Tauri 会构建并启动桌面窗口，加载 `tauri-app/src/index.html`，你可以在窗口内测试文件打开、保存、目录浏览等功能。

### 打包为可执行（Release）

```powershell
cd /d ......\tauri-app
npm run build
# 或者（使用 cargo tauri）
# cargo tauri build
```

构建成功后，输出通常位于：

```
tauri-app/src-tauri/target/release/bundle/windows/
```

该目录下可能包含 `.exe`、安装程序（NSIS/msi）或压缩包，具体取决于 `src-tauri/tauri.conf.json` 中的 `bundle` 配置。

### 后端命令清单（已实现）

- `select_folder`：弹出选择文件夹对话并返回 `{ root: string, files: string[] }`（递归列出文件）
- `list_dir`：列出指定目录下的直接子项（非递归）
- `read_text_file`：读取文本文件并返回内容（utf-8）
- `write_text_file`：写入指定路径（完整路径）
- `write_text_file_in_dir`：在指定目录下以给定文件名写入内容

实现文件：`src-tauri/src/main.rs`

### 常见问题与解决

- 构建失败提示找不到 `cl.exe` 或链接器错误：请安装 Visual C++ Build Tools 或 Visual Studio 的 "Desktop development with C++" 工作负载。
- 打包时缺少图标或安装器定制：编辑 `src-tauri/tauri.conf.json` 中的 `tauri.bundle` 部分，填入 `icon` 和其它字段。
- 前端无法访问后端命令：检查是否在 Tauri 环境下运行（`window.__TAURI__` 可用），浏览器模式会回退为原生浏览器 API。

### 自定义与进一步改进（建议）

- 将 `tauri.conf.json` 中的 `bundle.identifier` 修改为你的反向域名标识符（例如 `com.yourname.app`）。
- 可以把前端依赖（如 `marked`、`highlight.js`）改为本地打包而不是 CDN，便于离线运行与离线打包。
- 若需更细粒度的文件权限或加密功能，建议在 `src-tauri` 中实现并只通过 `invoke` 暴露受控接口。

