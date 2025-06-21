# 🤖 Tower Tumbler AI 开发助手指南

## 📋 项目文档导航

### Overview

- ✅ [[#1](https://github.com/xdanger/tower-tumbler/issues/1) 产品设计方案
- ✅ [[#2](https://github.com/xdanger/tower-tumbler/issues/2) MVP 开发计划

### 🏗️ 基础设施 (01-04)

- ⌛️ 01 [#4](https://github.com/xdanger/tower-tumbler/issues/4)：初始化 Cargo + Bevy + Trunk 架构
- ⌛️ 02 [#15](https://github.com/xdanger/tower-tumbler/issues/15)：GitHub Actions — wasm build & cache
- ⌛️ 03 [#5](https://github.com/xdanger/tower-tumbler/issues/5)：JS ↔ Rust 通道，权限按钮
- ⌛️ 04 [#6](https://github.com/xdanger/tower-tumbler/issues/6)：映射 β 角度到 Rapier 重力 (输入部分)

### 🎮 核心游戏系统 (05-12)

- ⌛️ 05 [#18](https://github.com/xdanger/tower-tumbler/issues/18)：Playwright — β±10° 可控性测试
- ⌛️ 06 [#10](https://github.com/xdanger/tower-tumbler/issues/10)：生成地面刚体 + 精灵
- ⌛️ 07 [#8](https://github.com/xdanger/tower-tumbler/issues/8)：周期性生成随机大小箱子实体
- ⌛️ 08 [#9](https://github.com/xdanger/tower-tumbler/issues/9)：监听碰撞，静态化，计算偏差
- ⌛️ 09 [#12](https://github.com/xdanger/tower-tumbler/issues/12)：Bevy UI 显示分数
- ⌛️ 10 [#13](https://github.com/xdanger/tower-tumbler/issues/13)：Bevy UI 显示分数 (UI 部分)
- ⌛️ 11 [#11](https://github.com/xdanger/tower-tumbler/issues/11)：倾斜 > 15° 或出界切换到 GameOver
- ⌛️ 12 [#14](https://github.com/xdanger/tower-tumbler/issues/14)：单击重启（重置 ECS）

### ⚡ 性能优化 (13-20)

- ⌛️ 13 [#19](https://github.com/xdanger/tower-tumbler/issues/19)：拆分 core/input/physics 目录
- ⌛️ 14 [#20](https://github.com/xdanger/tower-tumbler/issues/20)：脚本运行 wasm-opt -Oz + brotli
- ⌛️ 15 [#17](https://github.com/xdanger/tower-tumbler/issues/17)：临时箱子和背景 PNG + 加载器
- ⌛️ 16 [#21](https://github.com/xdanger/tower-tumbler/issues/21)：三像素内完美叠加 +2 分
- ⌛️ 17 [#22](https://github.com/xdanger/tower-tumbler/issues/22)：Rust 单元测试误差阈值计算
- ⌛️ 18 [#23](https://github.com/xdanger/tower-tumbler/issues/23)：Rustfmt & Clippy 门控
- ⌛️ 19 [#24](https://github.com/xdanger/tower-tumbler/issues/24)：无权限时左右 UI 按钮控制
- ⌛️ 20 [#7](https://github.com/xdanger/tower-tumbler/issues/7)：映射 β 角度到 Rapier 重力 (映射部分)
- ⌛️ 21 [#16](https://github.com/xdanger/tower-tumbler/issues/16)：Web profiler 控制台统计

## 📝 开发过程

- **linter**: `cargo clippy --all-targets --all-features -- -D warnings`
- **formatter**: `cargo fmt`

## 🎯 开发行为要求

### 主动沟通

- **不清楚的需求**：立即提出具体问题，要求澄清
- **效率考虑**：识别哪些任务人工处理更高效，主动建议
- **技术决策**：涉及架构或技术选型时，提供多个方案对比

### 技术原则

- **优先复用**：使用成熟的第三方库和组件，避免重复造轮子
- **代码质量**：关注冗余代码，及时清理无效实现
- **性能意识**：考虑 WASM 环境限制，优化包大小和运行性能

### 问题解决

- **遇到技术难题**：使用 WebSearch 搜索最新解决方案
- **Bevy 特定问题**：查阅官方文档和社区最佳实践
- **WASM 兼容性**：验证跨浏览器兼容性

### 开发流程

1. **理解需求**：仔细阅读 Issue 描述和验收标准
2. **技术调研**：评估现有解决方案，选择最佳实践
3. **实现开发**：遵循项目代码规范，添加必要测试
4. **验证测试**：确保功能正常，性能符合要求
5. **文档更新**：同步更新相关文档

## 🔧 关键技术上下文

### 项目特点

- **游戏类型**：物理堆叠游戏，设备倾斜控制
- **技术栈**：Rust + Bevy 0.14 + Rapier2D + WASM
- **目标平台**：移动端/桌面浏览器
- **性能目标**：WASM 包 < 2.3MB (gzip)

### 开发重点

- ECS 架构设计和系统优化
- 跨平台输入处理（倾斜 + 键盘）
- 物理引擎集成和性能调优
- WASM 构建优化和部署

需要更详细的技术信息时，请查阅 `docs/` 目录下的专门文档。
