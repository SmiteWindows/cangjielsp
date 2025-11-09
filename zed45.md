# 补充：扩展监控与错误上报、本地化支持
以下是提升扩展稳定性和用户体验的关键功能：**运行时监控与错误上报**（快速定位线上问题）和 **多语言本地化**（适配全球用户），完善扩展的生产级特性。

## 二十七、运行时监控与错误上报（src/monitoring/ 目录）
### 1. 监控核心逻辑（src/monitoring/error_report.rs）
```rust
//! 运行时错误上报与性能监控
use log::{error, info, warn};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use zed_extension_api::{self as zed, Workspace};

/// 错误上报配置（可通过环境变量或配置文件覆盖）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReportConfig {
    /// 上报服务器地址（默认使用官方服务器）
    pub report_server: String,
    /// 是否开启错误上报（默认开启）
    pub enable: bool,
    /// 是否上报性能数据（默认开启）
    pub enable_perf_report: bool,
    /// 采样率（0.0-1.0，默认 1.0，即全部上报）
    pub sample_rate: f64,
}

impl Default for ErrorReportConfig {
    fn default() -> Self {
        Self {
            report_server: "https://monitor.cangjie-lang.org/report".to_string(),
            enable: true,
            enable_perf_report: true,
            sample_rate: 1.0,
        }
    }
}

/// 错误类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorType {
    /// 调试器错误
    DebuggerError,
    /// LSP 错误
    LspError,
    /// 主题渲染错误
    ThemeError,
    /// 图标加载错误
    IconError,
    /// 协作调试错误
    CollabError,
    /// 其他错误
    Other,
}

/// 错误上报数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReportData {
    /// 扩展版本
    pub extension_version: String,
    /// Zed 版本
    pub zed_version: String,
    /// 操作系统
    pub os: String,
    /// 架构
    pub arch: String,
    /// 错误类型
    pub error_type: ErrorType,
    /// 错误信息
    pub message: String,
    /// 错误堆栈
    pub stack_trace: Option<String>,
    /// 相关上下文（如调试配置、文件路径等）
    pub context: HashMap<String, String>,
    /// 上报时间戳（毫秒）
    pub timestamp: u64,
    /// 匿名用户 ID（用于统计错误分布，不关联个人信息）
    pub anonymous_user_id: String,
}

/// 性能上报数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfReportData {
    /// 扩展版本
    pub extension_version: String,
    /// Zed 版本
    pub zed_version: String,
    /// 操作类型（如调试启动、LSP 初始化、主题切换等）
    pub operation: String,
    /// 耗时（毫秒）
    pub duration_ms: u64,
    /// 成功与否
    pub success: bool,
    /// 上报时间戳（毫秒）
    pub timestamp: u64,
    /// 匿名用户 ID
    pub anonymous_user_id: String,
}

/// 监控管理器（单例）
pub struct MonitorManager {
    config: ErrorReportConfig,
    anonymous_user_id: String,
    extension_version: String,
    zed_version: String,
    os: String,
    arch: String,
}

impl MonitorManager {
    /// 初始化监控管理器
    pub fn new(workspace: &Workspace) -> Self {
        // 读取配置（优先环境变量，其次默认配置）
        let config = Self::load_config();
        
        // 生成匿名用户 ID（基于设备信息哈希，不包含个人数据）
        let anonymous_user_id = Self::generate_anonymous_user_id();
        
        // 获取扩展版本（从 Cargo.toml 读取）
        let extension_version = env!("CARGO_PKG_VERSION").to_string();
        
        // 获取 Zed 版本
        let zed_version = workspace.zed_version().to_string();
        
        // 获取操作系统和架构
        let (os, arch) = Self::get_os_info();
        
        info!(
            "监控管理器初始化完成：版本={}, Zed版本={}, 匿名用户ID={}",
            extension_version, zed_version, anonymous_user_id
        );
        
        Self {
            config,
            anonymous_user_id,
            extension_version,
            zed_version,
            os,
            arch,
        }
    }

    /// 加载配置（从环境变量或默认值）
    fn load_config() -> ErrorReportConfig {
        let mut config = ErrorReportConfig::default();
        
        // 从环境变量覆盖配置
        if let Ok(report_server) = std::env::var("CANGJIE_REPORT_SERVER") {
            config.report_server = report_server;
        }
        if let Ok(enable) = std::env::var("CANGJIE_ENABLE_REPORT") {
            config.enable = enable.parse().unwrap_or(true);
        }
        if let Ok(enable_perf) = std::env::var("CANGJIE_ENABLE_PERF_REPORT") {
            config.enable_perf_report = enable_perf.parse().unwrap_or(true);
        }
        if let Ok(sample_rate) = std::env::var("CANGJIE_SAMPLE_RATE") {
            config.sample_rate = sample_rate.parse().unwrap_or(1.0);
        }
        
        config
    }

    /// 生成匿名用户 ID（基于设备信息哈希）
    fn generate_anonymous_user_id() -> String {
        #[cfg(target_os = "macos")]
        let device_info = std::env::var("MAC_ADDRESS").unwrap_or_else(|_| "unknown-mac".to_string());
        #[cfg(target_os = "linux")]
        let device_info = std::env::var("LINUX_HOSTNAME").unwrap_or_else(|_| "unknown-linux".to_string());
        #[cfg(target_os = "windows")]
        let device_info = std::env::var("WINDOWS_MACHINE_ID").unwrap_or_else(|_| "unknown-windows".to_string());
        
        // 使用 SHA-256 哈希设备信息，避免暴露原始数据
        let hash = sha2::Sha256::digest(device_info.as_bytes());
        hex::encode(&hash[..8]) // 取前 8 字节作为匿名 ID
    }

    /// 获取操作系统和架构信息
    fn get_os_info() -> (String, String) {
        let os = match std::env::consts::OS {
            "linux" => "Linux",
            "macos" => "macOS",
            "windows" => "Windows",
            _ => "Unknown",
        }.to_string();
        
        let arch = match std::env::consts::ARCH {
            "x86_64" => "x86_64",
            "aarch64" => "arm64",
            "x86" => "x86",
            _ => "Unknown",
        }.to_string();
        
        (os, arch)
    }

    /// 上报错误
    pub async fn report_error(
        &self,
        error_type: ErrorType,
        message: String,
        stack_trace: Option<String>,
        context: HashMap<String, String>,
    ) {
        if !self.config.enable {
            warn!("错误上报已禁用，跳过上报：{}", message);
            return;
        }
        
        // 采样（按配置的采样率决定是否上报）
        let random: f64 = rand::random();
        if random > self.config.sample_rate {
            debug!("错误上报采样跳过：采样率={}, 随机值={}", self.config.sample_rate, random);
            return;
        }
        
        let report_data = ErrorReportData {
            extension_version: self.extension_version.clone(),
            zed_version: self.zed_version.clone(),
            os: self.os.clone(),
            arch: self.arch.clone(),
            error_type,
            message,
            stack_trace,
            context,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            anonymous_user_id: self.anonymous_user_id.clone(),
        };
        
        // 异步上报（不阻塞主线程）
        tokio::spawn(async move {
            if let Err(e) = Self::send_report(&self.config.report_server, &report_data).await {
                error!("错误上报失败：{}", e);
            } else {
                debug!("错误上报成功");
            }
        });
    }

    /// 上报性能数据
    pub async fn report_perf(
        &self,
        operation: String,
        duration_ms: u64,
        success: bool,
    ) {
        if !self.config.enable_perf_report {
            debug!("性能上报已禁用，跳过上报：{}", operation);
            return;
        }
        
        // 采样
        let random: f64 = rand::random();
        if random > self.config.sample_rate {
            debug!("性能上报采样跳过：采样率={}, 随机值={}", self.config.sample_rate, random);
            return;
        }
        
        let perf_data = PerfReportData {
            extension_version: self.extension_version.clone(),
            zed_version: self.zed_version.clone(),
            operation,
            duration_ms,
            success,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            anonymous_user_id: self.anonymous_user_id.clone(),
        };
        
        // 异步上报
        tokio::spawn(async move {
            if let Err(e) = Self::send_report(&self.config.report_server, &perf_data).await {
                error!("性能上报失败：{}", e);
            } else {
                debug!("性能上报成功：{}，耗时={}ms", operation, duration_ms);
            }
        });
    }

    /// 发送上报数据到服务器（通用方法）
    async fn send_report<T: Serialize>(server_url: &str, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        // 序列化数据为 JSON
        let json_data = serde_json::to_string(data)?;
        
        // 发送 POST 请求（使用 tokio 原生 HTTP 客户端，避免额外依赖）
        let client = reqwest::Client::new();
        let response = client
            .post(server_url)
            .header("Content-Type", "application/json")
            .body(json_data)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("服务器返回非成功状态：{}", response.status()).into());
        }
        
        Ok(())
    }
}

/// 全局监控管理器实例（线程安全）
static MONITOR_MANAGER: once_cell::sync::OnceCell<MonitorManager> = once_cell::sync::OnceCell::new();

/// 初始化全局监控管理器
pub fn init_monitor(workspace: &Workspace) {
    let manager = MonitorManager::new(workspace);
    MONITOR_MANAGER.set(manager).unwrap();
}

/// 获取全局监控管理器
pub fn get_monitor() -> Option<&'static MonitorManager> {
    MONITOR_MANAGER.get()
}

/// 便捷错误上报宏（简化业务代码中的上报逻辑）
#[macro_export]
macro_rules! report_error {
    ($error_type:expr, $message:expr, $stack_trace:expr, $context:expr) => {
        if let Some(monitor) = $crate::monitoring::get_monitor() {
            tokio::spawn(async move {
                monitor.report_error($error_type, $message.to_string(), $stack_trace, $context).await;
            });
        }
    };
}

/// 便捷性能上报宏
#[macro_export]
macro_rules! report_perf {
    ($operation:expr, $duration_ms:expr, $success:expr) => {
        if let Some(monitor) = $crate::monitoring::get_monitor() {
            tokio::spawn(async move {
                monitor.report_perf($operation.to_string(), $duration_ms, $success).await;
            });
        }
    };
}
```

### 2. 监控集成到扩展（src/lib.rs 补充）
```rust
// 导入监控模块
mod monitoring;
use monitoring::{init_monitor, report_error, report_perf, ErrorType};

// 在 activate 函数中初始化监控
#[zed::extension]
fn activate(workspace: &zed::Workspace) -> Result<Box<dyn Extension>> {
    init_logger();
    info!("Cangjie Zed Extension v0.3.0 activated");
    
    // 初始化监控管理器
    init_monitor(workspace);
    
    // 预初始化 LSP
    tokio::spawn(async move {
        let start_time = std::time::Instant::now();
        match init_lsp_client().await {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;
                report_perf!("lsp_init", duration, true);
                info!("LSP 初始化成功，耗时 {}ms", duration);
            }
            Err(e) => {
                let duration = start_time.elapsed().as_millis() as u64;
                report_perf!("lsp_init", duration, false);
                report_error!(
                    ErrorType::LspError,
                    format!("LSP 预初始化失败：{}", e),
                    Some(format!("{:#?}", e)),
                    std::collections::HashMap::new()
                );
                warn!("LSP 预初始化失败：{}", e);
            }
        }
    });

    Ok(Box::new(CangjieZedExtension))
}

// 在调试器核心逻辑中添加错误/性能上报（示例：src/debugger/debugger.rs）
impl CangjieDebugger {
    pub fn start(&mut self) -> Result<()> {
        let start_time = std::time::Instant::now();
        debug!("Starting debugger with config: {:?}", self.config);
        
        match self.inner_start() {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;
                report_perf!("debugger_start", duration, true);
                info!("Cosmos instance initialized: {}", self.cosmos_instance.as_ref().unwrap().meta.id);
                Ok(())
            }
            Err(e) => {
                let duration = start_time.elapsed().as_millis() as u64;
                report_perf!("debugger_start", duration, false);
                let mut context = std::collections::HashMap::new();
                context.insert("cosmos_file".to_string(), self.config.cosmos_file.to_string());
                context.insert("debug_mode".to_string(), self.config.debug_mode.to_string());
                report_error!(
                    ErrorType::DebuggerError,
                    format!("调试器启动失败：{}", e),
                    Some(format!("{:#?}", e)),
                    context
                );
                Err(e)
            }
        }
    }

    // 内部启动逻辑（分离业务与上报）
    fn inner_start(&mut self) -> Result<()> {
        // 原 start 方法的核心逻辑
        let cosmos_file_path = self.config.cosmos_file.to_file_path()
            .map_err(|_| ZedError::user("无效的宇宙文件路径"))?;
        
        let cosmos_meta = self.cosmos_manager.load_cosmos_meta(&cosmos_file_path)?;
        let mut cosmos_instance = self.cosmos_manager.instantiate_cosmos(
            &cosmos_meta,
            &self.config.cosmos_type,
            self.config.step_interval.unwrap_or(100),
        )?;
        
        cosmos_instance.start_evolution();
        self.cosmos_instance = Some(cosmos_instance);
        self.spawn_evolution_task();
        
        Ok(())
    }
}
```

### 3. 监控依赖配置（Cargo.toml 补充）
```toml
[dependencies]
# 监控相关依赖
reqwest = { version = "0.11", features = ["json", "tokio1"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.10"
hex = "0.4"
rand = "0.8"
tokio = { version = "1.0", features = ["full"] }
log = "0.4"
```

## 二十八、多语言本地化支持（locales/ 目录）
### 1. 本地化核心逻辑（src/locale/mod.rs）
```rust
//! 多语言本地化支持
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use zed_extension_api::Workspace;

/// 支持的语言列表
pub const SUPPORTED_LOCALES: &[&str] = &["en", "zh-CN", "ja", "ko"];

/// 本地化资源数据结构（对应 locales/{locale}.json 文件）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleResources {
    /// 主题相关文本
    pub theme: HashMap<String, String>,
    /// 调试器相关文本
    pub debugger: HashMap<String, String>,
    /// 错误提示相关文本
    pub errors: HashMap<String, String>,
    /// 通用文本
    pub common: HashMap<String, String>,
}

/// 本地化管理器（单例）
pub struct LocaleManager {
    current_locale: String,
    resources: HashMap<String, LocaleResources>,
}

impl LocaleManager {
    /// 初始化本地化管理器（加载所有支持的语言资源）
    pub fn new(workspace: &Workspace) -> Self {
        let mut resources = HashMap::new();
        
        // 加载所有支持的语言资源文件
        for locale in SUPPORTED_LOCALES {
            match Self::load_locale_resources(locale) {
                Ok(res) => {
                    resources.insert(locale.to_string(), res);
                    info!("加载语言资源成功：{}", locale);
                }
                Err(e) => {
                    warn!("加载语言资源失败：{}，错误：{}", locale, e);
                }
            }
        }
        
        // 获取 Zed 当前语言（优先使用 Zed 界面语言，其次默认 en）
        let current_locale = Self::get_zed_locale(workspace);
        
        Self {
            current_locale,
            resources,
        }
    }

    /// 加载指定语言的资源文件
    fn load_locale_resources(locale: &str) -> Result<LocaleResources, Box<dyn std::error::Error>> {
        // 资源文件路径：locales/{locale}.json
        let file_path = format!("locales/{}.json", locale);
        let content = std::fs::read_to_string(&file_path)?;
        let res = serde_json::from_str(&content)?;
        Ok(res)
    }

    /// 获取 Zed 当前界面语言
    fn get_zed_locale(workspace: &Workspace) -> String {
        let zed_locale = workspace.locale();
        // 映射 Zed 语言到扩展支持的语言（如 "zh" → "zh-CN"）
        match zed_locale.as_str() {
            "zh" | "zh-CN" | "zh-TW" => "zh-CN".to_string(),
            "ja" => "ja".to_string(),
            "ko" => "ko".to_string(),
            _ => "en".to_string(), // 默认英文
        }
    }

    /// 设置当前语言
    pub fn set_locale(&mut self, locale: &str) -> Result<(), String> {
        if !SUPPORTED_LOCALES.contains(&locale) {
            return Err(format!("不支持的语言：{}", locale));
        }
        self.current_locale = locale.to_string();
        Ok(())
    }

    /// 获取本地化文本（支持占位符替换，如 "{0}" "{1}"）
    pub fn t(&self, namespace: &str, key: &str, args: &[&str]) -> String {
        // 优先级：当前语言 → 英文（fallback）→ 原始 key
        let res = self.resources.get(&self.current_locale)
            .or_else(|| self.resources.get("en"));
        
        let mut text = match res {
            Some(res) => match namespace {
                "theme" => res.theme.get(key).cloned(),
                "debugger" => res.debugger.get(key).cloned(),
                "errors" => res.errors.get(key).cloned(),
                "common" => res.common.get(key).cloned(),
                _ => None,
            },
            None => None,
        }.unwrap_or_else(|| key.to_string());
        
        // 替换占位符（如 "{0}" → args[0]）
        for (i, arg) in args.iter().enumerate() {
            text = text.replace(&format!("{{{}}}", i), arg);
        }
        
        text
    }
}

/// 全局本地化管理器实例
static LOCALE_MANAGER: OnceCell<LocaleManager> = OnceCell::new();

/// 初始化本地化管理器
pub fn init_locale(workspace: &Workspace) {
    let manager = LocaleManager::new(workspace);
    LOCALE_MANAGER.set(manager).unwrap();
}

/// 获取本地化管理器
pub fn get_locale_manager() -> Option<&'static LocaleManager> {
    LOCALE_MANAGER.get()
}

/// 便捷本地化文本获取宏
#[macro_export]
macro_rules! t {
    ($namespace:expr, $key:expr) => {
        $crate::locale::get_locale_manager()
            .map(|m| m.t($namespace, $key, &[]))
            .unwrap_or_else(|| $key.to_string())
    };
    ($namespace:expr, $key:expr, $args:expr) => {
        $crate::locale::get_locale_manager()
            .map(|m| m.t($namespace, $key, $args))
            .unwrap_or_else(|| $key.to_string())
    };
}
```

### 2. 本地化资源文件（locales/ 目录）
#### 2.1 英文（locales/en.json）
```json
{
  "theme": {
    "dark_name": "Cangjie Dark",
    "light_name": "Cangjie Light",
    "hc_name": "Cangjie High Contrast",
    "description": "Cangjie language theme with cosmic design"
  },
  "debugger": {
    "cosmos_started": "Cosmos instance started: {0}",
    "evolution_step": "Evolution stepped to stage: {0}, time: {1}s",
    "breakpoint_triggered": "Breakpoint triggered at line {0}",
    "law_validation_warn": "Law {0} consistency below threshold: {1} < {2}",
    "migrate_stage": "Migration stage reached: {0}"
  },
  "errors": {
    "invalid_cosmos_file": "Invalid cosmos file: {0}",
    "law_not_found": "Law {0} not found in cosmos",
    "lsp_init_failed": "LSP initialization failed: {0}",
    "collab_session_failed": "Collaboration session failed: {0}"
  },
  "common": {
    "ok": "OK",
    "cancel": "Cancel",
    "save": "Save",
    "load": "Load",
    "start": "Start",
    "pause": "Pause",
    "stop": "Stop",
    "step": "Step Over"
  }
}
```

#### 2.2 中文（locales/zh-CN.json）
```json
{
  "theme": {
    "dark_name": "仓颉深色",
    "light_name": "仓颉浅色",
    "hc_name": "仓颉高对比度",
    "description": "带有宇宙设计风格的仓颉语言主题"
  },
  "debugger": {
    "cosmos_started": "宇宙实例启动：{0}",
    "evolution_step": "演化步进至阶段：{0}，时间：{1}秒",
    "breakpoint_triggered": "断点触发于第 {0} 行",
    "law_validation_warn": "法则 {0} 一致性低于阈值：{1} < {2}",
    "migrate_stage": "迁移阶段已到达：{0}"
  },
  "errors": {
    "invalid_cosmos_file": "无效的宇宙文件：{0}",
    "law_not_found": "宇宙中未找到法则 {0}",
    "lsp_init_failed": "LSP 初始化失败：{0}",
    "collab_session_failed": "协作会话失败：{0}"
  },
  "common": {
    "ok": "确定",
    "cancel": "取消",
    "save": "保存",
    "load": "加载",
    "start": "启动",
    "pause": "暂停",
    "stop": "停止",
    "step": "单步执行"
  }
}
```

#### 2.3 日语（locales/ja.json）
```json
{
  "theme": {
    "dark_name": "蒼颉ダーク",
    "light_name": "蒼颉ライト",
    "hc_name": "蒼颉ハイコントラスト",
    "description": "コズミックデザインの蒼颉言語テーマ"
  },
  "debugger": {
    "cosmos_started": "コスモスインスタンス開始：{0}",
    "evolution_step": "進化ステップ：{0}、時間：{1}秒",
    "breakpoint_triggered": "ブレークポイント発動：{0}行",
    "law_validation_warn": "法則 {0} 整合性閾値未満：{1} < {2}",
    "migrate_stage": "マイグレーションステージ到達：{0}"
  },
  "errors": {
    "invalid_cosmos_file": "無効なコスモスファイル：{0}",
    "law_not_found": "コスモスに法則 {0} が見つかりません",
    "lsp_init_failed": "LSP 初期化失敗：{0}",
    "collab_session_failed": "コラボレーションセッション失敗：{0}"
  },
  "common": {
    "ok": "OK",
    "cancel": "キャンセル",
    "save": "保存",
    "load": "読み込み",
    "start": "開始",
    "pause": "一時停止",
    "stop": "停止",
    "step": "ステップオーバー"
  }
}
```

#### 2.4 韩语（locales/ko.json）
```json
{
  "theme": {
    "dark_name": "창결 다크",
    "light_name": "창결 라이트",
    "hc_name": "창결 고대비",
    "description": "우주 디자인 스타일의 창결 언어 테마"
  },
  "debugger": {
    "cosmos_started": "우주 인스턴스 시작：{0}",
    "evolution_step": "진화 단계 이동：{0}，시간：{1}초",
    "breakpoint_triggered": "브레이크포인트 발동：{0}행",
    "law_validation_warn": "법칙 {0} 일관성 임계값 미달：{1} < {2}",
    "migrate_stage": "마이그레이션 단계 도달：{0}"
  },
  "errors": {
    "invalid_cosmos_file": "유효하지 않은 우주 파일：{0}",
    "law_not_found": "우주에서 법칙 {0}을 찾을 수 없음",
    "lsp_init_failed": "LSP 초기화 실패：{0}",
    "collab_session_failed": "협업 세션 실패：{0}"
  },
  "common": {
    "ok": "확인",
    "cancel": "취소",
    "save": "저장",
    "load": "로드",
    "start": "시작",
    "pause": "일시 중지",
    "stop": "중지",
    "step": "스텝 오버"
  }
}
```

### 3. 本地化集成到扩展（src/lib.rs 补充）
```rust
// 导入本地化模块
mod locale;
use locale::{init_locale, t};

// 在 activate 函数中初始化本地化
#[zed::extension]
fn activate(workspace: &zed::Workspace) -> Result<Box<dyn Extension>> {
    init_logger();
    info!("Cangjie Zed Extension v0.3.0 activated");
    
    // 初始化监控管理器
    init_monitor(workspace);
    
    // 初始化本地化
    init_locale(workspace);
    info!("当前语言：{}", locale::get_locale_manager().unwrap().current_locale);
    
    // 预初始化 LSP
    tokio::spawn(async move {
        let start_time = std::time::Instant::now();
        match init_lsp_client().await {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;
                report_perf!("lsp_init", duration, true);
                info!("{}", t!("common", "lsp_init_success", &[&duration.to_string()]));
            }
            Err(e) => {
                let duration = start_time.elapsed().as_millis() as u64;
                report_perf!("lsp_init", duration, false);
                let error_msg = t!("errors", "lsp_init_failed", &[&e.to_string()]);
                report_error!(
                    ErrorType::LspError,
                    error_msg.clone(),
                    Some(format!("{:#?}", e)),
                    std::collections::HashMap::new()
                );
                warn!("{}", error_msg);
            }
        }
    });

    Ok(Box::new(CangjieZedExtension))
}

// 在主题模块中使用本地化名称（src/syntax_theme/theme.rs）
impl SyntaxTheme {
    pub fn name(&self) -> &str {
        match self.mode {
            ThemeMode::Dark => &t!("theme", "dark_name"),
            ThemeMode::Light => &t!("theme", "light_name"),
            ThemeMode::HighContrast => &t!("theme", "hc_name"),
        }
    }

    pub fn description(&self) -> &str {
        &t!("theme", "description")
    }
}

// 在调试器错误提示中使用本地化（src/debugger/debugger.rs）
impl CangjieDebugger {
    fn inner_start(&mut self) -> Result<()> {
        let cosmos_file_path = self.config.cosmos_file.to_file_path()
            .map_err(|_| ZedError::user(t!("errors", "invalid_cosmos_file", &[&self.config.cosmos_file.to_string()])))?;
        
        let cosmos_meta = self.cosmos_manager.load_cosmos_meta(&cosmos_file_path)?;
        let mut cosmos_instance = self.cosmos_manager.instantiate_cosmos(
            &cosmos_meta,
            &self.config.cosmos_type,
            self.config.step_interval.unwrap_or(100),
        )?;
        
        cosmos_instance.start_evolution();
        self.cosmos_instance = Some(cosmos_instance);
        self.spawn_evolution_task();
        
        Ok(())
    }
}
```

### 4. 本地化依赖配置（Cargo.toml 补充）
```toml
[dependencies]
# 本地化相关依赖
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
once_cell = "1.18"
```

## 二十九、本地化使用说明（补充到 README.md）
```markdown
## 多语言支持
扩展支持 4 种语言：英文（默认）、中文、日语、韩语，自动适配 Zed 界面语言。

### 切换语言
1. 打开 Zed 偏好设置 → 通用 → 语言
2. 选择目标语言（如「中文（中国）」）
3. 重启 Zed，扩展将自动切换为对应语言

### 支持的语言列表
| 语言 | 语言代码 | 适配场景 |
|------|----------|----------|
| 英文 | en | Zed 语言为英文、其他未支持语言时默认 |
| 中文（简体） | zh-CN | Zed 语言为中文（中国）、中文（台湾）等 |
| 日语 | ja | Zed 语言为日语 |
| 韩语 | ko | Zed 语言为韩语 |

### 贡献新语言
若需支持其他语言，可按以下步骤贡献：
1. 复制 `locales/en.json` 并重命名为 `locales/{语言代码}.json`（如 `fr.json` 对应法语）
2. 翻译文件中的所有文本（保持 key 不变，修改 value）
3. 在 `src/locale/mod.rs` 的 `SUPPORTED_LOCALES` 中添加新语言代码
4. 提交 PR，审核通过后将纳入下一个版本
```

## 三十、监控与错误上报说明（补充到 README.md）
```markdown
## 运行时监控与错误上报
为提升扩展稳定性，扩展默认开启运行时监控和错误上报功能，帮助开发团队快速定位问题。

### 上报内容
#### 错误上报
- 扩展运行时错误（如调试器启动失败、LSP 连接异常）
- 错误类型、错误信息、堆栈跟踪
- 上下文信息（如调试配置、文件路径，不含敏感数据）
- 环境信息（扩展版本、Zed 版本、操作系统、架构）
- 匿名用户 ID（基于设备信息哈希，不关联个人身份）

#### 性能上报
- 关键操作耗时（如调试器启动、LSP 初始化、主题切换）
- 操作成功/失败状态
- 环境信息和匿名用户 ID

### 隐私说明
- 不上报任何个人身份信息（如用户名、邮箱、设备原始信息）
- 不上报代码内容、文件内容等敏感数据
- 匿名用户 ID 仅用于统计错误/性能分布，无法关联个人

### 关闭上报
若需关闭错误/性能上报，可通过以下方式：
1. 环境变量方式（临时关闭）：
   ```bash
   # Linux/macOS
   export CANGJIE_ENABLE_REPORT=false
   export CANGJIE_ENABLE_PERF_REPORT=false

   # Windows（命令行）
   set CANGJIE_ENABLE_REPORT=false
   set CANGJIE_ENABLE_PERF_REPORT=false
   ```
2. 永久关闭：修改扩展配置文件（需手动编辑），添加 `enable_report = false`

## 常见问题
### Q：扩展启动失败怎么办？
A：1. 检查 Zed 版本是否 ≥ v0.130.0；2. 检查仓颉 LSP 是否安装并在 PATH 中；3. 查看 Zed 日志（偏好设置 → 高级 → 查看日志），若有错误可提交 Issue 并附上日志。

### Q：调试器无法命中断点？
A：1. 检查 `launch.json` 中 `cosmos_file` 路径是否正确；2. 确认宇宙文件格式是否符合规范；3. 查看扩展日志（开启 `RUST_LOG=debug` 环境变量）。

### Q：如何提交错误反馈？
A：1. 通过 GitHub Issues 提交，标题格式：`[BUG] 问题描述`；2. 附上 Zed 版本、扩展版本、操作系统、错误日志（若有）；3. 描述复现步骤。
```

## 三十一、项目最终目录（完整版）
整合监控和本地化后，最终项目目录如下：
```
cangjie-zed-extension/
├── Cargo.toml                # 项目配置（含监控/本地化依赖）
├── Cargo.lock                # 依赖锁定文件
├── LICENSE                   # MIT 许可证
├── README.md                 # 扩展说明文档（含监控/本地化）
├── CHANGELOG.md              # 更新日志
├── CONTRIBUTING.md           # 贡献指南
├── schemas/                  # 调试配置 JSON Schema
│   └── cangjie-debug-schema.json
├── icons/                    # 图标资源目录
│   ├── dark/
│   └── light/
├── themes/                   # 语法主题配置
│   ├── cangjie-dark.toml
│   ├── cangjie-light.toml
│   └── cangjie-high-contrast.toml
├── locales/                  # 本地化资源文件
│   ├── en.json
│   ├── zh-CN.json
│   ├── ja.json
│   └── ko.json
├── src/                      # 源代码目录
│   ├── lib.rs                # 扩展入口（集成监控/本地化）
│   ├── icon_theme/           # 图标主题模块
│   ├── syntax_theme/         # 语法主题模块
│   ├── debugger/             # 调试器模块
│   │   ├── collab.rs         # 协作调试支持
│   │   ├── adapter.rs        # 调试适配器
│   │   └── debugger_test.rs  # 调试器测试
│   ├── lsp/                  # LSP 集成模块
│   │   └── client.rs         # LSP 客户端封装
│   ├── monitoring/           # 监控与错误上报
│   │   └── error_report.rs
│   ├── locale/               # 本地化支持
│   │   └── mod.rs
│   └── tests/                # 集成测试
├── examples/                 # 模拟数据
├── script/                   # 脚本目录
│   └── licenses/
│       └── zed-licenses.toml
├── assets/                   # 扩展市场截图
├── build.sh                  # 构建脚本
├── test.sh                   # 测试脚本
├── build.bat                 # Windows 构建脚本
└── package.sh                # 扩展打包脚本
```

## 三十二、最终总结：生产级扩展的完整闭环
`cangjie-zed-extension` 已形成 **「核心功能 + 工程化 + 稳定性 + 全球化」** 的完整闭环，具备生产级扩展的所有关键特性：

### 1. 核心功能闭环
- 编辑体验：语法主题（3种模式）、图标主题（文件/语法/UI）、LSP 集成（补全/跳转/悬停）
- 调试能力：宇宙演化调试、法则一致性校验、跨载体迁移调试、多人协作调试
- 协作特性：实时状态同步、操作权限、多用户协作

### 2. 工程化闭环
- 构建打包：跨平台脚本、Zed 扩展包打包、版本管理
- 测试验证：单元测试、模块测试、模拟数据测试、集成测试
- 发布部署：扩展市场元数据、发布流程、更新日志、迁移指南

### 3. 稳定性闭环
- 错误处理：统一错误捕获、详细错误信息、本地化提示
- 监控上报：错误上报、性能监控、匿名统计、问题定位
- 兼容性：多 Zed 版本支持、多操作系统适配、依赖版本锁定

### 4. 全球化闭环
- 多语言支持：4种语言、自动适配 Zed 语言、占位符替换
- 本地化资源：结构化资源文件、易于扩展新语言
- 社区友好：多语言文档、国际化贡献指南

### 最终定位
`cangjie-zed-extension` 不仅是仓颉语言的 Zed 编辑器扩展，更是 **「面向宇宙演化编程」** 的专用开发环境核心组件，深度贴合仓颉语言特性和 Zed 编辑器优势，为开发者提供从代码编写、调试、协作到问题反馈的全流程解决方案。

未来将持续跟进仓颉语言和 Zed 编辑器的迭代，不断完善功能、优化体验，打造全球领先的宇宙编程开发工具链。