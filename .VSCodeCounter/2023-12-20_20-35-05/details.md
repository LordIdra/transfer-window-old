# Details

Date : 2023-12-20 20:35:05

Directory /home/idra/GitHub/transfer-window

Total : 81 files,  7109 codes, 198 comments, 1066 blanks, all 8373 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [Cargo.lock](/Cargo.lock) | TOML | 3,082 | 2 | 350 | 3,434 |
| [Cargo.toml](/Cargo.toml) | TOML | 18 | 0 | 4 | 22 |
| [integrator.py](/integrator.py) | Python | 70 | 0 | 19 | 89 |
| [plan.md](/plan.md) | Markdown | 83 | 0 | 6 | 89 |
| [profile.json](/profile.json) | JSON | 1 | 0 | 0 | 1 |
| [resources/shaders/geometry.frag](/resources/shaders/geometry.frag) | OpenGL Shading Language | 5 | 0 | 2 | 7 |
| [resources/shaders/geometry.vert](/resources/shaders/geometry.vert) | OpenGL Shading Language | 15 | 0 | 3 | 18 |
| [resources/shaders/icon.frag](/resources/shaders/icon.frag) | OpenGL Shading Language | 9 | 0 | 3 | 12 |
| [resources/shaders/icon.vert](/resources/shaders/icon.vert) | OpenGL Shading Language | 18 | 0 | 3 | 21 |
| [src/camera.rs](/src/camera.rs) | Rust | 64 | 0 | 15 | 79 |
| [src/components.rs](/src/components.rs) | Rust | 47 | 0 | 5 | 52 |
| [src/components/celestial_body_component.rs](/src/components/celestial_body_component.rs) | Rust | 29 | 0 | 9 | 38 |
| [src/components/icon_component.rs](/src/components/icon_component.rs) | Rust | 94 | 0 | 18 | 112 |
| [src/components/mass_component.rs](/src/components/mass_component.rs) | Rust | 11 | 0 | 2 | 13 |
| [src/components/name_component.rs](/src/components/name_component.rs) | Rust | 11 | 0 | 2 | 13 |
| [src/components/parent_component.rs](/src/components/parent_component.rs) | Rust | 15 | 0 | 4 | 19 |
| [src/components/position_component.rs](/src/components/position_component.rs) | Rust | 15 | 0 | 4 | 19 |
| [src/components/trajectory_component.rs](/src/components/trajectory_component.rs) | Rust | 67 | 1 | 14 | 82 |
| [src/components/trajectory_component/segment.rs](/src/components/trajectory_component/segment.rs) | Rust | 97 | 0 | 18 | 115 |
| [src/components/trajectory_component/segment/burn.rs](/src/components/trajectory_component/segment/burn.rs) | Rust | 81 | 0 | 20 | 101 |
| [src/components/trajectory_component/segment/burn/burn_point.rs](/src/components/trajectory_component/segment/burn/burn_point.rs) | Rust | 39 | 0 | 7 | 46 |
| [src/components/trajectory_component/segment/orbit.rs](/src/components/trajectory_component/segment/orbit.rs) | Rust | 128 | 1 | 34 | 163 |
| [src/components/trajectory_component/segment/orbit/conic.rs](/src/components/trajectory_component/segment/orbit/conic.rs) | Rust | 135 | 9 | 24 | 168 |
| [src/components/trajectory_component/segment/orbit/conic/ellipse.rs](/src/components/trajectory_component/segment/orbit/conic/ellipse.rs) | Rust | 281 | 21 | 44 | 346 |
| [src/components/trajectory_component/segment/orbit/conic/hyperbola.rs](/src/components/trajectory_component/segment/orbit/conic/hyperbola.rs) | Rust | 350 | 4 | 48 | 402 |
| [src/components/trajectory_component/segment/orbit/orbit_direction.rs](/src/components/trajectory_component/segment/orbit/orbit_direction.rs) | Rust | 29 | 1 | 6 | 36 |
| [src/components/trajectory_component/segment/orbit/orbit_point.rs](/src/components/trajectory_component/segment/orbit/orbit_point.rs) | Rust | 45 | 0 | 11 | 56 |
| [src/components/velocity_component.rs](/src/components/velocity_component.rs) | Rust | 15 | 0 | 4 | 19 |
| [src/main.rs](/src/main.rs) | Rust | 21 | 0 | 5 | 26 |
| [src/rendering.rs](/src/rendering.rs) | Rust | 5 | 0 | 0 | 5 |
| [src/rendering/geometry_renderer.rs](/src/rendering/geometry_renderer.rs) | Rust | 31 | 0 | 9 | 40 |
| [src/rendering/shader_program.rs](/src/rendering/shader_program.rs) | Rust | 64 | 0 | 13 | 77 |
| [src/rendering/texture.rs](/src/rendering/texture.rs) | Rust | 25 | 0 | 4 | 29 |
| [src/rendering/texture_renderer.rs](/src/rendering/texture_renderer.rs) | Rust | 38 | 0 | 10 | 48 |
| [src/rendering/vertex_array_object.rs](/src/rendering/vertex_array_object.rs) | Rust | 70 | 0 | 12 | 82 |
| [src/resources.rs](/src/resources.rs) | Rust | 61 | 0 | 11 | 72 |
| [src/state.rs](/src/state.rs) | Rust | 130 | 3 | 11 | 144 |
| [src/storage.rs](/src/storage.rs) | Rust | 3 | 0 | 0 | 3 |
| [src/storage/entity_allocator.rs](/src/storage/entity_allocator.rs) | Rust | 59 | 0 | 10 | 69 |
| [src/storage/entity_builder.rs](/src/storage/entity_builder.rs) | Rust | 149 | 0 | 22 | 171 |
| [src/storage/index_storage.rs](/src/storage/index_storage.rs) | Rust | 59 | 0 | 8 | 67 |
| [src/systems.rs](/src/systems.rs) | Rust | 14 | 0 | 0 | 14 |
| [src/systems/camera_update_system.rs](/src/systems/camera_update_system.rs) | Rust | 45 | 3 | 8 | 56 |
| [src/systems/debug_system.rs](/src/systems/debug_system.rs) | Rust | 19 | 0 | 5 | 24 |
| [src/systems/debug_system/general.rs](/src/systems/debug_system/general.rs) | Rust | 5 | 0 | 2 | 7 |
| [src/systems/debug_system/selected.rs](/src/systems/debug_system/selected.rs) | Rust | 119 | 0 | 10 | 129 |
| [src/systems/delta_time_update_system.rs](/src/systems/delta_time_update_system.rs) | Rust | 7 | 0 | 2 | 9 |
| [src/systems/deselect_system.rs](/src/systems/deselect_system.rs) | Rust | 13 | 0 | 2 | 15 |
| [src/systems/icon_system.rs](/src/systems/icon_system.rs) | Rust | 13 | 0 | 4 | 17 |
| [src/systems/icon_system/burn_icon_cleanup.rs](/src/systems/icon_system/burn_icon_cleanup.rs) | Rust | 20 | 0 | 4 | 24 |
| [src/systems/icon_system/icon_click.rs](/src/systems/icon_system/icon_click.rs) | Rust | 68 | 1 | 10 | 79 |
| [src/systems/icon_system/icon_click/burn_arrow_icon.rs](/src/systems/icon_system/icon_click/burn_arrow_icon.rs) | Rust | 17 | 2 | 5 | 24 |
| [src/systems/icon_system/icon_click/burn_icon.rs](/src/systems/icon_system/icon_click/burn_icon.rs) | Rust | 36 | 3 | 9 | 48 |
| [src/systems/icon_system/icon_click/object_icon.rs](/src/systems/icon_system/icon_click/object_icon.rs) | Rust | 24 | 4 | 6 | 34 |
| [src/systems/icon_system/icon_position.rs](/src/systems/icon_system/icon_position.rs) | Rust | 16 | 0 | 4 | 20 |
| [src/systems/icon_system/icon_position/burn_arrow_icon.rs](/src/systems/icon_system/icon_position/burn_arrow_icon.rs) | Rust | 9 | 0 | 2 | 11 |
| [src/systems/icon_system/icon_position/burn_icon.rs](/src/systems/icon_system/icon_position/burn_icon.rs) | Rust | 8 | 0 | 2 | 10 |
| [src/systems/icon_system/icon_position/object_icon.rs](/src/systems/icon_system/icon_position/object_icon.rs) | Rust | 6 | 0 | 1 | 7 |
| [src/systems/icon_system/icon_precedence.rs](/src/systems/icon_system/icon_precedence.rs) | Rust | 58 | 6 | 8 | 72 |
| [src/systems/mouse_over_any_element_system.rs](/src/systems/mouse_over_any_element_system.rs) | Rust | 6 | 0 | 1 | 7 |
| [src/systems/orbit_point_selection_system.rs](/src/systems/orbit_point_selection_system.rs) | Rust | 150 | 0 | 17 | 167 |
| [src/systems/time_step_update_system.rs](/src/systems/time_step_update_system.rs) | Rust | 49 | 0 | 10 | 59 |
| [src/systems/toolbar_system.rs](/src/systems/toolbar_system.rs) | Rust | 24 | 0 | 6 | 30 |
| [src/systems/toolbar_system/burn_toolbar.rs](/src/systems/toolbar_system/burn_toolbar.rs) | Rust | 49 | 0 | 14 | 63 |
| [src/systems/toolbar_system/orbit_click_point_toolbar.rs](/src/systems/toolbar_system/orbit_click_point_toolbar.rs) | Rust | 60 | 0 | 16 | 76 |
| [src/systems/trajectory_prediction_system.rs](/src/systems/trajectory_prediction_system.rs) | Rust | 3 | 0 | 0 | 3 |
| [src/systems/trajectory_prediction_system/celestial_body_prediction.rs](/src/systems/trajectory_prediction_system/celestial_body_prediction.rs) | Rust | 52 | 8 | 9 | 69 |
| [src/systems/trajectory_prediction_system/spacecraft_prediction.rs](/src/systems/trajectory_prediction_system/spacecraft_prediction.rs) | Rust | 69 | 1 | 13 | 83 |
| [src/systems/trajectory_prediction_system/util.rs](/src/systems/trajectory_prediction_system/util.rs) | Rust | 64 | 10 | 11 | 85 |
| [src/systems/trajectory_update_system.rs](/src/systems/trajectory_update_system.rs) | Rust | 11 | 0 | 2 | 13 |
| [src/systems/underlay_render_system.rs](/src/systems/underlay_render_system.rs) | Rust | 39 | 0 | 7 | 46 |
| [src/systems/underlay_render_system/render_icons.rs](/src/systems/underlay_render_system/render_icons.rs) | Rust | 28 | 0 | 4 | 32 |
| [src/systems/underlay_render_system/render_object.rs](/src/systems/underlay_render_system/render_object.rs) | Rust | 30 | 0 | 5 | 35 |
| [src/systems/underlay_render_system/render_segment.rs](/src/systems/underlay_render_system/render_segment.rs) | Rust | 23 | 0 | 4 | 27 |
| [src/systems/underlay_render_system/render_segment/render_burn.rs](/src/systems/underlay_render_system/render_segment/render_burn.rs) | Rust | 40 | 1 | 6 | 47 |
| [src/systems/underlay_render_system/render_segment/render_orbit.rs](/src/systems/underlay_render_system/render_segment/render_orbit.rs) | Rust | 72 | 91 | 15 | 178 |
| [src/systems/underlay_render_system/render_segment/util.rs](/src/systems/underlay_render_system/render_segment/util.rs) | Rust | 25 | 1 | 9 | 35 |
| [src/systems/underlay_render_system/render_segment/visual_segment_point.rs](/src/systems/underlay_render_system/render_segment/visual_segment_point.rs) | Rust | 32 | 0 | 7 | 39 |
| [src/systems/util.rs](/src/systems/util.rs) | Rust | 119 | 21 | 15 | 155 |
| [src/systems/warp_update_system.rs](/src/systems/warp_update_system.rs) | Rust | 54 | 4 | 10 | 68 |
| [src/util.rs](/src/util.rs) | Rust | 44 | 0 | 7 | 51 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)