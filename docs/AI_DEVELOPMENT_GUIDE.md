# 🤖 AI 代理开发指南

本文档为 AI 开发助手（如 Claude、GPT-4、Copilot）提供项目特定的上下文和指导，帮助 AI 更好地理解和参与 Tower Tumbler 项目的开发。

## 📋 项目概述

Tower Tumbler 是一个基于 Rust + Bevy + WASM 的物理堆叠游戏。主要特点：

- **引擎**: Bevy 0.14 ECS 架构
- **物理**: Rapier2D 物理引擎
- **平台**: WASM 目标，支持移动端和桌面浏览器
- **输入**: 设备倾斜控制（通过 JS-Rust 桥接）
- **目标**: WASM 包大小 < 2.3MB (gzip)

## 🎯 AI 开发任务类型

### 1. 功能实现

- 查看对应的 GitHub Issue 文档
- 遵循现有代码风格和架构模式
- 使用 Bevy ECS 和组件系统模式
- 考虑 WASM 环境的限制

### 2. 性能优化

- **包大小优化**: 关注 WASM 构建产物大小
- **运行时性能**: 避免不必要的系统调用和分配
- **依赖管理**: 谨慎添加新的 crate 依赖
- **条件编译**: 使用 `#[cfg(target_arch = "wasm32")]`

### 3. 调试修复

- 提供完整的错误上下文和堆栈跟踪
- 检查 Bevy 版本兼容性问题
- 验证 WASM 特定的运行时问题
- 考虑浏览器兼容性差异

## 🛠️ Bevy 特定指导

### ECS 模式最佳实践

```rust
// ✅ 推荐：使用 Bundle 组织相关组件
#[derive(Bundle)]
struct CrateBundle {
    sprite: SpriteBundle,
    crate_component: Crate,
    physics: RigidBody,
    collider: Collider,
    transform: Transform,
}

// ❌ 避免：过多的单独组件插入
commands.spawn((
    sprite,
    crate_component,
    physics,
    collider,
    transform,
    // ... 很多组件会影响性能
));
```

### 系统组织和调度

```rust
// ✅ 推荐：明确的系统执行顺序
app.add_systems(
    Update,
    (
        input_system,
        physics_step_system,
        game_logic_system,
        ui_update_system,
    ).chain() // 确保执行顺序
);

// ✅ 推荐：使用系统集合管理复杂依赖
app.add_systems(
    FixedUpdate,
    (
        physics_systems,
        collision_systems,
    ).in_set(PhysicsSet)
);

// ❌ 避免：隐式系统依赖
app.add_system(some_system); // 执行顺序不明确
```

### 资源管理

```rust
// ✅ 推荐：使用 Bevy 的资源系统
#[derive(Resource, Default)]
struct GameState {
    score: u32,
    high_score: u32,
    perfect_stacks: u32,
}

// ✅ 推荐：资源的线程安全访问
fn update_score(mut game_state: ResMut<GameState>) {
    game_state.score += 10;
}

// ❌ 避免：全局静态变量
static mut SCORE: u32 = 0; // 不安全且不符合 ECS 原则
```

### 查询优化

```rust
// ✅ 推荐：使用过滤器减少查询范围
fn move_crates(
    mut query: Query<&mut Transform, (With<Crate>, Changed<Velocity>)>,
    time: Res<Time>,
) {
    // 只处理速度发生变化的箱子
}

// ✅ 推荐：避免嵌套查询
fn collision_system(
    crates: Query<(Entity, &Transform), With<Crate>>,
    mut commands: Commands,
) {
    // 一次查询，多次使用
}

// ❌ 避免：查询过于宽泛
fn inefficient_system(
    query: Query<&Transform>, // 查询所有带 Transform 的实体
) {
    // 性能差
}
```

## 🌐 WASM 特定注意事项

### 1. 异步操作处理

```rust
// WASM 是单线程环境，避免阻塞操作
#[cfg(target_arch = "wasm32")]
fn load_assets_wasm(asset_server: Res<AssetServer>) {
    // 使用 Bevy 的异步资源加载
    let handle: Handle<Image> = asset_server.load("sprites/crate.png");
}

// 非 WASM 环境可以使用不同的加载策略
#[cfg(not(target_arch = "wasm32"))]
fn load_assets_native(asset_server: Res<AssetServer>) {
    // 可以使用更高级的预加载策略
}
```

### 2. 内存管理

```rust
// 为 WASM 配置轻量级分配器
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 避免大量内存分配
fn efficient_system(
    mut query: Query<&mut Transform, With<Crate>>,
    mut local_cache: Local<Vec<Entity>>, // 复用 Vec
) {
    local_cache.clear();
    // 使用本地缓存避免重复分配
}
```

### 3. JavaScript 互操作

```rust
use wasm_bindgen::prelude::*;

// 设备方向 API 绑定
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn requestDeviceOrientationPermission() -> js_sys::Promise;

    #[wasm_bindgen(js_name = DeviceOrientationEvent)]
    type DeviceOrientationEvent;

    #[wasm_bindgen(method, getter)]
    fn gamma(this: &DeviceOrientationEvent) -> Option<f64>;
}

// 游戏状态桥接
#[wasm_bindgen]
pub struct GameBridge {
    score: u32,
}

#[wasm_bindgen]
impl GameBridge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameBridge {
        GameBridge { score: 0 }
    }

    #[wasm_bindgen(getter)]
    pub fn score(&self) -> u32 {
        self.score
    }
}
```

## 📝 代码生成模板

### 新组件定义

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

// 如果需要序列化
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SerializableComponent {
    pub data: String,
}
```

### 新系统实现

```rust
use bevy::prelude::*;

pub fn new_system(
    mut query: Query<(&mut Transform, &NewComponent), Changed<NewComponent>>,
    time: Res<Time>,
) {
    for (mut transform, component) in &mut query {
        if component.active {
            // 系统逻辑
            transform.translation.x += component.field * time.delta_seconds();
        }
    }
}

// 带错误处理的系统
pub fn safe_system(
    mut query: Query<&mut Transform, With<NewComponent>>,
    mut error_events: EventWriter<ErrorEvent>,
) {
    for mut transform in &mut query {
        match update_transform(&mut transform) {
            Ok(_) => {},
            Err(e) => {
                error_events.send(ErrorEvent::TransformUpdate(e));
            }
        }
    }
}
```

### 新插件结构

```rust
use bevy::prelude::*;

pub struct NewPlugin;

impl Plugin for NewPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NewResource>()
            .add_event::<NewEvent>()
            .add_systems(Startup, setup_system)
            .add_systems(
                Update,
                (
                    input_system,
                    logic_system,
                    output_system,
                ).chain()
            )
            .add_systems(
                FixedUpdate,
                physics_system.run_if(in_state(GameState::Playing))
            );
    }
}

#[derive(Resource, Default)]
struct NewResource {
    data: Vec<Entity>,
}

#[derive(Event)]
struct NewEvent {
    entity: Entity,
    data: f32,
}
```

## ⚡ 性能优化清单

### 1. 查询优化

- [ ] 使用 `With<T>` 和 `Without<T>` 过滤器
- [ ] 利用 `Changed<T>` 减少不必要的计算
- [ ] 缓存复杂查询结果到 `Local<T>` 资源
- [ ] 避免在同一系统中多次查询相同数据

### 2. 系统优化

- [ ] 合理使用 `FixedUpdate` vs `Update`
- [ ] 使用 `run_if` 条件避免不必要的系统执行
- [ ] 利用并行查询 `par_iter_mut()` (注意 WASM 限制)
- [ ] 避免每帧分配内存

### 3. WASM 特定优化

- [ ] 使用 `wasm-opt -Oz` 优化构建产物
- [ ] 启用 LTO (Link Time Optimization)
- [ ] 条件编译移除调试代码：`#[cfg(not(debug_assertions))]`
- [ ] 使用 `#[inline]` 标记关键函数

### 4. 资源管理

- [ ] 及时清理不需要的实体：`commands.entity(entity).despawn()`
- [ ] 使用对象池模式重用游戏对象
- [ ] 避免克隆大型数据结构
- [ ] 使用引用计数 `Rc<T>` 共享只读数据

## 🐛 常见问题和解决方案

### Q: Bevy 系统无法找到实体或组件

**A**:

- 检查组件是否正确添加到实体
- 注意 `Commands` 的修改会在下一帧生效
- 使用 `Query` 的 `get()` 方法进行安全查询

### Q: WASM 构建失败

**A**:

- 确保所有依赖都支持 `wasm32-unknown-unknown` 目标
- 检查是否使用了不兼容的系统调用
- 使用 `cargo tree` 检查依赖树

### Q: 物理模拟不稳定

**A**:

- 检查 `FixedUpdate` 的时间步长设置
- 确保与 Rapier 配置的时间步长匹配
- 调整 Rapier 的稳定性参数

### Q: 倾斜控制在某些浏览器不工作

**A**:

- 检查设备方向权限是否已请求
- 实现降级到键盘/触摸控制
- 测试不同浏览器的 API 差异

### Q: WASM 包过大

**A**:

- 使用 `wee_alloc` 替代默认分配器
- 移除未使用的 Bevy 功能
- 启用 `codegen-units = 1` 和 `lto = true`

## 📚 参考资源

### 官方文档

- [Bevy Book](https://bevyengine.org/learn/book/)
- [Bevy API 文档](https://docs.rs/bevy/)
- [Rapier2D 用户指南](https://rapier.rs/docs/)
- [wasm-bindgen 教程](https://rustwasm.github.io/wasm-bindgen/)

### 社区资源

- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/programming.html)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [WASM 性能优化指南](https://web.dev/wasm-performance/)

### 项目特定

- [项目 Issue 模板](https://github.com/xdanger/tower-tumbler/issues)
- [贡献指南](../CONTRIBUTING.md)
- [Bevy 兼容性文档](BEVY_COMPATIBILITY.md)

## 🎨 代码风格指南

### Rust 代码风格

````rust
// ✅ 推荐的命名约定
struct GameState;           // 类型用 PascalCase
const MAX_SCORE: u32 = 100; // 常量用 SCREAMING_SNAKE_CASE
fn update_score() {}        // 函数用 snake_case
let current_score = 0;      // 变量用 snake_case

// ✅ 推荐的错误处理
fn safe_operation() -> Result<(), GameError> {
    // 使用 Result 类型进行错误处理
    Ok(())
}

// ✅ 推荐的文档注释
/// Updates the game score based on stacking accuracy.
///
/// # Arguments
///
/// * `accuracy` - A float between 0.0 and 1.0
///
/// # Examples
///
/// ```
/// update_score_from_accuracy(0.9);
/// ```
fn update_score_from_accuracy(accuracy: f32) {
    // 实现
}
````

### 项目约定

- 使用 `cargo fmt` 自动格式化代码
- 遵循 `cargo clippy` 的建议
- 添加适当的文档注释
- 保持函数简短（通常 < 50 行）
- 使用有意义的变量和函数名

## 💡 开发提示

### 开始开发前

1. **阅读 Issue**: 完整理解需求和验收标准
2. **检查现有代码**: 了解相关模块的实现模式
3. **运行测试**: 确保当前代码状态正常
4. **设置环境**: 验证开发环境配置正确

### 开发过程中

1. **小步迭代**: 频繁测试和提交
2. **遵循模式**: 参考现有代码的架构和命名
3. **性能意识**: 考虑 WASM 环境的限制
4. **错误处理**: 优雅处理边界情况

### 完成开发后

1. **运行测试**: 确保所有测试通过
2. **性能检查**: 验证 WASM 包大小和运行性能
3. **文档更新**: 更新相关文档和注释
4. **Code Review**: 自我审查代码质量

### 提交代码时

1. **清晰的提交信息**: 使用语义化提交格式
2. **原子性提交**: 每个提交只做一件事
3. **测试覆盖**: 确保新功能有对应测试
4. **文档同步**: 保持代码和文档一致

## 🔧 调试和故障排除

### 开发环境问题

```bash
# 检查 Rust 工具链
rustc --version
cargo --version

# 检查 WASM 工具
wasm-pack --version
trunk --version

# 清理构建缓存
cargo clean
rm -rf dist/
```

### 运行时调试

```rust
// 使用 Bevy 的调试工具
#[cfg(debug_assertions)]
fn debug_system(
    query: Query<(Entity, &Transform), With<Crate>>,
) {
    for (entity, transform) in &query {
        println!("Entity {:?} at {:?}", entity, transform.translation);
    }
}

// WASM 控制台输出
#[cfg(target_arch = "wasm32")]
fn wasm_log(message: &str) {
    web_sys::console::log_1(&message.into());
}
```

### 性能分析

```bash
# 分析 WASM 包大小
wasm-pack build --target web --out-dir pkg
ls -la pkg/
wasm-opt -Oz pkg/tower_tumbler_bg.wasm -o pkg/optimized.wasm

# 性能分析
cargo bench
```

这份指南应该能帮助 AI 更好地理解和参与 Tower Tumbler 项目的开发。记住始终关注代码质量、性能和用户体验！
