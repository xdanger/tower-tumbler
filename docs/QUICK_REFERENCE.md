# 🚀 Tower Tumbler 快速参考

这是一个简洁的快速参考指南，包含最常用的命令、代码片段和项目信息。

## 📋 目录

- [常用命令](#常用命令)
- [项目结构](#项目结构)
- [关键组件](#关键组件)
- [系统架构](#系统架构)
- [代码模板](#代码模板)
- [调试技巧](#调试技巧)
- [性能优化](#性能优化)
- [常见错误](#常见错误)

## 💻 常用命令

### 开发命令

```bash
# 快速启动开发环境
trunk serve --open                    # 启动开发服务器
trunk serve --port 8080 --open       # 指定端口启动

# 构建命令
trunk build                          # 开发构建
trunk build --release               # 发布构建
./scripts/build-optimized.sh        # 优化构建

# 测试命令
cargo test                           # 运行所有测试
cargo test --lib                     # 仅运行库测试
wasm-pack test --headless --firefox  # WASM 测试
cargo bench                          # 性能测试
```

### 代码质量

```bash
# 格式化和检查
cargo fmt                            # 代码格式化
cargo clippy -- -D warnings         # 代码检查
cargo check                          # 编译检查
cargo audit                          # 安全审计

# 文档生成
cargo doc --open                     # 生成并打开文档
cargo doc --document-private-items   # 包含私有项的文档
```

### 依赖管理

```bash
# 依赖操作
cargo tree                           # 查看依赖树
cargo update                         # 更新依赖
cargo outdated                       # 检查过期依赖
cargo machete                        # 移除未使用依赖
```

### WASM 优化

```bash
# WASM 包大小分析
wasm-pack build --target web
wasm-opt -Oz pkg/tower_tumbler_bg.wasm -o pkg/optimized.wasm
ls -la pkg/*.wasm                    # 对比文件大小

# 压缩测试
gzip -9 -c pkg/optimized.wasm | wc -c  # 查看 gzip 压缩大小
```

## 📁 项目结构

```
tower-tumbler/
├── 📁 src/
│   ├── 📁 core/                     # 核心游戏逻辑
│   │   ├── game.rs                  # 游戏状态管理
│   │   ├── scoring.rs               # 计分系统
│   │   └── mod.rs
│   ├── 📁 input/                    # 输入处理
│   │   ├── tilt.rs                  # 倾斜检测
│   │   ├── keyboard.rs              # 键盘输入
│   │   └── mod.rs
│   ├── 📁 physics/                  # 物理系统
│   │   ├── world.rs                 # 物理世界
│   │   ├── bodies.rs                # 刚体管理
│   │   └── mod.rs
│   ├── 📁 ui/                       # 用户界面
│   │   ├── hud.rs                   # 抬头显示
│   │   ├── menu.rs                  # 菜单系统
│   │   └── mod.rs
│   ├── lib.rs                       # 库入口
│   └── main.rs                      # 主程序入口
├── 📁 assets/                       # 游戏资源
├── 📁 docs/                         # 项目文档
├── 📁 scripts/                      # 构建脚本
└── 📄 Cargo.toml                    # 项目配置
```

### 重要文件说明

| 文件 | 用途 | 修改频率 |
|------|------|---------|
| `src/main.rs` | 程序入口，插件注册 | 低 |
| `src/core/game.rs` | 游戏状态和逻辑 | 高 |
| `src/input/tilt.rs` | 设备倾斜控制 | 中 |
| `src/physics/world.rs` | 物理世界配置 | 低 |
| `src/ui/hud.rs` | 游戏界面 UI | 中 |
| `Cargo.toml` | 依赖和配置 | 中 |
| `Trunk.toml` | WASM 构建配置 | 低 |

## 🧩 关键组件

### 核心组件

```rust
// 游戏对象
#[derive(Component)]
struct Crate {
    pub fall_speed: f32,
    pub perfect_landing: bool,
}

// 游戏控制
#[derive(Component)]
struct GameController {
    pub tilt_sensitivity: f32,
}

// UI 组件
#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct HighScoreDisplay;
```

### 资源类型

```rust
// 游戏状态
#[derive(Resource)]
struct GameScore {
    pub current: u32,
    pub high: u32,
    pub multiplier: f32,
}

// 输入状态
#[derive(Resource)]
struct TiltInput {
    pub angle: f32,
    pub enabled: bool,
}

// 游戏配置
#[derive(Resource)]
struct GameConfig {
    pub physics_scale: f32,
    pub spawn_interval: f32,
}
```

### 游戏状态

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,        // 资源加载
    Menu,          // 主菜单
    Playing,       // 游戏进行中
    Paused,        // 暂停
    GameOver,      // 游戏结束
}
```

## 🏗️ 系统架构

### 系统执行顺序

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        
        // 启动系统
        .add_systems(Startup, (
            setup_camera,
            setup_ui,
            load_assets,
        ))
        
        // 更新系统
        .add_systems(Update, (
            // 输入处理
            handle_keyboard_input,
            handle_tilt_input,
            
            // 游戏逻辑
            spawn_crates.run_if(in_state(GameState::Playing)),
            update_score,
            check_game_over,
            
            // UI 更新
            update_score_display,
            update_menu.run_if(in_state(GameState::Menu)),
        ))
        
        // 固定更新 (物理)
        .add_systems(FixedUpdate, (
            physics_step,
            collision_detection,
        ).run_if(in_state(GameState::Playing)))
        
        .run();
}
```

### 插件组织

```rust
// 核心游戏插件
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_resource::<GameScore>()
            .add_systems(/* ... */);
    }
}

// 输入处理插件
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TiltInput>()
            .add_systems(/* ... */);
    }
}
```

## 📝 代码模板

### 新组件模板

```rust
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct NewComponent {
    pub field: f32,
    pub active: bool,
}

impl Default for NewComponent {
    fn default() -> Self {
        Self {
            field: 0.0,
            active: true,
        }
    }
}
```

### 新系统模板

```rust
pub fn new_system(
    // 查询可变组件
    mut query: Query<(&mut Transform, &NewComponent), Changed<NewComponent>>,
    // 只读资源
    time: Res<Time>,
    // 可变资源
    mut score: ResMut<GameScore>,
    // 命令队列
    mut commands: Commands,
) {
    for (mut transform, component) in &mut query {
        if component.active {
            // 系统逻辑
            transform.translation.x += component.field * time.delta_seconds();
        }
    }
}
```

### 事件处理模板

```rust
// 定义事件
#[derive(Event)]
pub struct CrateLanded {
    pub entity: Entity,
    pub accuracy: f32,
}

// 发送事件
fn send_event_system(
    mut events: EventWriter<CrateLanded>,
    query: Query<Entity, With<JustLanded>>,
) {
    for entity in &query {
        events.send(CrateLanded {
            entity,
            accuracy: 0.95,
        });
    }
}

// 接收事件
fn receive_event_system(
    mut events: EventReader<CrateLanded>,
    mut score: ResMut<GameScore>,
) {
    for event in events.read() {
        score.current += (event.accuracy * 100.0) as u32;
    }
}
```

## 🐛 调试技巧

### 控制台输出

```rust
// 不同级别的日志
info!("Game started with score: {}", score.current);
warn!("Low performance detected: {} fps", fps);
error!("Failed to load asset: {}", asset_path);
debug!("Entity {:?} spawned at {:?}", entity, position);
trace!("Physics step completed in {}ms", duration);

// WASM 特定输出
#[cfg(target_arch = "wasm32")]
fn wasm_log(message: &str) {
    web_sys::console::log_1(&message.into());
}
```

### 可视化调试

```rust
use bevy::prelude::*;

fn debug_draw_system(
    query: Query<&Transform, With<Crate>>,
    mut gizmos: Gizmos,
) {
    for transform in &query {
        // 绘制边界框
        gizmos.rect_2d(
            transform.translation.truncate(),
            0.0,
            Vec2::new(32.0, 32.0),
            Color::RED,
        );
        
        // 绘制速度向量
        gizmos.line_2d(
            transform.translation.truncate(),
            transform.translation.truncate() + Vec2::new(0.0, -50.0),
            Color::BLUE,
        );
    }
}
```

### 运行时检查

```rust
fn validation_system(
    query: Query<(Entity, &Transform), With<Crate>>,
    mut commands: Commands,
) {
    for (entity, transform) in &query {
        // 检查异常位置
        if transform.translation.y < -1000.0 {
            warn!("Entity {:?} fell out of bounds", entity);
            commands.entity(entity).despawn();
        }
        
        // 检查 NaN 值
        if transform.translation.is_nan() {
            error!("Entity {:?} has NaN position", entity);
            commands.entity(entity).despawn();
        }
    }
}
```

## ⚡ 性能优化

### 查询优化

```rust
// ✅ 使用过滤器减少查询范围
fn optimized_system(
    query: Query<&mut Transform, (With<Crate>, Changed<Velocity>)>,
) {
    // 只处理速度变化的箱子
}

// ✅ 缓存查询结果
fn cached_system(
    query: Query<&Transform, With<Crate>>,
    mut local_cache: Local<Vec<Vec3>>,
) {
    local_cache.clear();
    for transform in &query {
        local_cache.push(transform.translation);
    }
    // 使用缓存的数据
}

// ❌ 避免宽泛查询
fn inefficient_system(
    query: Query<&Transform>, // 查询所有 Transform
) {
    // 性能差
}
```

### 内存管理

```rust
// 对象池模式
#[derive(Resource, Default)]
struct EntityPool<T: Component> {
    inactive: Vec<Entity>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Component> EntityPool<T> {
    fn get_or_spawn(&mut self, commands: &mut Commands) -> Entity {
        self.inactive.pop().unwrap_or_else(|| {
            commands.spawn_empty().id()
        })
    }
    
    fn return_entity(&mut self, entity: Entity) {
        self.inactive.push(entity);
    }
}
```

### WASM 特定优化

```rust
// 条件编译优化
#[cfg(not(target_arch = "wasm32"))]
const PHYSICS_SUBSTEPS: usize = 8;

#[cfg(target_arch = "wasm32")]
const PHYSICS_SUBSTEPS: usize = 4;  // WASM 中减少物理步数

// 内存分配器
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

## 🚨 常见错误

### 编译错误

```bash
# 错误：missing features
error: failed to build archive: failed to parse
解决：检查 Cargo.toml 中的 Bevy 特性配置

# 错误：wasm-bindgen version mismatch  
error: it looks like the Rust project used to create this was linked
解决：cargo clean && cargo update

# 错误：can't find wasm-opt
解决：npm install -g wasm-opt
```

### 运行时错误

```rust
// 错误：实体不存在
// 原因：Commands 延迟执行
// 解决：使用 Query 检查存在性
if let Ok(mut transform) = query.get_mut(entity) {
    transform.translation += Vec3::X;
}

// 错误：资源未初始化
// 原因：忘记注册资源
// 解决：在 App 中添加
app.init_resource::<MyResource>();
```

### WASM 特有问题

```javascript
// 错误：AudioContext 未激活
// 解决：需要用户交互后播放音频
document.addEventListener('click', () => {
    if (audioContext.state === 'suspended') {
        audioContext.resume();
    }
});

// 错误：设备方向权限
// 解决：请求权限
if (typeof DeviceOrientationEvent.requestPermission === 'function') {
    DeviceOrientationEvent.requestPermission();
}
```

## 🔗 快速链接

### 开发资源
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/) - 最佳实践指南
- [Rust WASM Book](https://rustwasm.github.io/docs/book/) - WASM 开发指南
- [MDN Web APIs](https://developer.mozilla.org/docs/Web/API) - Web API 参考

### 项目链接
- [GitHub 仓库](https://github.com/xdanger/tower-tumbler)
- [Issues 追踪](https://github.com/xdanger/tower-tumbler/issues)
- [项目看板](https://github.com/xdanger/tower-tumbler/projects)

### 工具和扩展
- [rust-analyzer](https://rust-analyzer.github.io/) - LSP 服务
- [trunk](https://trunkrs.dev/) - WASM 打包工具
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) - WASM 构建工具

---

💡 **提示**: 将此文档添加到浏览器书签，开发时随时查阅！

🚀 **更新**: 此文档会随项目发展持续更新，建议定期查看最新版本。