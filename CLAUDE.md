# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
cargo build                # debug build
cargo build --release      # release build (LTO enabled)
cargo run                  # run the app (canberra_bin)
cargo clippy               # lint
cargo fmt                  # format
cargo fmt --check          # format check only
```

No tests exist yet. 

## Architecture Overview

**Canberra** is a Rust 3D map/rendering engine with an integrated editor UI. It is a Cargo workspace with two crates:

- `canberra_engine/` — core framework: windowing, rendering, ECS-lite, editor UI
- `canberra_app/` (`canberra_bin`) — thin binary that constructs a scene and runs it

### Application Lifecycle

Entry point in `canberra_app/` calls `Application::run()` with a scene-builder closure. `Application` implements `winit::application::ApplicationHandler`. On first resume, `ApplicationState::new()` initializes wgpu, the surface, the renderer, and egui. Each frame: handle events → acquire surface → render 3D → overlay egui → present.

**Drop order in `ApplicationState` is intentional** — reordering fields breaks Wayland. See the comment at the top of `application/state.rs`.

### Component/Entity System (`canberra_engine/src/hierarchy/`)

Not a full ECS. `Entity` holds a `HashMap<TypeId, Box<dyn Component>>` plus a list of child `Entity`s (scene graph). Components are retrieved by concrete type via `get_component::<T>()` / `get_component_mut::<T>()`. No query system or archetype optimization.

`Component` trait (serialized via `typetag`):
- `name() -> &'static str` — display name
- `as_any()` / `as_any_mut()` — downcasting support
- `inspect(&mut self, ui: &mut egui::Ui)` — default impl just shows the name; components can override for property editing

Built-in components in `canberra_engine/src/components/`:
- `Transform` — position (Vec3), rotation (Quat), scale (Vec3); provides `matrix() -> Mat4`
- `Camera` — perspective projection (fov, aspect, near/far)
- `Mesh` — `Vec<Vertex>` + `Vec<u16>` indices; ships a built-in cube
- `Material` — RGBA color `[f32; 4]` + optional `ShaderHandle` for custom shaders

Scene is a flat `Vec<Entity>` (`Scene.entities`). `Scene::camera_view_proj(aspect)` iterates the first entity with a `Camera` component to produce the view-projection matrix.

**ECS gaps** — compared to a full ECS (e.g. `specs`, `bevy_ecs`):
- **No query system** — no `query::<&Transform>()` or `query::<(&Transform, &Material)>()` returning typed iterators
- **No archetype/chunked storage** — components are scattered across `HashMap` buckets; no cache-friendly iteration
- **No component relations** — no read/write conflict tracking, no `&T` / `&mut T` borrow semantics
- **No resource storage** — no separate world-level singleton pool for non-entity data (time, config, game state)
- **No component groups/filters** — no "all entities with A but not B" queries
- **No component lifecycle** — no `on_add`, `on_remove`, `on_enable` hooks
- **No multi-threaded system execution** — all rendering and update runs on the main thread
- **No world/entity storage management** — no `World` type managing the entity store, archetype compilation, or deferred removals

### Renderer (`canberra_engine/src/renderer/`)

Multi-pipeline wgpu render. Per-frame flow:
1. Write camera uniform (view-projection from first `Camera` entity in scene)
2. Collect renderable entities (must have both `Mesh` and `Transform`) via recursive tree traversal
3. Pack per-object data (model matrix + color) into a dynamic uniform buffer (max 256 objects, 256-byte stride each)
4. Issue draw calls with dynamic offsets into the object bind group
5. Meshes are uploaded to GPU on first render and cached by entity UUID hash

**Shader registry** (`ShaderRegistry`): global `ArcSwap<ShaderRegistry>` stores WGSL source strings. Default lit and unlit shaders ship with the engine (`shader_lit.wgsl`, `shader_unlit.wgsl`). Apps can register custom shaders (e.g. `wobble.wgsl`, `bloom.wgsl`) at startup. Each material selects its shader via `ShaderHandle`.

Depth testing is enabled; the depth texture is recreated on resize.

**Key types:**
- `Renderer` — single renderer instance with pipelines, camera/object bind groups, depth texture
- `AssetManager` — meshes cached by `MeshHandle` (content hash); uploads to GPU on first use
- `CameraUniform` — `view_proj: mat4x4<f32>` + `time: f32` (32-byte padding)
- `ObjectUniformData` — `model: mat4x4<f32>` + `color: vec4<f32>`

### Editor UI (`canberra_engine/src/editor/`)

Egui is integrated directly into the render loop (not a separate thread). Two separate egui windows:
- **Hierarchy** (`hierarchy.rs`) — tree view of the scene graph with selectable entities (collapsing headers for parents, single rows for leaves)
- **Inspector** (`inspector.rs`) — property editor for the selected entity, showing each component in a `CollapsingHeader` that calls `component.inspect(ui)`

Both windows are resizable with default minimum sizes (200px for hierarchy, 260px for inspector).

### Key Dependencies

| Crate | Version | Role |
|---|---|---|
| wgpu | 29 | cross-platform GPU (WebGPU API) |
| egui / egui-wgpu / egui-winit | 0.34 | immediate-mode UI |
| winit | 0.30 | windowing and event loop |
| glam | 0.32 | math (Vec3, Quat, Mat4) |
| bytemuck | 1 | safe GPU buffer casting |
| tracing / tracing-subscriber | 0.1 / 0.3 | structured logging |
| serde | 1 (+derive) | serialization (used by typetag, postcard) |
| postcard | 1 | binary scene serialization |
| typetag | 0.2 | dynamic component trait serialization |
| uuid | 1 (+serde) | entity IDs |
| arc-swap | 1 | thread-safe shader registry |
| tokio | 1 (+full) | async runtime (unused in practice) |
| pollster | 0.4 | blocking async in `Application::run` |
| thiserror | 2 | error types |
| num-traits | 0.2 | numeric bounds |
