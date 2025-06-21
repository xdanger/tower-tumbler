# 🤝 贡献指南

感谢您对 Tower Tumbler 项目的关注和贡献意向！本指南将帮助您了解如何有效地参与项目开发。

## 📋 目录

- [快速开始](#快速开始)
- [开发环境设置](#开发环境设置)
- [贡献流程](#贡献流程)
- [代码规范](#代码规范)
- [提交规范](#提交规范)
- [测试要求](#测试要求)
- [文档标准](#文档标准)
- [问题报告](#问题报告)
- [功能请求](#功能请求)
- [代码审查](#代码审查)

## 🚀 快速开始

1. **Fork 项目**

   ```bash
   # 在 GitHub 上点击 Fork 按钮
   # 然后克隆您的 fork
   git clone https://github.com/YOUR_USERNAME/tower-tumbler.git
   cd tower-tumbler
   ```

2. **设置上游远程仓库**

   ```bash
   git remote add upstream https://github.com/xdanger/tower-tumbler.git
   git fetch upstream
   ```

3. **创建功能分支**

   ```bash
   git checkout -b feat/amazing-feature
   ```

4. **进行开发**

   ```bash
   # 安装依赖和工具
   make setup

   # 启动开发服务器
   trunk serve
   ```

5. **提交和推送**

   ```bash
   git add .
   git commit -m "feat: 添加精彩功能"
   git push origin feat/amazing-feature
   ```

6. **创建 Pull Request**
   - 在 GitHub 上点击 "New Pull Request"
   - 填写 PR 模板中的信息
   - 等待代码审查

## 🛠️ 开发环境设置

### 系统要求

- **Rust**: 1.75+ (推荐使用 rustup)
- **Node.js**: 18+
- **Git**: 最新版本

### 必需工具

```bash
# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# 安装构建工具
cargo install trunk wasm-bindgen-cli wasm-pack

# 安装开发工具
cargo install cargo-watch cargo-audit cargo-deny
```

### 可选工具

```bash
# 代码质量工具
cargo install cargo-clippy rustfmt

# 性能分析工具
cargo install cargo-flamegraph

# WASM 优化工具
npm install -g wasm-opt
```

### IDE 配置

#### VS Code (推荐)

安装以下扩展：

- rust-analyzer
- CodeLLDB
- Better TOML
- Even Better TOML

#### 其他编辑器

- **Vim/Neovim**: 配置 rust-analyzer LSP
- **Emacs**: 使用 rustic-mode
- **IntelliJ**: Rust 插件

## 🔄 贡献流程

### 1. 选择任务

- 查看 [Issues](https://github.com/xdanger/tower-tumbler/issues)
- 寻找标有 `good first issue` 的任务
- 在 Issue 中评论表示您想要处理该任务

### 2. 开发工作

```bash
# 确保分支是最新的
git checkout main
git pull upstream main

# 创建新分支
git checkout -b type/short-description

# 例如：
git checkout -b feat/tilt-sensitivity-settings
git checkout -b fix/wasm-memory-leak
git checkout -b docs/api-documentation
```

### 3. 提交更改

- 遵循[提交规范](#提交规范)
- 确保每个提交都是逻辑完整的
- 包含必要的测试和文档

### 4. 推送和 PR

```bash
# 推送到您的 fork
git push origin your-branch-name

# 在 GitHub 上创建 Pull Request
# 填写 PR 模板
# 链接相关的 Issue
```

## 📏 代码规范

### Rust 代码风格

我们使用标准的 Rust 代码风格，通过以下工具确保一致性：

```bash
# 格式化代码
cargo fmt

# 代码质量检查
cargo clippy -- -D warnings

# 运行完整检查
make lint
```

### 命名约定

- **类型**: `PascalCase` (例如：`GameState`, `CrateBundle`)
- **函数和变量**: `snake_case` (例如：`update_score`, `player_input`)
- **常量**: `SCREAMING_SNAKE_CASE` (例如：`MAX_STACK_HEIGHT`)
- **模块**: `snake_case` (例如：`input_handler`, `physics_world`)

### 代码组织

```rust
// 文件头部：导入顺序
use std::collections::HashMap;          // 标准库
use bevy::prelude::*;                   // 外部 crate
use crate::core::GameState;             // 本地模块

// 类型定义
#[derive(Component, Debug, Clone)]
pub struct ExampleComponent {
    pub field: f32,
}

// 实现块
impl ExampleComponent {
    pub fn new(field: f32) -> Self {
        Self { field }
    }
}

// 系统函数
pub fn example_system(
    mut query: Query<&mut ExampleComponent>,
    time: Res<Time>,
) {
    // 实现
}
```

### 错误处理

```rust
// ✅ 推荐：使用 Result 进行错误处理
fn safe_operation() -> Result<(), GameError> {
    // 实现
    Ok(())
}

// ✅ 推荐：自定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Physics error: {0}")]
    Physics(String),
    #[error("Input error: {0}")]
    Input(String),
}

// ❌ 避免：忽略错误
let _ = risky_operation(); // 不好的做法
```

## 📝 提交规范

我们使用[语义化提交](https://www.conventionalcommits.org/)格式：

### 格式

```
<类型>[可选的作用域]: <描述>

[可选的正文]

[可选的脚注]
```

### 提交类型

- **feat**: 新功能
- **fix**: 错误修复
- **docs**: 文档更新
- **style**: 代码格式调整（不影响功能）
- **refactor**: 代码重构（不修复错误也不添加功能）
- **perf**: 性能优化
- **test**: 测试相关
- **build**: 构建系统或外部依赖变更
- **ci**: CI 配置文件和脚本变更
- **chore**: 其他不修改源码的更改
- **revert**: 撤销之前的提交

### 示例

```bash
# 功能开发
git commit -m "feat(input): 添加倾斜敏感度设置"

# 错误修复
git commit -m "fix(physics): 修复方块重叠检测问题"

# 文档更新
git commit -m "docs: 更新 API 文档和使用示例"

# 性能优化
git commit -m "perf(wasm): 优化 WASM 包大小减少 15%"

# 重构
git commit -m "refactor(ui): 将 HUD 组件拆分为更小的模块"
```

### 提交正文

对于复杂的更改，请添加详细的提交正文：

```
feat(scoring): 实现完美叠加奖励系统

- 添加精确度计算算法
- 实现分数倍数机制
- 更新 UI 显示奖励信息
- 添加相关测试用例

Closes #123
```

## 🧪 测试要求

### 测试覆盖率

- 新功能必须包含测试
- 目标测试覆盖率：**80%+**
- 关键逻辑必须有测试覆盖

### 测试类型

#### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_calculation() {
        let accuracy = 0.95;
        let base_score = 100;
        let result = calculate_score(base_score, accuracy);
        assert_eq!(result, 150); // 完美叠加奖励
    }
}
```

#### 集成测试

```rust
// tests/integration_test.rs
use tower_tumbler::prelude::*;

#[test]
fn test_game_flow() {
    let mut app = App::new();
    app.add_plugins(TestPlugins);

    // 测试完整游戏流程
}
```

#### WASM 测试

```bash
# 运行 WASM 环境测试
wasm-pack test --headless --firefox
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_score_calculation

# 运行集成测试
cargo test --test integration_test

# 测试覆盖率报告
cargo tarpaulin --out Html
```

## 📚 文档标准

### 代码文档

````rust
/// 计算基于叠加精度的游戏分数
///
/// # 参数
///
/// * `base_score` - 基础分数值
/// * `accuracy` - 叠加精度 (0.0 到 1.0)
///
/// # 返回值
///
/// 计算后的最终分数，包括精度奖励
///
/// # 示例
///
/// ```
/// let score = calculate_score(100, 0.95);
/// assert_eq!(score, 150);
/// ```
///
/// # 错误
///
/// 当 `accuracy` 超出 [0.0, 1.0] 范围时会 panic
pub fn calculate_score(base_score: u32, accuracy: f32) -> u32 {
    // 实现
}
````

### README 更新

- 添加新功能时更新功能列表
- 更新安装和使用说明
- 保持示例代码最新

### API 文档

```bash
# 生成并查看 API 文档
cargo doc --open

# 生成包含私有项的文档
cargo doc --document-private-items
```

## 🐛 问题报告

### 报告错误

使用 [Bug Report 模板](https://github.com/xdanger/tower-tumbler/issues/new?template=bug_report.md)：

1. **环境信息**

   - 操作系统和版本
   - 浏览器和版本
   - Rust 版本

2. **复现步骤**

   - 详细的操作步骤
   - 预期行为
   - 实际行为

3. **附加信息**
   - 错误日志
   - 截图或录屏
   - 相关配置

### 性能问题

对于性能相关问题，请提供：

- 性能分析报告
- 对比数据
- 测试环境信息

## 💡 功能请求

使用 [Feature Request 模板](https://github.com/xdanger/tower-tumbler/issues/new?template=feature_request.md)：

1. **功能描述**：清晰描述建议的功能
2. **使用场景**：说明为什么需要这个功能
3. **实现建议**：如果有技术建议请详细说明
4. **替代方案**：是否考虑过其他解决方案

## 👥 代码审查

### 审查标准

代码审查关注以下方面：

1. **功能正确性**

   - 代码是否按预期工作
   - 边界情况处理
   - 错误处理

2. **代码质量**

   - 代码可读性
   - 遵循项目约定
   - 性能考虑

3. **测试覆盖**

   - 是否有足够测试
   - 测试质量
   - 边界情况测试

4. **文档完整性**
   - 代码注释
   - API 文档
   - 使用示例

### 审查流程

1. **自我审查**：提交前先自己审查代码
2. **自动检查**：确保 CI 检查通过
3. **同行审查**：至少一个项目维护者审查
4. **反馈处理**：及时回应审查意见
5. **合并准备**：所有检查通过后准备合并

### 审查参与

我们鼓励所有贡献者参与代码审查：

- 学习他人代码
- 分享知识和经验
- 提高整体代码质量

## 🌟 最佳实践

### 开发建议

1. **小步快跑**：频繁提交小的、完整的更改
2. **测试驱动**：先写测试，再实现功能
3. **性能意识**：考虑 WASM 环境的限制
4. **文档同步**：代码和文档同步更新

### 沟通建议

1. **清晰表达**：在 Issue 和 PR 中清楚表达想法
2. **及时响应**：合理时间内回应评论和请求
3. **建设性反馈**：提供具体、有帮助的建议
4. **保持尊重**：维护友好的社区氛围

## 🎉 社区参与

### 讨论渠道

- **GitHub Issues**: 错误报告和功能请求
- **GitHub Discussions**: 一般讨论和问答
- **Discord/Slack**: 实时交流（如果有的话）

### 贡献认可

我们会在以下方式认可贡献者：

- README 中的贡献者列表
- Release Notes 中提及
- 特殊贡献的个人感谢

## ❓ 获得帮助

如果您在贡献过程中遇到问题：

1. **查看文档**：阅读项目文档和 FAQ
2. **搜索 Issues**：查看是否有类似问题
3. **提问**：在 GitHub Discussions 中提问
4. **联系维护者**：通过 Issue 或邮件联系

## 📄 许可证

通过向本项目贡献代码，您同意您的贡献将按照 [MIT 许可证](LICENSE) 进行许可。

---

再次感谢您的贡献！每一个贡献都让 Tower Tumbler 变得更好。🚀
