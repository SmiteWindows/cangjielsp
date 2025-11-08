//! 性能优化语料库（存储常见性能问题与优化方案，供 RAG 检索）
use std::collections::HashMap;

/// 性能问题类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PerformanceIssueType {
    CpuBound,          // CPU 密集型
    MemoryLeak,        // 内存泄漏
    CoroutineOverhead, // 协程切换开销
    LockContention,    // 锁竞争
    IoBlocking,        // IO 阻塞
    GcOverhead,        // GC 开销过大
    DuplicateCompute,  // 重复计算
}

/// 优化方案
#[derive(Debug, Clone)]
pub struct OptimizationSolution {
    pub issue_id: String,           // 问题 ID
    pub title: String,              // 标题
    pub description: String,        // 问题描述
    pub solution: String,           // 优化方案
    pub code_example: String,       // 代码示例（优化前后）
    pub related_rules: Vec<String>, // 关联规则（如 cjlint 规则）
    pub documentation: String,      // 参考文档链接
}

/// 性能优化语料库管理器
#[derive(Debug, Default)]
pub struct PerformanceCorpus {
    corpus: HashMap<PerformanceIssueType, Vec<OptimizationSolution>>,
}

impl PerformanceCorpus {
    /// 创建新的语料库
    pub fn new() -> Self {
        let mut corpus = HashMap::new();

        // 1. CPU 密集型问题
        corpus.insert(
            PerformanceIssueType::CpuBound,
            vec![
                OptimizationSolution {
                    issue_id: "CPU-001".to_string(),
                    title: "循环内重复计算".to_string(),
                    description: "循环体内执行重复的计算逻辑，导致 CPU 使用率过高",
                    solution: "将循环外可预计算的逻辑提取到循环外部，避免重复执行",
                    code_example: "// 优化前\nfor i in 0..1000 {\n  let x = expensive_calculation(); // 重复计算\n  println!(\"{}\", x + i);\n}\n\n// 优化后\nlet x = expensive_calculation(); // 只计算一次\nfor i in 0..1000 {\n  println!(\"{}\", x + i);\n}",
                    related_rules: vec!["I004".to_string(), "PERF-001".to_string()],
                    documentation: "https://docs.cangjie-lang.cn/docs/1.0.3/performance/loop_optimization.html",
                },
                OptimizationSolution {
                    issue_id: "CPU-002".to_string(),
                    title: "低效迭代器使用".to_string(),
                    description: "使用嵌套循环或低效迭代器方法，时间复杂度较高",
                    solution: "使用仓颉标准库提供的高效迭代器方法（如 map、filter、fold），减少嵌套",
                    code_example: "// 优化前\nlet mut result = Vec::new();\nfor item in &items {\n  if item > 10 {\n    result.push(item * 2);\n  }\n}\n\n// 优化后\nlet result = items.iter()\n  .filter(|&&x| x > 10)\n  .map(|&x| x * 2)\n  .collect::<Vec<_>>();",
                    related_rules: vec!["I002".to_string(), "PERF-003".to_string()],
                    documentation: "https://docs.cangjie-lang.cn/docs/1.0.3/performance/iterator_optimization.html",
                },
            ],
        );

        // 2. 协程切换开销问题
        corpus.insert(
            PerformanceIssueType::CoroutineOverhead,
            vec![
                OptimizationSolution {
                    issue_id: "CORO-001".to_string(),
                    title: "过多协程创建".to_string(),
                    description: "无限制创建协程，导致协程切换开销过大",
                    solution: "使用协程池管理协程数量，避免创建过多协程",
                    code_example: "// 优化前\nfor _ in 0..10000 {\n  spawn(async { // 创建 10000 个协程\n    do_work().await;\n  });\n}\n\n// 优化后\nlet pool = CoroutinePool::new(100); // 限制 100 个协程\nfor _ in 0..10000 {\n  pool.spawn(async { // 复用协程\n    do_work().await;\n  });\n}",
                    related_rules: vec!["PERF-005".to_string(), "CJPROF-CORO-001".to_string()],
                    documentation: "https://docs.cangjie-lang.cn/docs/1.0.3/concurrency/coroutine_pool.html",
                },
            ],
        );

        // 其他问题类型的语料（省略，格式同上）
        corpus
    }

    /// 根据问题类型检索优化方案
    pub fn search_by_issue_type(
        &self,
        issue_type: &PerformanceIssueType,
    ) -> Vec<OptimizationSolution> {
        self.corpus.get(issue_type).cloned().unwrap_or_default()
    }

    /// 根据关键词检索优化方案
    pub fn search_by_keyword(&self, keyword: &str) -> Vec<OptimizationSolution> {
        let keyword = keyword.to_lowercase();
        self.corpus
            .values()
            .flatten()
            .filter(|sol| {
                sol.title.to_lowercase().contains(&keyword)
                    || sol.description.to_lowercase().contains(&keyword)
                    || sol.solution.to_lowercase().contains(&keyword)
            })
            .cloned()
            .collect()
    }
}

/// 全局语料库实例
pub fn global_corpus() -> &'static PerformanceCorpus {
    static CORPUS: std::sync::OnceLock<PerformanceCorpus> = std::sync::OnceLock::new();
    CORPUS.get_or_init(PerformanceCorpus::new)
}
