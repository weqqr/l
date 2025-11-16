use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use egui::{Align2, Context, Key, Modifiers, RichText, ScrollArea, TextEdit, TextStyle, Vec2};

use crate::world_manager::WorldManager;

pub struct Ui {
    command: String,
    command_history: VecDeque<String>,
    show_command_console: bool,

    world_manager: Arc<Mutex<WorldManager>>,
}

impl Ui {
    pub fn new(world_manager: Arc<Mutex<WorldManager>>) -> Self {
        Self {
            command: String::new(),
            command_history: VecDeque::new(),
            show_command_console: false,
            world_manager,
        }
    }

    pub fn ui(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open world...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            println!("open: {:?}", path);
                            self.world_manager.lock().unwrap().open("aboba");
                        }
                    }
                });
            });
        });

        egui::Window::new("command console")
            .title_bar(false)
            .open(&mut self.show_command_console)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                let edit_response = TextEdit::singleline(&mut self.command)
                    .hint_text("Enter a WorldEdit command")
                    .font(TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .show(ui);
                edit_response.response.request_focus();

                ScrollArea::new([false, true])
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        for command in &self.command_history {
                            ui.label(RichText::new(command).monospace());
                        }
                    });
            });

        if self.show_command_console {
            ctx.input_mut(|input| {
                if input.consume_key(Modifiers::NONE, Key::Escape) {
                    self.show_command_console = false;
                    self.command.clear();
                }
            });

            ctx.input_mut(|input| {
                if input.consume_key(Modifiers::NONE, Key::Enter) {
                    self.show_command_console = false;
                    println!("command: {}", self.command);
                    self.command_history.push_front(self.command.clone());
                    if self.command_history.len() > 100 {
                        self.command_history.drain(100..);
                    }
                    self.command.clear();
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
}
