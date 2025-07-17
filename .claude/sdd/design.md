# LineLint 測試覆蓋率改善架構設計

## 1. 概述

本文件為 LineLint 專案測試覆蓋率改善的架構設計方案，基於需求文件制定，目標是將測試覆蓋率從 56% 提升至 100%。

### 設計原則
- **可測試性優先**：所有設計決策都以提高可測試性為首要考量
- **最小侵入性**：保持 API 向後相容，避免大規模重構
- **漸進式改進**：分階段實施，每步都可驗證
- **關注點分離**：將 I/O 操作與業務邏輯分離

## 2. 系統架構

### 2.1 分層架構

```
┌─────────────────────────────────────────┐
│           測試層 (Tests)                │
├─────────────────────────────────────────┤
│      測試輔助層 (Test Helpers)          │
│  - Mocks   - Fixtures   - Builders      │
├─────────────────────────────────────────┤
│         應用層 (Application)            │
│  - CLI     - Config    - Reporter       │
├─────────────────────────────────────────┤
│         核心層 (Core)                   │
│  - Checker - Discovery - Fixer          │
├─────────────────────────────────────────┤
│         基礎設施層 (Infrastructure)     │
│  - FileSystem - Git - Output            │
└─────────────────────────────────────────┘
```

### 2.2 依賴注入架構

為提高可測試性，引入依賴注入模式：

```rust
// 原始設計（難以測試）
pub fn check_file(path: &Path, config: &Config) -> Result<Vec<Issue>> {
    let content = fs::read_to_string(path)?;
    // 直接依賴檔案系統
}

// 新設計（易於測試）
pub fn check_file<FS: FileSystem>(
    path: &Path,
    config: &Config,
    fs: &FS
) -> Result<Vec<Issue>> {
    let content = fs.read_to_string(path)?;
    // 透過抽象介面操作
}
```

## 3. 核心模組設計

### 3.1 Checker 模組重構

#### 3.1.1 抽象介面定義

```rust
// src/checker/traits.rs
pub trait FileReader {
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
    fn open(&self, path: &Path) -> io::Result<Box<dyn Read>>;
    fn metadata(&self, path: &Path) -> io::Result<Metadata>;
}

pub trait LineChecker {
    fn check_line(&self, line: &str, line_number: usize) -> Vec<Issue>;
    fn check_final_newline(&self, content: &str) -> Option<Issue>;
}
```

#### 3.1.2 核心邏輯分離

```rust
// src/checker/core.rs
pub struct CheckerCore {
    config: Config,
}

impl CheckerCore {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // 純函數，易於測試
    pub fn check_newline_ending(&self, content: &str) -> Option<Issue> {
        if content.is_empty() || content.ends_with('\n') {
            None
        } else {
            Some(Issue::MissingNewline)
        }
    }

    pub fn check_trailing_spaces(&self, line: &str, line_num: usize) -> Option<Issue> {
        if line.ends_with(' ') || line.ends_with('\t') {
            Some(Issue::TrailingSpace { line: line_num })
        } else {
            None
        }
    }
}
```

#### 3.1.3 檔案檢查器實作

```rust
// src/checker/file_checker.rs
pub struct FileChecker<FR: FileReader> {
    core: CheckerCore,
    file_reader: FR,
}

impl<FR: FileReader> FileChecker<FR> {
    pub fn check_file(&self, path: &Path) -> Result<Vec<Issue>> {
        let metadata = self.file_reader.metadata(path)?;

        if metadata.len() < MEMORY_THRESHOLD {
            self.check_file_in_memory(path)
        } else {
            self.check_file_streaming(path)
        }
    }
}
```

### 3.2 Reporter 模組重構

#### 3.2.1 輸出抽象

```rust
// src/reporter/traits.rs
pub trait Output: Send + Sync {
    fn write(&mut self, content: &str) -> io::Result<()>;
    fn write_line(&mut self, content: &str) -> io::Result<()>;
    fn flush(&mut self) -> io::Result<()>;
}

pub trait ColoredOutput: Output {
    fn write_colored(&mut self, content: &str, color: Color) -> io::Result<()>;
}
```

#### 3.2.2 報告器重構

```rust
// src/reporter/base.rs
pub trait Reporter {
    type Output: Output;

    fn report_results(
        &self,
        results: &[CheckResult],
        output: &mut Self::Output,
    ) -> Result<()>;
}

// src/reporter/human.rs
pub struct HumanReporter {
    show_progress: bool,
    verbose: bool,
}

impl Reporter for HumanReporter {
    type Output = dyn ColoredOutput;

    fn report_results(
        &self,
        results: &[CheckResult],
        output: &mut Self::Output,
    ) -> Result<()> {
        // 實作人類可讀格式輸出
    }
}
```

## 4. 測試基礎設施

### 4.1 Mock 實作

#### 4.1.1 檔案系統 Mock

```rust
// src/testing/mocks/filesystem.rs
pub struct MockFileSystem {
    files: HashMap<PathBuf, String>,
    metadata: HashMap<PathBuf, MockMetadata>,
    errors: HashMap<PathBuf, io::Error>,
}

impl MockFileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P, content: String) {
        self.files.insert(path.into(), content);
    }

    pub fn add_error<P: Into<PathBuf>>(&mut self, path: P, error: io::Error) {
        self.errors.insert(path.into(), error);
    }
}

impl FileReader for MockFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        if let Some(error) = self.errors.get(path) {
            return Err(io::Error::new(error.kind(), error.to_string()));
        }

        self.files.get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}
```

#### 4.1.2 輸出 Mock

```rust
// src/testing/mocks/output.rs
pub struct MockOutput {
    pub buffer: Vec<String>,
    pub color_calls: Vec<(String, Color)>,
}

impl Output for MockOutput {
    fn write(&mut self, content: &str) -> io::Result<()> {
        self.buffer.push(content.to_string());
        Ok(())
    }
}

impl ColoredOutput for MockOutput {
    fn write_colored(&mut self, content: &str, color: Color) -> io::Result<()> {
        self.color_calls.push((content.to_string(), color));
        self.write(content)
    }
}
```

### 4.2 測試建構器

#### 4.2.1 測試檔案建構器

```rust
// src/testing/builders/file_builder.rs
pub struct TestFileBuilder {
    content: String,
    path: PathBuf,
}

impl TestFileBuilder {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            content: String::new(),
            path: path.into(),
        }
    }

    pub fn with_line(mut self, line: &str) -> Self {
        self.content.push_str(line);
        self.content.push('\n');
        self
    }

    pub fn with_trailing_spaces(mut self) -> Self {
        self.content.push_str("line with spaces   ");
        self.content.push('\n');
        self
    }

    pub fn without_final_newline(mut self) -> Self {
        self.content.pop(); // 移除最後的換行
        self
    }

    pub fn build(self) -> (PathBuf, String) {
        (self.path, self.content)
    }
}
```

#### 4.2.2 測試場景建構器

```rust
// src/testing/builders/scenario_builder.rs
pub struct TestScenario {
    files: Vec<(PathBuf, String)>,
    config: Config,
    expected_issues: Vec<Issue>,
}

impl TestScenario {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_file(mut self, builder: TestFileBuilder) -> Self {
        let (path, content) = builder.build();
        self.files.push((path, content));
        self
    }

    pub fn expecting_issue(mut self, issue: Issue) -> Self {
        self.expected_issues.push(issue);
        self
    }

    pub fn run<C: Checker>(self, checker: &C) -> TestResult {
        // 執行測試並驗證結果
    }
}
```

### 4.3 測試固件

```rust
// src/testing/fixtures.rs
pub mod fixtures {
    pub const SIMPLE_FILE: &str = "line 1\nline 2\nline 3\n";
    pub const FILE_WITH_TRAILING_SPACES: &str = "line 1   \nline 2\n";
    pub const FILE_WITHOUT_NEWLINE: &str = "line 1\nline 2";
    pub const EMPTY_FILE: &str = "";
    pub const LARGE_FILE_GENERATOR: fn(usize) -> String = |lines| {
        (0..lines).map(|i| format!("line {}\n", i)).collect()
    };
}
```

## 5. 測試策略

### 5.1 單元測試組織

```
src/
├── checker/
│   ├── mod.rs
│   └── tests.rs          # 單元測試
├── reporter/
│   ├── mod.rs
│   └── tests.rs          # 單元測試
└── testing/              # 測試輔助模組
    ├── mocks/
    ├── builders/
    └── fixtures.rs
```

### 5.2 測試分類

#### 單元測試特徵
- 執行時間 < 100ms
- 無外部依賴
- 使用 Mock 隔離 I/O
- 專注於單一功能

#### 整合測試特徵
- 測試多模組協作
- 可能使用真實檔案系統
- 測試端到端流程
- 標記為 `#[ignore]` 的慢測試

### 5.3 測試覆蓋策略

#### Checker 模組測試計畫

1. **正常路徑測試**
   - 正確格式的檔案
   - 各種換行符號（LF、CRLF）
   - 不同檔案大小

2. **錯誤路徑測試**
   - 檔案不存在
   - 權限不足
   - 編碼錯誤
   - I/O 錯誤

3. **邊界條件測試**
   - 空檔案
   - 單行檔案
   - 超大檔案（串流處理）
   - 特殊字元

#### Reporter 模組測試計畫

1. **格式測試**
   - Human 格式輸出
   - JSON 格式輸出
   - GitHub Actions 格式

2. **錯誤處理測試**
   - 輸出錯誤
   - 序列化錯誤

3. **互動測試**
   - 進度條顯示
   - 彩色輸出
   - 靜默模式

## 6. 實作計畫

### 第一階段：基礎設施建立（2天）
1. 建立測試輔助模組結構
2. 實作基本 Mock（FileSystem、Output）
3. 建立測試建構器框架

### 第二階段：Checker 模組改造（3天）
1. 抽取介面定義
2. 重構核心邏輯為純函數
3. 編寫完整單元測試
4. 達成 90%+ 覆蓋率

### 第三階段：Reporter 模組改造（2天）
1. 抽取輸出介面
2. 重構報告器實作
3. 編寫格式測試
4. 達成 90%+ 覆蓋率

### 第四階段：整合與優化（2天）
1. 整理測試組織結構
2. 優化測試執行時間
3. 更新 CI 配置
4. 文件更新

## 7. 技術決策

### 7.1 為何使用依賴注入
- **優點**：提高可測試性、降低耦合
- **缺點**：增加程式碼複雜度
- **決策**：採用簡單的泛型參數方式，避免過度設計

### 7.2 為何分離純函數
- **優點**：易於測試、可預測行為
- **缺點**：可能需要更多參數傳遞
- **決策**：核心邏輯全部提取為純函數

### 7.3 Mock 設計選擇
- **選項 1**：使用 mockall 等第三方框架
- **選項 2**：手動實作簡單 Mock
- **決策**：手動實作，保持簡單且易於理解

## 8. 風險評估與緩解

### 8.1 技術風險

| 風險 | 影響 | 機率 | 緩解措施 |
|------|------|------|----------|
| 重構引入新 bug | 高 | 中 | 小步重構、保持測試通過 |
| 效能退化 | 中 | 低 | 基準測試、效能監控 |
| API 不相容 | 高 | 低 | 使用特性標記、漸進式遷移 |

### 8.2 時程風險

| 風險 | 影響 | 機率 | 緩解措施 |
|------|------|------|----------|
| 低估工作量 | 中 | 中 | 優先核心模組、分階段交付 |
| 測試編寫耗時 | 低 | 高 | 使用測試建構器、重用測試案例 |

### 8.3 品質風險

| 風險 | 影響 | 機率 | 緩解措施 |
|------|------|------|----------|
| 測試品質不足 | 高 | 低 | Code review、測試覆蓋率門檻 |
| 過度測試 | 低 | 中 | 專注於關鍵路徑、避免重複測試 |

## 9. 非功能性需求

### 9.1 效能要求
- 單元測試總執行時間 < 5 秒
- 單一測試執行時間 < 100ms
- Mock 操作零開銷

### 9.2 可維護性
- 測試程式碼遵循 DRY 原則
- 清晰的測試命名規範
- 完整的測試文件

### 9.3 可擴展性
- Mock 系統易於擴展新功能
- 測試建構器支援組合
- 測試場景可重用

## 10. 驗收標準檢查清單

### Checker 模組
- [ ] 覆蓋率達到 90%+
- [ ] 所有錯誤路徑都有測試
- [ ] 邊界條件完整覆蓋
- [ ] 測試執行時間 < 1 秒

### Reporter 模組
- [ ] 覆蓋率達到 90%+
- [ ] 三種格式都有測試
- [ ] Mock 輸出驗證正確
- [ ] 無實際終端輸出

### 測試基礎設施
- [ ] Mock 實作完整
- [ ] 測試建構器易用
- [ ] 文件齊全
- [ ] 可重用性高

### 整體目標
- [ ] 總覆蓋率達到 100%
- [ ] cargo test --lib < 5 秒
- [ ] CI 分階段執行
- [ ] 無測試隨機失敗

## 11. 結論

本架構設計方案通過引入依賴注入、分離純函數、建立完善的測試基礎設施，能夠有效提升 LineLint 專案的測試覆蓋率。設計充分考慮了可測試性、可維護性和實施風險，預計能在 3 週內完成全部實施工作，達成 100% 測試覆蓋率的目標。
