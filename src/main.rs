use std::time::{Duration, Instant};

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::{Texture, TextureCreator, WindowCanvas};
use sdl3::surface::Surface;
use sdl3::video::WindowContext;

const VERSION: &str = "0.3.0";
const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const TICK_INTERVAL: Duration = Duration::from_millis(20);
const SPEED_FACTOR: f32 = 0.5;

fn load_texture<'a>(
    creator: &'a TextureCreator<WindowContext>,
    path: &str,
    color_key: bool,
) -> Result<Texture<'a>, String> {
    let surface = Surface::load_bmp(path).map_err(|e| e.to_string())?;
    // ponytail: the BMPs are 4-bit palettized and the crate maps color keys with a null
    // palette, which silently picks the wrong index — convert to direct color first
    let mut surface = surface
        .convert_format(sdl3::pixels::PixelFormat::RGBA8888)
        .map_err(|e| e.to_string())?;
    if color_key {
        surface.set_color_key(true, Color::RGB(0xFF, 0x00, 0xFF)).map_err(|e| e.to_string())?;
    }
    let mut texture = creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
    // pixel art: nearest-neighbor scaling, or linear filtering bleeds the (transparent)
    // magenta key color into sprite edges when fullscreen upscales
    texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    Ok(texture)
}

fn draw_plane(canvas: &mut WindowCanvas, texture: &Texture, time: i32, scale: f32, y: i32) {
    let w = texture.width() as i32;
    let offset = ((time as f32 * scale * SPEED_FACTOR) as i32).rem_euclid(w);

    for x in [offset - w, offset] {
        let _ = canvas.copy(
            texture,
            None,
            Rect::new(x, y, texture.width(), texture.height()),
        );
    }
}

fn draw_sky(canvas: &mut WindowCanvas) {
    let bands: [(u8, u32); 10] = [
        (99, 76),
        (115, 27),
        (132, 14),
        (148, 10),
        (165, 8),
        (181, 7),
        (198, 6),
        (214, 6),
        (231, 4),
        (247, 6),
    ];

    let mut y = 0;
    for (red, height) in bands {
        canvas.set_draw_color(Color::RGB(red, 113, 132));
        let _ = canvas.fill_rect(Rect::new(0, y, WIDTH, height));
        y += height as i32;
    }
}

fn main() -> Result<(), String> {
    let sdl = sdl3::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;

    let window = video
        .window("Shadow of the Blitz", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();
    // ponytail: logical presentation replaces SDL 1.2's real 320x240 video mode; fullscreen scales
    canvas
        .set_logical_size(WIDTH, HEIGHT, sdl3::sys::render::SDL_RendererLogicalPresentation::LETTERBOX)
        .map_err(|e| e.to_string())?;

    println!("Shadow of the Blitz {VERSION}");
    println!("http://www.glop.org/software/sotb");
    println!();
    let (w, h) = canvas.output_size().map_err(|e| e.to_string())?;
    println!("Resolution used: {w}x{h}");

    let creator = canvas.texture_creator();

    let herbe0 = load_texture(&creator, "data/herbe0.bmp", false)?;
    let herbe1 = load_texture(&creator, "data/herbe1.bmp", false)?;
    let herbe2 = load_texture(&creator, "data/herbe2.bmp", false)?;
    let herbe3 = load_texture(&creator, "data/herbe3.bmp", false)?;
    let herbe4 = load_texture(&creator, "data/herbe4.bmp", false)?;
    let nuages0 = load_texture(&creator, "data/nuages0.bmp", true)?;
    let nuages1 = load_texture(&creator, "data/nuages1.bmp", true)?;
    let nuages2 = load_texture(&creator, "data/nuages2.bmp", true)?;
    let nuages3 = load_texture(&creator, "data/nuages3.bmp", true)?;
    let nuages4 = load_texture(&creator, "data/nuages4.bmp", true)?;
    let barriere = load_texture(&creator, "data/barriere.bmp", true)?;
    let montagnes = load_texture(&creator, "data/montagnes.bmp", true)?;
    let lune = load_texture(&creator, "data/lune.bmp", true)?;

    let mut events = sdl.event_pump().map_err(|e| e.to_string())?;
    let mut scroll: i32 = 0;
    let mut next_tick = Instant::now();

    'running: loop {
        draw_sky(&mut canvas);

        let _ = canvas.copy(&lune, None, Rect::new(184, 16, lune.width(), lune.height()));

        draw_plane(&mut canvas, &montagnes, scroll, 1.0, 97);
        draw_plane(&mut canvas, &herbe0, scroll, 2.0, 170);
        draw_plane(&mut canvas, &herbe1, scroll, 3.0, 172);
        draw_plane(&mut canvas, &herbe2, scroll, 4.0, 175);
        draw_plane(&mut canvas, &herbe3, scroll, 5.0, 182);
        draw_plane(&mut canvas, &herbe4, scroll, 6.0, 189);
        draw_plane(&mut canvas, &barriere, scroll, 7.0, 179);

        draw_plane(&mut canvas, &nuages0, scroll, 2.0, 0);
        draw_plane(&mut canvas, &nuages1, scroll, 1.0, 22);
        draw_plane(&mut canvas, &nuages2, scroll, 1.0 / 2.0, 63);
        draw_plane(&mut canvas, &nuages3, scroll, 1.0 / 3.0, 82);
        draw_plane(&mut canvas, &nuages4, scroll, 1.0 / 4.0, 91);

        canvas.present();

        // port of TimeLeft(): pace the scroll at one tick per 20 ms
        let now = Instant::now();
        if next_tick > now {
            std::thread::sleep(next_tick - now);
        }
        next_tick = Instant::now() + TICK_INTERVAL;

        scroll += 1;

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Got quit event, exiting.");
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("ESC key hit, exiting.");
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    println!("switching fullscreen");
                    let window = canvas.window_mut();
                    let off = window.fullscreen_state() == sdl3::video::FullscreenType::Off;
                    window.set_fullscreen(off).map_err(|e| e.to_string())?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
