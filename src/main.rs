//! A very simple shader example.

#[macro_use]
extern crate gfx;
extern crate ggez;

use ggez::{Context, GameResult};
use ggez::event;
use ggez::timer;
use ggez::conf;
use ggez::graphics::{self, DrawMode, Point2, DrawParam};
use std::env;
use std::path;

gfx_defines!{
    constant Dim {
        rate: f32 = "u_Rate",
    }
}

struct MainState {
    font: graphics::Font,
    dim: Dim,
    shader: graphics::Shader<Dim>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 24)?;

        let dim = Dim { rate: 0.5 };
        let shader = graphics::Shader::new(
            ctx,
            "/basic_150.glslv",
            "/dimmer_150.glslf",
            dim,
            "Dim",
            None,
        )?;
        Ok(MainState { font, dim, shader })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const PHYSICS_FPS: u32 = 60;

        while timer::check_update_time(ctx, PHYSICS_FPS) {
            // Physics ticks
        }

        self.dim.rate = 0.5 + (((timer::get_ticks(ctx) as f32) / 100.0).cos() / 2.0);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let dest = graphics::Point2::new(10.0, 10.0);
        let scale = graphics::Point2::new(0.5, 0.5);

        let text = format!("Ticks: {}", timer::get_ticks(ctx));
        let text = graphics::Text::new(ctx, &text, &self.font)?;
        graphics::draw_ex(ctx, &text, DrawParam { dest, scale, .. Default::default() })?;

        let dest = graphics::Point2::new(dest.x, dest.y + (self.font.get_height() as f32) * scale.y);
        let text = format!("FPS: {:.2}", timer::get_fps(ctx));
        let text = graphics::Text::new(ctx, &text, &self.font)?;
        graphics::draw_ex(ctx, &text, DrawParam { dest, scale, .. Default::default() })?;

        graphics::circle(ctx, DrawMode::Fill, Point2::new(100.0, 300.0), 100.0, 2.0)?;

        {
            let _lock = graphics::use_shader(ctx, &self.shader);
            self.shader.send(ctx, self.dim)?;
            graphics::circle(ctx, DrawMode::Fill, Point2::new(400.0, 300.0), 200.0, 0.1)?;
        }

        graphics::circle(ctx, DrawMode::Fill, Point2::new(700.0, 300.0), 100.0, 2.0)?;

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() -> Result<(), Box<std::error::Error>> {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("tiles", "mag", c)?;

    println!("Running with DPI: {:?}", ctx.sdl_context.video()?.display_dpi(0)?);

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, state)?;

    Ok(())
}
