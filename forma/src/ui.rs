use std::collections::VecDeque;

use egui::{Align2, Context, Key, Modifiers, RichText, ScrollArea, TextEdit, TextStyle, Vec2};

pub struct Ui {
    command: String,
    command_history: VecDeque<String>,
    show_command_console: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            command: String::new(),
            command_history: VecDeque::new(),
            show_command_console: false,
        }
    }

    pub fn ui(&mut self, ctx: &Context) {
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
