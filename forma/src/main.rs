use std::error::Error;

use egui::ViewportId;
use render::Renderer;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::{Window, WindowId};

struct App {
    ctx: egui::Context,
    egui_winit_state: egui_winit::State,
    egui_renderer: Option<egui_wgpu::Renderer>,
    renderer: Option<Renderer>,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let ctx = egui::Context::default();

        let egui_winit_state = egui_winit::State::new(
            ctx.clone(),
            ViewportId::ROOT,
            &event_loop.display_handle().unwrap(),
            None,
            None,
            None,
        );

        Self {
            ctx,
            egui_winit_state,
            egui_renderer: None,
            renderer: None,
        }
    }

    pub fn paint(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        let Some(egui_renderer) = &mut self.egui_renderer else {
            return;
        };

        let raw_input: egui::RawInput = self.egui_winit_state.take_egui_input(renderer.window());

        let full_output = self.ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Hello world!");
                if ui.button("Click me").clicked() {
                    println!("clicked");
                }
            });
        });

        self.egui_winit_state
            .handle_platform_output(renderer.window(), full_output.platform_output);

        let clipped_primitives = self
            .ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, image_delta) in full_output.textures_delta.set {
            egui_renderer.update_texture(renderer.device(), renderer.queue(), id, &image_delta);
        }

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: renderer.window().inner_size().into(),
            pixels_per_point: renderer.window().scale_factor() as f32,
        };

        renderer.render(|renderer, encoder, rp| {
            egui_renderer.update_buffers(
                renderer.device(),
                renderer.queue(),
                encoder,
                &clipped_primitives,
                &screen_descriptor,
            );
            egui_renderer.render(rp, &clipped_primitives, &screen_descriptor);
        });

        for id in full_output.textures_delta.free {
            egui_renderer.free_texture(&id);
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Forma")
            .with_inner_size(PhysicalSize::new(1280, 720));

        let window = event_loop.create_window(window_attributes).unwrap();
        let renderer = Renderer::new(window);

        let egui_renderer = egui_wgpu::Renderer::new(
            renderer.device(),
            renderer.surface_config().format,
            egui_wgpu::RendererOptions::default(),
        );

        self.renderer = Some(renderer);
        self.egui_renderer = Some(egui_renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        event_loop.set_control_flow(ControlFlow::Wait);

        if let Some(renderer) = &mut self.renderer {
            let _ = self
                .egui_winit_state
                .on_window_event(renderer.window(), &event);
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
            }
            WindowEvent::RedrawRequested => {
                self.paint();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(renderer) = &mut self.renderer {
            renderer.window().request_redraw();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app)?;

    Ok(())
}
