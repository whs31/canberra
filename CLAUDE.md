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

Not a full ECS. `Entity` holds a `HashMap<TypeId, Box<dyn Component>>`. Components are retrieved by concrete type via `get_component::<T>()` / `get_component_mut::<T>()`. No query system or archetype optimization.

Built-in components in `canberra_engine/src/components/`:
- `Transform` — position (Vec3), rotation (Quat), scale (Vec3); provides `matrix() -> Mat4`
- `Camera` — perspective projection (fov, aspect, near/far)
- `Mesh` — `Vec<Vertex>` + `Vec<u16>` indices; ships a built-in cube
- `Material` — RGBA color `[f32; 4]`

### Renderer (`canberra_engine/src/renderer/`)

Single wgpu render pipeline using `shader.wgsl` (WGSL). Per-frame flow:
1. Write camera uniform (view-projection from first `Camera` entity in scene)
2. Collect renderable entities (must have both `Mesh` and `Transform`)
3. Pack per-object data (model matrix + color) into a dynamic uniform buffer
4. Issue draw calls with dynamic offsets
5. Meshes are uploaded to GPU on first render and cached by entity UUID

Depth testing is enabled; the depth texture is recreated on resize.

### Editor UI (`canberra_engine/src/editor/`)

Egui is integrated directly into the render loop (not a separate thread). `inspector.rs` renders a left panel with a selectable entity list and their component names.

### Key Dependencies

| Crate | Version | Role |
|---|---|---|
| wgpu | 29 | cross-platform GPU (WebGPU API) |
| egui / egui-wgpu / egui-winit | 0.34 | immediate-mode UI |
| winit | 0.30 | windowing and event loop |
| glam | latest | math (Vec3, Quat, Mat4) |
| bytemuck | latest | safe GPU buffer casting |
| tracing | latest | structured logging |
