基于前文完整的扩展开发内容，整理出 **仓颉 Zed 扩展（v1.0.0）** 的标准文件目录结构，包含所有核心模块、配置文件、静态资源及辅助脚本，目录层级清晰且与功能模块一一对应：

```
cangjie-zed-extension/
├── src/                      # 核心源代码目录
│   ├── ai/                   # AI 辅助开发模块
│   │   ├── mod.rs            # AI 模块入口（管理器、核心逻辑）
│   │   ├── completion.rs     # AI 代码补全功能
│   │   ├── law_conflict.rs   # AI 法则冲突检测
│   │   ├── magic_generate.rs # AI 魔术方法生成
│   │   └── tests/            # AI 模块单元测试
│   │       └── mod.rs
│   ├── cli/                  # Cangjie CLI 工具链集成模块
│   │   ├── mod.rs            # CLI 管理器、命令构建与执行
│   │   ├── commands.rs       # CLI 命令参数定义
│   │   ├── result.rs         # CLI 执行结果处理
│   │   └── tests/            # CLI 模块单元测试
│   │       └── mod.rs
│   ├── commands/             # 全局命令处理器
│   │   ├── mod.rs            # 命令注册入口
│   │   ├── ai_commands.rs    # AI 相关命令处理
│   │   ├── cli_commands.rs   # CLI 相关命令处理
│   │   ├── debug_commands.rs # 调试相关命令处理
│   │   └── perf_commands.rs  # 性能优化相关命令处理
│   ├── debugger/             # 调试核心模块
│   │   ├── mod.rs            # 调试模块入口
│   │   ├── single_cosmos.rs  # 单宇宙调试逻辑
│   │   ├── multi_cosmos.rs   # 多宇宙并行调试
│   │   ├── breakpoint.rs     # 断点管理（法则/魔术方法）
│   │   ├── lazy_load.rs      # 调试数据懒加载
│   │   ├── comparison.rs     # 多宇宙对比分析
│   │   └── tests/            # 调试模块单元测试
│   │       └── mod.rs
│   ├── lsp/                  # LSP 协议实现模块
│   │   ├── mod.rs            # LSP 入口（协议绑定）
│   │   ├── completion.rs     # 代码补全（基础+AI 融合）
│   │   ├── definition.rs     # 定义跳转（含热点缓存）
│   │   ├── diagnostics.rs    # 语法/法则诊断（含 AI 辅助）
│   │   ├── code_action.rs    # 代码操作（重构、修复）
│   │   └── tests/            # LSP 模块单元测试
│   │       └── mod.rs
│   ├── optimization/         # 性能优化模块
│   │   ├── mod.rs            # 性能优化入口（管理器）
│   │   ├── incremental_parse.rs # 文档增量解析
│   │   ├── hot_symbol.rs     # 热点符号缓存
│   │   ├── metrics.rs        # 性能指标统计
│   │   └── tests/            # 性能优化模块单元测试
│   │       └── mod.rs
│   ├── syntax/               # 仓颉语法支持模块
│   │   ├── mod.rs            # 语法模块入口
│   │   ├── highlighting.rs   # 语法高亮配置
│   │   ├── parser.rs         # 简化语法解析器（适配增量解析）
│   │   └── tests/            # 语法模块单元测试
│   │       └── mod.rs
│   ├── ui/                   # 自定义 UI 组件模块
│   │   ├── mod.rs            # UI 模块入口
│   │   ├── notifications.rs  # 通知提示组件
│   │   ├── dialogs.rs        # 对话框组件（配置、确认等）
│   │   ├── visualizer.rs     # 宇宙演化/对比可视化面板
│   │   └── tests/            # UI 模块单元测试
│   │       └── mod.rs
│   ├── utils/                # 通用工具函数模块
│   │   ├── mod.rs            # 工具模块入口
│   │   ├── config.rs         # 配置读取/解析
│   │   ├── logger.rs         # 日志工具
│   │   ├── path.rs           # 路径处理（跨平台适配）
│   │   ├── serde.rs          # 序列化/反序列化工具
│   │   └── tests/            # 工具模块单元测试
│   │       └── mod.rs
│   ├── lib.rs                # 扩展入口文件（初始化、模块注册）
│   └── main.rs               # 开发调试入口（可选）
├── assets/                   # 静态资源目录
│   ├── icons/                # 图标资源（SVG/PNG）
│   │   ├── cangjie-logo.svg  # 仓颉品牌图标
│   │   ├── debug/            # 调试相关图标
│   │   │   ├── cosmos-session.svg
│   │   │   └── compare-cosmos.svg
│   │   ├── ai/               # AI 功能图标
│   │   │   ├── ai-completion.svg
│   │   │   └── ai-refactor.svg
│   │   └── cli/              # CLI 功能图标
│   │       ├── build-cosmos.svg
│   │       └── validate-cosmos.svg
│   ├── syntax/               # 语法高亮配置文件
│   │   └── cangjie.tmLanguage.json # TextMate 语法规则
│   └── styles/               # 自定义 UI 样式
│       └── visualizer.css    # 可视化面板样式
├── tests/                    # 集成测试目录
│   ├── integration/          # 端到端集成测试
│   │   ├── lsp_test.rs       # LSP 功能集成测试
│   │   ├── debug_test.rs     # 调试功能集成测试
│   │   └── cli_test.rs       # CLI 集成测试
│   └── fixtures/             # 测试 fixtures（测试用例文件）
│       ├── test.cosmic.law   # 测试用法则文件
│       ├── test.cosmos       # 测试用宇宙文件
│       └── cosmos.toml       # 测试用宇宙配置文件
├── Cargo.toml                # Rust 项目配置（依赖、构建规则）
├── Cargo.lock                # Rust 依赖版本锁定文件
├── package.json              # Node.js 配置（Zed 扩展打包、脚本）
├── package-lock.json         # Node.js 依赖版本锁定文件
├── extension.json            # Zed 扩展清单（必填，声明扩展信息）
├── settings.schema.json      # 扩展配置 JSON Schema（IDE 自动提示）
├── keybindings.json          # 默认快捷键配置（可被用户覆盖）
├── README.md                 # 扩展说明文档（用户指南）
├── CONTRIBUTING.md           # 贡献指南（开发者文档）
├── LICENSE                   # 开源许可证（Apache 2.0）
├── CHANGELOG.md              # 版本更新日志
├── .gitignore                # Git 忽略文件配置
├── rustfmt.toml              # Rust 代码格式化配置
└── scripts/                  # 辅助脚本
    ├── build.sh              # 构建脚本（Linux/macOS）
    ├── build.ps1             # 构建脚本（Windows）
    ├── test.sh               # 测试脚本（Linux/macOS）
    └── test.ps1              # 测试脚本（Windows）
```

### 核心目录说明
1. **`src/`**：所有业务逻辑的核心目录，按功能模块拆分，每个子模块独立负责一类功能，便于维护和扩展；
2. **`assets/`**：存储静态资源，包含图标、语法高亮规则、UI 样式等，与代码分离，便于资源管理；
3. **`tests/`**：包含单元测试（模块内 `tests/` 目录）和集成测试（顶层 `tests/` 目录），确保功能稳定性；
4. **配置文件**：`extension.json`（Zed 扩展必需清单）、`settings.schema.json`（配置自动提示）、`keybindings.json`（默认快捷键）是扩展运行的核心配置；
5. **脚本目录**：`scripts/` 提供跨平台构建/测试脚本，简化开发流程。

该目录结构严格遵循 Rust 项目规范和 Zed 扩展开发最佳实践，同时适配仓颉生态的功能扩展需求，可直接用于实际开发和部署。

要将 **仓颉 Zed 扩展项目** 完整压缩并上传到云盘，需按「项目清理 → 压缩打包 → 云盘上传」三步操作，确保压缩包完整、无冗余文件，同时适配主流云盘（百度网盘、阿里云盘、OneDrive 等）。以下是详细操作指南：

### 一、前提准备
1. **本地环境**：确保已安装压缩工具（Windows 自带「发送到压缩文件夹」、macOS 自带「压缩」、Linux 用 `tar` 命令）；
2. **项目状态**：已完成开发/测试，无未提交的临时文件（建议先执行 `git clean -fd` 清理 Git 未跟踪文件）；
3. **云盘账号**：已登录目标云盘（如百度网盘、阿里云盘），确保有足够存储空间（项目压缩后约 5-10MB，含代码+资源）。

### 二、项目清理（可选但推荐）
压缩前清理冗余文件，减少压缩包体积：
```bash
# 进入项目根目录
cd cangjie-zed-extension

# 1. 清理 Rust 构建产物（最大冗余来源）
cargo clean

# 2. 清理 Node.js 依赖（打包后无需携带，部署时重新安装）
rm -rf node_modules package-lock.json

# 3. 清理测试临时文件
rm -rf target/tests/tmp
rm -rf tests/tmp

# 4. 清理日志/缓存文件
rm -rf .logs .cache
```

### 三、压缩打包（按系统选择操作）
#### 1. Windows 系统
1. 打开文件资源管理器，定位到 `cangjie-zed-extension` 项目根目录；
2. 右键点击项目文件夹 → 选择「发送到」→「压缩(zipped)文件夹」；
3. 等待压缩完成，生成 `cangjie-zed-extension.zip`（默认名称，可自定义）。

#### 2. macOS 系统
1. 打开访达，定位到 `cangjie-zed-extension` 项目根目录；
2. 右键点击项目文件夹 → 选择「压缩“cangjie-zed-extension”」；
3. 生成 `cangjie-zed-extension.zip` 压缩包（自动命名）。

#### 3. Linux 系统（终端命令）
```bash
# 进入项目上级目录
cd ..

# 压缩项目（排除冗余文件，仅保留核心文件）
tar -zcvf cangjie-zed-extension.tar.gz \
  --exclude="target" \
  --exclude="node_modules" \
  --exclude=".git" \
  --exclude=".logs" \
  cangjie-zed-extension/
```
- 生成 `cangjie-zed-extension.tar.gz`（Linux 常用压缩格式，体积更小）；
- 若需兼容 Windows，可改用 `zip` 命令：`zip -r cangjie-zed-extension.zip cangjie-zed-extension/ -x "*/target/*" "*/node_modules/*" "*/.git/*"`。

### 四、云盘上传（以主流云盘为例）
#### 1. 百度网盘
1. 打开百度网盘客户端（或网页版）；
2. 新建文件夹（建议命名「仓颉 Zed 扩展_v1.0.0」）；
3. 拖拽本地压缩包（`cangjie-zed-extension.zip`）到该文件夹；
4. 等待上传完成（5-10MB 约 10 秒内完成）。

#### 2. 阿里云盘
1. 打开阿里云盘客户端（或网页版）；
2. 点击「上传」→「文件」，选择本地压缩包；
3. 可选：设置文件访问权限（公开/私有），生成分享链接（如需协作）。

#### 3. OneDrive
1. 打开 OneDrive 客户端，定位到目标目录（如「开发项目/仓颉生态」）；
2. 拖拽本地压缩包到该目录，自动同步上传；
3. 同步完成后，可通过 OneDrive 分享功能发送给协作成员。

### 五、压缩包内容校验（确保完整）
上传后建议下载测试，或通过云盘「预览」功能确认压缩包包含以下核心文件（避免遗漏）：
```
cangjie-zed-extension/
├── src/（完整源代码目录）
├── assets/（静态资源目录）
├── tests/（测试目录）
├── Cargo.toml、Cargo.lock
├── package.json、extension.json
├── settings.schema.json、keybindings.json
├── README.md、LICENSE、CHANGELOG.md
```

### 六、注意事项
1. **版本命名**：压缩包/云盘文件夹建议添加版本号（如 `cangjie-zed-extension_v1.0.0.zip`），便于后续版本管理；
2. **权限设置**：若为公开分享，建议设置「只读权限」，避免文件被篡改；
3. **备份策略**：重要版本建议在多个云盘备份（如百度网盘+阿里云盘），防止数据丢失；
4. **协作场景**：如需多人协作开发，可直接将项目上传到 Git 仓库（如 GitCode、GitHub），云盘仅作为离线备份。

完成以上步骤后，项目已完整存储到云盘，可随时下载、分享或部署使用！

要将 **仓颉 Zed 扩展项目压缩包** 上传到 **豆包云盘**，需遵循豆包云盘的官方操作流程（适配网页版/客户端），以下是 step-by-step 详细指南，确保上传完整且可快速访问：

### 一、前置准备（已完成可跳过）
1. 已按前文步骤完成项目压缩，得到压缩包（如 `cangjie-zed-extension_v1.0.0.zip`）；
2. 确保压缩包包含核心文件（`src/`、`assets/`、配置文件等，无冗余）；
3. 登录豆包账号（网页版/客户端需提前登录，确保有云盘使用权限）。

### 二、上传到豆包云盘（推荐网页版，操作更直观）
#### 步骤 1：打开豆包云盘
- 方式 1（网页版）：
  1. 打开浏览器，访问豆包云盘官方入口：https://www.doubao.com/cloud （或通过豆包官网 → 右上角「云盘」进入）；
  2. 输入豆包账号密码登录（支持手机号、第三方账号登录）。
- 方式 2（客户端）：
  1. 下载安装「豆包客户端」（官网：https://www.doubao.com/download ）；
  2. 登录后，点击左侧导航栏「云盘」进入云盘页面。

#### 步骤 2：创建专属文件夹（便于管理）
1. 在豆包云盘页面，点击左上角「新建文件夹」；
2. 文件夹命名规范：`仓颉 Zed 扩展_版本号`（如 `仓颉 Zed 扩展_v1.0.0`），避免与其他文件混淆；
3. 点击「确定」，生成专属存储文件夹（后续压缩包、相关文档可统一放在这里）。

#### 步骤 3：上传项目压缩包
1. 进入刚创建的文件夹，点击页面右上角「上传」按钮（图标为 ↑ 或「上传文件」）；
2. 在弹出的文件选择框中，找到本地的项目压缩包（如 `cangjie-zed-extension_v1.0.0.zip`），选中后点击「打开」；
3. 等待上传完成：
   - 压缩包体积约 5-10MB，网络正常情况下 10 秒内完成；
   - 上传过程中可查看进度条，避免关闭页面或断网。

#### 步骤 4：校验上传结果（关键）
1. 上传完成后，文件夹内会显示压缩包文件，且文件名、大小与本地一致；
2. 可选操作：
   - 点击压缩包右侧「预览」，确认文件可正常读取（豆包云盘支持 zip/tar.gz 格式预览）；
   - 点击「下载」测试，确保压缩包无损坏（下载后解压，检查核心目录 `src/`、`assets/` 是否完整）。

### 三、进阶操作（可选）
#### 1. 设置文件权限（协作/分享场景）
1. 选中压缩包，点击右侧「分享」按钮；
2. 选择分享权限：
   - 「仅自己可见」：默认权限，仅登录账号可访问；
   - 「公开分享」：生成分享链接，任何人可通过链接下载；
   - 「指定人可见」：输入对方豆包账号，仅指定用户可访问；
3. 点击「生成链接」，复制后可分享给协作成员。

#### 2. 备份多版本（版本管理）
1. 后续更新扩展版本时，新建对应版本文件夹（如 `仓颉 Zed 扩展_v1.1.0`）；
2. 按相同流程上传新版本压缩包，避免覆盖旧版本，便于回滚测试。

#### 3. 关联项目文档（可选）
将 `README.md`、`CHANGELOG.md` 等文档单独上传到同一文件夹，便于查看项目说明，无需解压压缩包。

### 四、常见问题排查
1. 上传失败/进度卡住：
   - 检查网络连接，切换 Wi-Fi/有线网络；
   - 刷新页面后重新上传，若仍失败，可将压缩包分卷（适合极个别大文件，本项目无需）；
2. 压缩包损坏：
   - 本地重新压缩项目（确保执行 `cargo clean` 清理冗余）；
   - 重新上传，避免上传过程中中断网络；
3. 无法预览/下载：
   - 确认压缩包格式正确（推荐 zip 格式，豆包云盘对 tar.gz 支持稍弱）；
   - 联系豆包客服（页面右下角「客服」入口），反馈文件访问问题。

### 五、总结
通过以上步骤，项目已完整存储到豆包云盘，可实现：
- 离线备份：避免本地文件丢失；
- 便捷访问：任何设备登录豆包账号即可下载使用；
- 协作分享：生成链接即可与他人共享项目。

后续如需更新项目，只需重复「压缩 → 上传到对应版本文件夹」流程，保持版本管理规范即可！
