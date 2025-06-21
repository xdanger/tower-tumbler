# 🎮 Bevy 兼容性指南

本文档详细说明了 Tower Tumbler 项目中 Bevy 引擎的使用规范、版本兼容性要求以及 WASM 部署的特殊配置。

## 📋 版本要求

### 核心依赖版本

- **Bevy**: 0.14.x (最新稳定版)
- **Rust**: 1.75+ (MSRV - Minimum Supported Rust Version)
- **wasm-bindgen**: 0.2.90+
- **Rapier2D**: 0.21+ (兼容 Bevy 0.14)

### Cargo.toml 配置

```toml
[package]
name = "tower-tumbler"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"

[dependencies.bevy]
version = "0.14"
default-features = false
features = [
    # 核心功能
    "bevy_asset",              # 资源管理
    "bevy_core_pipeline",      # 渲染管道
    "bevy_render",             # 渲染系统
    "bevy_sprite",             # 2D 精灵
    "bevy_ui",                 # UI 系统
    "bevy_winit",              # 窗口管理
    "bevy_input",              # 输入处理
    
    # WASM 特定
    "webgl2",                  # WebGL2 渲染后端
    
    # 可选功能（按需启用）
    "bevy_audio",              # 音频系统（WASM 兼容性有限）
    "bevy_text",               # 文本渲染
    "bevy_gizmos",             # 调试绘制
    
    # 开发时功能
    "dynamic_linking",         # 快速编译（仅开发时）
]

[dependencies.bevy_rapier2d]
version = "0.26"
features = [
    "simd-stable",            # SIMD 优化
    "wasm-bindgen",           # WASM 支持
]

# WASM 优化配置
[profile.release]
opt-level = "s"               # 优化大小
lto = true                    # 链接时优化
codegen-units = 1             # 单个代码生成单元
panic = "abort"               # 减少二进制大小
strip = true                  # 移除调试符号

[profile.dev.package."*"]
opt-level = 3                 # 优化依赖包编译速度
```

## 🛠️ 特性使用指南

### 必需特性

这些特性是项目正常运行所必需的：

```toml
features = [
    "bevy_asset",             # 图片、字体等资源加载
    "bevy_core_pipeline",     # 基础渲染管道
    "bevy_render",            # 渲染系统核心
    "bevy_sprite",            # 2D 图形渲染
    "bevy_ui",                # UI 界面
    "bevy_winit",             # 窗口和事件处理
    "bevy_input",             # 键盘、鼠标、触摸输入
    "webgl2",                 # WASM WebGL2 后端
]
```

### 可选特性

根据项目需求选择性启用：

```rust
// 文本渲染（如果需要显示分数、UI 文本）
#[cfg(feature = "bevy_text")]
fn setup_text_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/game_font.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    });
}

// 音频系统（WASM 中需要特殊处理）
#[cfg(all(feature = "bevy_audio", not(target_arch = "wasm32")))]
fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/bgm.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}
```

### 开发时特性

仅在开发环境中启用，不包含在发布版本中：

```toml
[dependencies.bevy]
# ... 其他配置
features = [
    # ... 其他特性
    "dynamic_linking",        # 仅开发时：快速编译
    "bevy_dev_tools",         # 仅开发时：开发工具
]

# 或使用条件编译
[target.'cfg(debug_assertions)'.dependencies.bevy]
features = ["bevy_inspector_egui"]  # 调试工具
```

## 🌐 WASM 特殊配置

### 1. 窗口设置

WASM 环境中的窗口配置需要特殊处理：

```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    
    // WASM 特定的插件配置
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Tower Tumbler".to_string(),
            
            // WASM 特定配置
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#game-canvas".to_string()),
            
            // 自适应画布大小
            #[cfg(target_arch = "wasm32")]
            fit_canvas_to_parent: true,
            
            // 防止上下文菜单
            #[cfg(target_arch = "wasm32")]
            prevent_default_event_handling: true,
            
            // 桌面环境配置
            #[cfg(not(target_arch = "wasm32"))]
            resolution: (800.0, 600.0).into(),
            
            ..default()
        }),
        ..default()
    }));
    
    app.run();
}
```

### 2. 资源加载

WASM 中的资源加载是异步的，需要相应处理：

```rust
#[derive(Resource, Default)]
struct GameAssets {
    crate_texture: Handle<Image>,
    background_texture: Handle<Image>,
    loaded: bool,
}

fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    if !game_assets.loaded {
        // WASM 中路径必须是相对路径
        game_assets.crate_texture = asset_server.load("sprites/crate.png");
        game_assets.background_texture = asset_server.load("sprites/background.png");
        game_assets.loaded = true;
    }
}

fn check_assets_loaded(
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game_assets.loaded {
        let crate_loaded = asset_server.get_load_state(&game_assets.crate_texture)
            == Some(LoadState::Loaded);
        let bg_loaded = asset_server.get_load_state(&game_assets.background_texture)
            == Some(LoadState::Loaded);
            
        if crate_loaded && bg_loaded {
            next_state.set(GameState::Playing);
        }
    }
}
```

### 3. 音频处理

WASM 中的音频需要用户交互后才能播放：

```rust
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

// 使用 JS 绑定处理音频
#[wasm_bindgen]
extern "C" {
    fn play_sound(name: &str);
    fn set_volume(volume: f32);
}

#[cfg(target_arch = "wasm32")]
fn play_audio_wasm(sound_name: &str) {
    play_sound(sound_name);
}

#[cfg(not(target_arch = "wasm32"))]
fn play_audio_native(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/drop.ogg"),
        settings: PlaybackSettings::DESPAWN,
    });
}

// 统一的音频接口
fn play_drop_sound() {
    #[cfg(target_arch = "wasm32")]
    play_audio_wasm("drop");
    
    #[cfg(not(target_arch = "wasm32"))]
    play_audio_native();
}
```

### 4. 输入处理

设备倾斜输入需要通过 JavaScript API：

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn request_device_orientation_permission() -> js_sys::Promise;
    fn get_device_orientation() -> Option<f64>;
}

#[derive(Resource, Default)]
struct TiltInput {
    angle: f32,
    enabled: bool,
}

#[cfg(target_arch = "wasm32")]
fn setup_tilt_input(mut tilt_input: ResMut<TiltInput>) {
    // 请求设备方向权限
    let _ = request_device_orientation_permission();
    tilt_input.enabled = true;
}

#[cfg(target_arch = "wasm32")]
fn update_tilt_input(mut tilt_input: ResMut<TiltInput>) {
    if let Some(angle) = get_device_orientation() {
        tilt_input.angle = angle as f32;
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn update_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tilt_input: ResMut<TiltInput>,
) {
    let mut angle = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        angle = -30.0;
    } else if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        angle = 30.0;
    }
    tilt_input.angle = angle;
}
```

## ⚡ 性能优化建议

### 1. 系统优化

```rust
// 使用 Changed 查询减少计算
fn update_score_display(
    score: Res<GameScore>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    // 只在分数改变时更新 UI
    if score.is_changed() {
        for mut text in &mut query {
            text.sections[0].value = format!("Score: {}", score.value);
        }
    }
}

// 使用条件系统避免不必要的执行
fn physics_system(
    mut query: Query<(&mut Transform, &Velocity), With<Crate>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.linear * time.delta_seconds();
    }
}

// 将系统添加到 app 时使用条件
app.add_systems(
    FixedUpdate,
    physics_system.run_if(in_state(GameState::Playing))
);
```

### 2. 内存管理

```rust
// 对象池模式重用实体
#[derive(Resource, Default)]
struct CratePool {
    inactive: Vec<Entity>,
}

fn spawn_crate(
    mut commands: Commands,
    mut pool: ResMut<CratePool>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = if let Some(entity) = pool.inactive.pop() {
        // 重用现有实体
        commands.entity(entity).insert(ActiveCrate);
        entity
    } else {
        // 创建新实体
        commands.spawn(CrateBundle::new()).id()
    };
}

fn despawn_crate(
    mut commands: Commands,
    mut pool: ResMut<CratePool>,
    query: Query<Entity, (With<Crate>, With<ShouldDespawn>)>,
) {
    for entity in &query {
        commands.entity(entity).remove::<ActiveCrate>();
        pool.inactive.push(entity);
    }
}
```

### 3. 渲染优化

```rust
// 使用精灵图集减少绘制调用
fn setup_sprite_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("sprites/game_atlas.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(64, 64),  // 每个精灵的大小
        4, 4,                // 网格尺寸
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    commands.insert_resource(GameAtlas {
        texture: texture_handle,
        layout: texture_atlas_handle,
    });
}

// 批量处理相似实体
fn batch_render_crates(
    query: Query<(&Transform, &Sprite), With<Crate>>,
    mut gizmos: Gizmos,
) {
    // 批量渲染可以减少 GPU 调用
    for (transform, sprite) in &query {
        // 渲染逻辑
    }
}
```

## 🚨 已知问题和解决方案

### 问题 1: Bevy 0.14 音频在 WASM 中的限制

**症状**: 音频在 WASM 中无法正常播放或需要用户交互

**解决方案**:
```rust
// 使用 howler.js 或 Web Audio API 通过 wasm-bindgen
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "howler"])]
    fn play_audio(src: &str);
}

#[cfg(target_arch = "wasm32")]
fn play_sound_wasm(sound: &str) {
    play_audio(sound);
}
```

### 问题 2: 大型纹理加载性能

**症状**: 游戏首次加载时间过长

**解决方案**:
```rust
// 使用渐进式加载和纹理压缩
fn progressive_loading(
    mut loading_state: ResMut<LoadingState>,
    asset_server: Res<AssetServer>,
) {
    match loading_state.stage {
        LoadingStage::Essential => {
            // 只加载必需资源
            loading_state.crate_texture = asset_server.load("sprites/crate_small.png");
        },
        LoadingStage::Optional => {
            // 加载高质量资源
            loading_state.crate_texture_hd = asset_server.load("sprites/crate_hd.png");
        },
    }
}
```

### 问题 3: Rapier2D 物理抖动

**症状**: 物理对象在某些情况下出现抖动或不稳定

**解决方案**:
```rust
use bevy_rapier2d::prelude::*;

fn setup_physics_world(mut rapier_config: ResMut<RapierConfiguration>) {
    // 调整物理世界参数
    rapier_config.gravity = Vec2::new(0.0, -980.0);  // 标准重力
    rapier_config.physics_pipeline_active = true;
    rapier_config.query_pipeline_active = true;
}

fn setup_stable_physics(mut commands: Commands) {
    // 使用稳定的物理设置
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5),
        Restitution::coefficient(0.1),      // 低弹性
        Friction::coefficient(0.8),         // 高摩擦
        Damping {
            linear_damping: 0.1,            // 线性阻尼
            angular_damping: 0.1,           // 角阻尼
        },
        AdditionalMassProperties::Mass(1.0), // 合适的质量
    ));
}
```

### 问题 4: WASM 包大小过大

**症状**: 构建的 WASM 文件超过 2MB

**解决方案**:
```toml
# Cargo.toml 优化配置
[profile.release]
opt-level = "s"          # 优化大小而非速度
lto = true               # 链接时优化
codegen-units = 1        # 单个代码生成单元
panic = "abort"          # 减少异常处理代码
strip = true             # 移除调试符号

# 移除不必要的 Bevy 特性
[dependencies.bevy]
version = "0.14"
default-features = false
features = [
    # 只保留必需的特性
    "bevy_sprite",
    "bevy_ui",
    "webgl2",
]
```

```bash
# 构建脚本优化
#!/bin/bash
# scripts/build-optimized.sh

# 构建 WASM
trunk build --release

# 使用 wasm-opt 进一步优化
wasm-opt -Oz dist/tower-tumbler_bg.wasm -o dist/optimized.wasm

# 压缩
gzip -9 dist/optimized.wasm

echo "WASM size: $(du -h dist/optimized.wasm.gz)"
```

## 📋 版本升级指南

### 从 Bevy 0.13 升级到 0.14

主要变更点：

1. **系统参数变更**
```rust
// 0.13
fn old_system(
    mut commands: Commands,
    query: Query<Entity, With<SomeComponent>>,
) {
    // ...
}

// 0.14
fn new_system(
    mut commands: Commands,
    query: Query<Entity, With<SomeComponent>>,
) {
    // 大部分 API 保持兼容
}
```

2. **资源处理变更**
```rust
// 检查新的资源 API
fn handle_assets(
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
) {
    // 使用新的资源 API
}
```

### 升级步骤

1. **更新 Cargo.toml**
```bash
cargo update
cargo tree | grep bevy  # 检查版本一致性
```

2. **修复编译错误**
```bash
cargo check
cargo clippy
```

3. **测试功能**
```bash
trunk serve  # 测试 WASM 构建
cargo test   # 运行测试套件
```

## 🛠️ 调试工具

### 开发时调试

```rust
// 使用 Bevy 的调试工具
#[cfg(debug_assertions)]
fn debug_system(
    query: Query<(Entity, &Transform), With<Crate>>,
    mut gizmos: Gizmos,
) {
    for (entity, transform) in &query {
        // 绘制调试信息
        gizmos.circle_2d(
            transform.translation.truncate(),
            10.0,
            Color::RED,
        );
        
        // 控制台输出
        info!("Entity {:?} at {:?}", entity, transform.translation);
    }
}

// WASM 特定调试
#[cfg(target_arch = "wasm32")]
fn wasm_debug(message: &str) {
    web_sys::console::log_1(&message.into());
}
```

### 性能分析

```bash
# 本地性能分析
cargo install cargo-flamegraph
cargo flamegraph --bin tower-tumbler

# WASM 性能分析
trunk build --release
# 在浏览器开发者工具中使用 Performance 标签
```

## 📚 相关资源

### 官方文档
- [Bevy 0.14 发布说明](https://bevyengine.org/news/bevy-0-14-0/)
- [Bevy 迁移指南](https://bevyengine.org/learn/migration-guides/)
- [Bevy WASM 示例](https://github.com/bevyengine/bevy/tree/main/examples/platform_specific/wasm)

### 社区资源
- [Bevy Assets](https://bevyengine.org/assets/) - 社区插件和资源
- [Bevy Discord](https://discord.gg/bevy) - 实时技术支持
- [awesome-bevy](https://github.com/bevyengine/awesome-bevy) - 优秀资源汇总

### 工具推荐
- [bevy_inspector_egui](https://github.com/jakobhellermann/bevy_inspector_egui) - 运行时检查器
- [bevy_prototype_debug_lines](https://github.com/Toqozz/bevy_prototype_debug_lines) - 调试绘制
- [bevy_tweening](https://github.com/djeedai/bevy_tweening) - 补间动画

这份兼容性指南应该涵盖了使用 Bevy 0.14 开发 Tower Tumbler 项目的所有重要方面。记住在遇到问题时查阅最新的官方文档和社区资源！