# 🏗️ Tower Tumbler

<div align="center">
  <strong>基于物理的堆叠游戏，使用设备倾斜控制</strong>
  <br />
  <br />

[![Build Status](https://github.com/xdanger/tower-tumbler/workflows/CI/badge.svg)](https://github.com/xdanger/tower-tumbler/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Bevy](https://img.shields.io/badge/bevy-0.14-green.svg)](https://bevyengine.org)
[![WASM](https://img.shields.io/badge/platform-wasm-orange.svg)](https://webassembly.org)

</div>

## 📋 用途

Tower Tumbler 是一个基于 Rust 和 Bevy 引擎开发的物理堆叠游戏。玩家需要通过倾斜设备来控制落下的方块，精准地将它们叠加在一起，形成稳定而高耸的塔楼。游戏特色包括真实的物理模拟、直观的倾斜控制和完美叠加奖励机制。

## 🎮 游戏特色

- **倾斜控制**：使用设备陀螺仪进行直观控制
- **物理模拟**：基于 Rapier2D 的真实物理效果
- **完美叠加**：精准操作获得额外分数
- **跨平台**：支持移动端和桌面端浏览器

## 🚀 使用方法

### 环境配置

确保系统已安装以下依赖：

- Rust 1.75+ (with cargo)
- Node.js 18+ (for tooling)
- wasm-pack 0.12+
- Trunk 0.17+

### 安装/初始化

```bash
# 克隆仓库
git clone https://github.com/xdanger/tower-tumbler.git
cd tower-tumbler

# 安装 Rust 工具链
rustup target add wasm32-unknown-unknown

# 安装构建工具
cargo install trunk wasm-bindgen-cli
```

### 编译/构建步骤

#### 开发模式

```bash
# 启动开发服务器（热重载）
trunk serve --open

# 或者指定端口
trunk serve --port 8080 --open
```

#### 发布构建

```bash
# 构建优化版本
trunk build --release

# 或使用构建脚本
./scripts/build-release.sh
```

### 如何使用该组件

#### 基础游戏操作

1. **移动端**：倾斜设备左右来控制方块落下方向
2. **桌面端**：使用左右箭头键或 A/D 键控制
3. **目标**：将方块精准叠加，形成稳定的塔楼
4. **得分**：完美叠加获得额外分数

#### 集成到其他项目

```rust
use tower_tumbler::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TowerTumblerPlugin)
        .run();
}
```

### 测试流程

```bash
# 运行所有测试
cargo test

# 运行 WASM 测试
wasm-pack test --headless --firefox

# 端到端测试
npm run test:e2e

# 性能测试
cargo bench
```

### 调试和日志

项目内置了完整的日志系统，用于开发和调试：

#### 启动开发环境（带日志）

```bash
# 启动浏览器控制台日志服务器
cd utils && ./start-log-server.sh

# 启动开发服务器（在另一个终端）
cd utils && ./start-trunk-server.sh

# 访问 http://localhost:8080
```

#### 查看日志文件

```bash
# 浏览器控制台日志
tail -f logs/browser/console-$(date +%Y-%m-%d).log

# 服务器日志
tail -f logs/server/trunk-$(date +%Y-%m-%d).log
```

更多调试工具信息请参考 [utils/README.md](utils/README.md)。

## 🛠️ 技术栈

- **[Bevy](https://bevyengine.org)** 0.14 - Rust 游戏引擎
- **[Rapier2D](https://rapier.rs)** - 物理引擎
- **[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)** - WASM 绑定
- **[Trunk](https://trunkrs.dev)** - WASM 应用打包工具

### 项目结构

```
tower-tumbler/
├── src/
│   ├── core/          # 核心游戏逻辑
│   │   ├── game.rs    # 游戏状态管理
│   │   ├── scoring.rs # 计分系统
│   │   └── mod.rs
│   ├── input/         # 输入处理系统
│   │   ├── tilt.rs    # 倾斜检测
│   │   ├── keyboard.rs # 键盘输入
│   │   └── mod.rs
│   ├── physics/       # 物理引擎集成
│   │   ├── world.rs   # 物理世界设置
│   │   ├── bodies.rs  # 刚体管理
│   │   └── mod.rs
│   ├── ui/            # UI 组件
│   │   ├── hud.rs     # 抬头显示
│   │   ├── menu.rs    # 菜单系统
│   │   └── mod.rs
│   ├── lib.rs         # 库入口
│   └── main.rs        # 可执行入口
├── assets/            # 游戏资源
│   ├── sprites/       # 精灵图像
│   ├── sounds/        # 音效文件
│   └── fonts/         # 字体文件
├── scripts/           # 构建和工具脚本
│   ├── build-release.sh
│   ├── optimize-wasm.sh
│   └── deploy.sh
├── docs/              # 项目文档
│   ├── AI_DEVELOPMENT_GUIDE.md # AI 开发详细指南
│   ├── BEVY_COMPATIBILITY.md   # Bevy 兼容性文档
│   ├── QUICK_REFERENCE.md      # 快速参考
│   └── api/           # API 文档
├── AGENT.md           # AI 助手项目导航
├── tests/             # 测试文件
│   ├── integration/
│   └── e2e/
└── Cargo.toml         # Rust 包配置
```

## 📱 设备兼容性

| 平台            | 倾斜控制      | 后备控制 | 测试状态 |
| --------------- | ------------- | -------- | -------- |
| iOS Safari      | ✅ (需要权限) | ✅       | ✅       |
| Android Chrome  | ✅            | ✅       | ✅       |
| Desktop Chrome  | ❌            | ✅       | ✅       |
| Desktop Firefox | ❌            | ✅       | ✅       |
| Desktop Safari  | ❌            | ✅       | 🔄       |

## 📋 路线图

### ✅ 已完成

- [x] 基础物理和渲染系统
- [x] 设备倾斜输入支持
- [x] 基础得分系统
- [x] 完美叠加机制
- [x] WASM 构建优化

### 🔄 进行中

- [ ] 音效和音乐系统
- [ ] 多种游戏模式
- [ ] 移动端 UI 优化

### ⏳ 计划中

- [ ] 排行榜系统
- [ ] 多人对战模式
- [ ] 成就系统
- [ ] 自定义主题
- [ ] 游戏回放功能

### 🔍 审核中

- [ ] 性能优化方案
- [ ] 无障碍功能

### ❌ 已阻塞

- [ ] PWA 离线支持（等待 Bevy WASM 改进）

## 📝 下一步行动项

### 高优先级

1. **音效集成** - 使用 Web Audio API 通过 wasm-bindgen
2. **UI/UX 改进** - 响应式设计和移动端体验
3. **性能优化** - 减少 WASM 包大小到 < 2MB

### 中优先级

4. **测试覆盖** - 增加单元测试和集成测试
5. **文档完善** - API 文档和开发者指南
6. **CI/CD 流程** - 自动化测试和部署

### 低优先级

7. **多语言支持** - 国际化框架
8. **高级功能** - 特殊方块类型和效果

## 🤝 贡献指南

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

### AI 辅助开发

本项目支持 AI 辅助开发，详见 [AGENT.md](AGENT.md)。

## 📚 相关资源

- [Bevy 官方文档](https://bevyengine.org/learn/)
- [Rapier2D 用户指南](https://rapier.rs/docs/)
- [WASM 最佳实践](https://rustwasm.github.io/docs/book/)
- [项目 Wiki](https://github.com/xdanger/tower-tumbler/wiki)

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Bevy](https://bevyengine.org) 社区提供的优秀游戏引擎
- [Rapier](https://rapier.rs) 团队的物理引擎
- 所有贡献者和测试用户的支持
