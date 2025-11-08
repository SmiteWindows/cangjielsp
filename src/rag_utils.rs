//! RAG 辅助工具（检索增强生成，用于性能问题诊断与优化建议）
use super::corpus::{OptimizationSolution, PerformanceCorpus, PerformanceIssueType, global_corpus};
use std::collections::HashSet;
use zed_extension_api as zed;

/// RAG 检索器
#[derive(Debug, Default)]
pub struct RagRetriever {
    corpus: &'static PerformanceCorpus,
}

impl RagRetriever {
    /// 创建新的检索器
    pub fn new() -> Self {
        Self {
            corpus: global_corpus(),
        }
    }

    /// 基于性能分析结果检索优化建议
    pub fn retrieve_from_profiling(
        &self,
        bottlenecks: &[super::cjprof::Bottleneck],
    ) -> Vec<OptimizationSolution> {
        let mut solutions = Vec::new();
        let mut added_ids = HashSet::new();

        for bottleneck in bottlenecks {
            // 映射 cjprof 瓶颈类型到语料库问题类型
            let issue_type = match bottleneck.type_ {
                super::cjprof::BottleneckType::CpuBound => PerformanceIssueType::CpuBound,
                super::cjprof::BottleneckType::MemoryLeak => PerformanceIssueType::MemoryLeak,
                super::cjprof::BottleneckType::CoroutineOverhead => {
                    PerformanceIssueType::CoroutineOverhead
                }
                super::cjprof::BottleneckType::LockContention => {
                    PerformanceIssueType::LockContention
                }
                super::cjprof::BottleneckType::IoBlocking => PerformanceIssueType::IoBlocking,
                super::cjprof::BottleneckType::GcOverhead => PerformanceIssueType::GcOverhead,
                super::cjprof::BottleneckType::DuplicateCompute => {
                    PerformanceIssueType::DuplicateCompute
                }
            };

            // 检索该类型的优化方案
            let type_solutions = self.corpus.search_by_issue_type(&issue_type);
            for sol in type_solutions {
                if !added_ids.contains(&sol.issue_id) {
                    added_ids.insert(sol.issue_id.clone());
                    solutions.push(sol);
                }
            }

            // 基于瓶颈描述关键词补充检索
            let keywords = extract_keywords(&bottleneck.description);
            for keyword in keywords {
                let keyword_solutions = self.corpus.search_by_keyword(&keyword);
                for sol in keyword_solutions {
                    if !added_ids.contains(&sol.issue_id) {
                        added_ids.insert(sol.issue_id.clone());
                        solutions.push(sol);
                    }
                }
            }
        }

        solutions
    }

    /// 生成优化建议报告
    pub fn generate_optimization_report(&self, solutions: &[OptimizationSolution]) -> String {
        let mut report = String::from("# 仓颉性能优化建议报告\n\n");

        for (i, sol) in solutions.iter().enumerate() {
            report.push_str(&format!("## {}. {}\n\n", i + 1, sol.title));
            report.push_str(&format!("### 问题描述\n{}\n\n", sol.description));
            report.push_str(&format!("### 优化方案\n{}\n\n", sol.solution));
            report.push_str(&format!(
                "### 代码示例\n```cangjie\n{}\n```\n\n",
                sol.code_example
            ));
            report.push_str(&format!("### 参考文档\n{}\n\n", sol.documentation));
            report.push_str("---\n\n");
        }

        if solutions.is_empty() {
            report.push_str("未找到匹配的优化建议，请检查性能瓶颈描述或扩展语料库。");
        }

        report
    }
}

/// 从文本中提取关键词（简单分词）
fn extract_keywords(text: &str) -> Vec<String> {
    let stop_words = HashSet::from([
        "的", "了", "是", "在", "有", "和", "就", "不", "人", "都", "一", "个", "上", "下", "来",
        "去", "大", "小", "多", "少", "中", "里", "外", "到", "着", "过", "要", "会", "能", "可",
        "以", "把", "被", "让", "使", "对", "于", "关于", "因为", "所以", "如果", "虽然", "但是",
        "不仅", "而且", "cpu", "内存", "协程", "锁", "io", "gc", "开销", "耗时", "占用", "切换",
        "泄漏", "阻塞", "竞争",
    ]);

    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|word| !word.is_empty() && !stop_words.contains(word) && word.len() >= 2)
        .map(|word| word.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_extraction() {
        let text = "协程切换开销过大，导致 CPU 使用率飙升";
        let keywords = extract_keywords(text);
        assert!(keywords.contains(&"切换".to_string()));
        assert!(keywords.contains(&"飙升".to_string()));
        assert!(!keywords.contains(&"协程".to_string())); // stop word
        assert!(!keywords.contains(&"cpu".to_string())); // stop word
    }

    #[test]
    fn test_rag_retrieval() {
        let retriever = RagRetriever::new();
        let bottlenecks = vec![super::cjprof::Bottleneck {
            type_: super::cjprof::BottleneckType::CoroutineOverhead,
            description: "协程切换开销过大，创建了过多协程".to_string(),
            affected_functions: vec!["main".to_string()],
            severity: super::cjprof::SeverityLevel::High,
        }];

        let solutions = retriever.retrieve_from_profiling(&bottlenecks);
        assert!(!solutions.is_empty());
        assert_eq!(solutions[0].issue_id, "CORO-001");
    }
}
