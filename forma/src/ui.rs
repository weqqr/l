use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use anyhow::{Result, anyhow};
use egui::epaint::CircleShape;
use egui::{
    Align2, Area, Color32, Context, Frame, Id, Key, LayerId, Margin, Modifiers, Popup, PopupKind,
    RichText, ScrollArea, Sense, Shape, TextEdit, TextStyle, UiBuilder, Vec2,
};
use egui_tiles::{Behavior, Container, ContainerKind, SimplificationOptions, Tile, Tree};
use render::VoxelRenderer;
use uuid::Uuid;

use crate::world_manager::WorldManager;

pub struct View {
    command_text: String,
    show_command_console: bool,

    controller: Controller,
    tree: Tree<Pane>,
    tree_controller: TreeController,
}

impl View {
    pub fn new(controller: Controller) -> Self {
        let tree_controller = TreeController {
            world_manager: Arc::clone(&controller.world_manager),
        };

        Self {
            command_text: String::new(),
            show_command_console: false,
            controller,
            tree: Tree::new_tabs(Uuid::new_v4().to_string(), vec![]),
            tree_controller,
        }
    }

    pub fn ui(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open world...").clicked() {
                        if let Ok(world_id) = self.controller.open_world() {
                            self.insert_pane(Pane::World(world_id));
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default()
            .frame(Frame {
                outer_margin: Margin::ZERO,
                fill: ctx.style().visuals.panel_fill,
                ..Default::default()
            })
            .show(ctx, |ui| {
                self.tree.ui(&mut self.tree_controller, ui);
            });

        egui::Window::new("command console")
            .title_bar(false)
            .open(&mut self.show_command_console)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                let edit_response = TextEdit::singleline(&mut self.command_text)
                    .hint_text("Enter a WorldEdit command")
                    .font(TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .show(ui);
                edit_response.response.request_focus();
            });

        if self.show_command_console {
            ctx.input_mut(|input| {
                if input.consume_key(Modifiers::NONE, Key::Escape) {
                    self.show_command_console = false;
                    self.command_text.clear();
                }
            });

            ctx.input_mut(|input| {
                if input.consume_key(Modifiers::NONE, Key::Enter) {
                    self.controller.execute_command(self.command_text.clone());
                    self.show_command_console = false;
                    self.command_text.clear();
                }
            });
        } else {
            ctx.input_mut(|input| {
                if input.consume_key(Modifiers::NONE, Key::Slash) {
                    self.show_command_console = true;
                }
            });
        }
    }

    fn insert_pane(&mut self, pane: Pane) {
        let new_tile_id = self.tree.tiles.insert_pane(pane);

        let tile_id = self
            .tree
            .tiles
            .tile_ids()
            .find(|id| {
                self.tree.tiles.get(*id).unwrap().container_kind() == Some(ContainerKind::Tabs)
            })
            .unwrap_or(self.tree.tiles.tile_ids().nth(0).unwrap());

        if let Some(Tile::Container(Container::Tabs(tabs))) = self.tree.tiles.get_mut(tile_id) {
            tabs.add_child(new_tile_id);
        } else {
            let tabs_id = self.tree.tiles.insert_tab_tile(vec![new_tile_id]);

            self.tree.root = Some(tabs_id);
        }
    }
}

enum Pane {
    World(Uuid),
}

struct TreeController {
    world_manager: Arc<Mutex<WorldManager>>,
}

impl Behavior<Pane> for TreeController {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::World(id) => {
                let rect = ui.available_rect_before_wrap();
                ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                    let (response, painter) =
                        ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

                    painter.add(egui_wgpu::Callback::new_paint_callback(
                        response.rect,
                        WorldViewCallback::new(*id),
                    ));

                    if response.secondary_clicked() {
                        println!("right click");
                    }
                });
                ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                    ui.label(RichText::new("Cursor: X=0 Y=0 Z=0").color(Color32::BLACK));
                });
            }
        }
        Default::default()
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::World(id) => self
                .world_manager
                .lock()
                .unwrap()
                .world_by_id(*id)
                .map(|world| world.name.as_str().into())
                .unwrap_or("unknown".into()),
        }
    }

    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions {
            prune_empty_tabs: true,
            prune_empty_containers: true,
            prune_single_child_tabs: true,
            prune_single_child_containers: true,
            all_panes_must_have_tabs: true,
            join_nested_linear_containers: true,
        }
    }
}

pub struct Controller {
    command_history: VecDeque<String>,

    world_manager: Arc<Mutex<WorldManager>>,
}

impl Controller {
    pub fn new(world_manager: Arc<Mutex<WorldManager>>) -> Self {
        Self {
            command_history: VecDeque::new(),
            world_manager,
        }
    }

    pub fn open_world(&self) -> Result<Uuid> {
        let path = rfd::FileDialog::new()
            .pick_folder()
            .ok_or(anyhow!("canceled"))?;
        let id = self.world_manager.lock().unwrap().open(path)?;
        Ok(id)
    }

    pub fn execute_command(&mut self, command: String) {
        println!("command: {command}");

        self.command_history.push_front(command.clone());
        if self.command_history.len() > 100 {
            self.command_history.drain(100..);
        }
    }
}

struct WorldViewCallback {
    world_id: Uuid,
}

impl WorldViewCallback {
    pub fn new(world_id: Uuid) -> Self {
        Self { world_id }
    }
}

impl egui_wgpu::CallbackTrait for WorldViewCallback {
    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut eframe::wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        let voxel_renderer = callback_resources.get::<VoxelRenderer>().unwrap();
        voxel_renderer.render(render_pass);
    }
}
