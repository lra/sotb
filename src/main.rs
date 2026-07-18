use std::time::{Duration, Instant};

use sdl3::event::Event;
use sdl3::iostream::IOStream;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::{Texture, TextureCreator, WindowCanvas};
use sdl3::surface::Surface;
use sdl3::video::WindowContext;
use sdl3::EventPump;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const TICK_INTERVAL: Duration = Duration::from_millis(20);
const SPEED_FACTOR: f32 = 0.5;

// Assets are compiled into the binary so releases are a single file.
const HERBE0: &[u8] = include_bytes!("../data/herbe0.bmp");
const HERBE1: &[u8] = include_bytes!("../data/herbe1.bmp");
const HERBE2: &[u8] = include_bytes!("../data/herbe2.bmp");
const HERBE3: &[u8] = include_bytes!("../data/herbe3.bmp");
const HERBE4: &[u8] = include_bytes!("../data/herbe4.bmp");
const NUAGES0: &[u8] = include_bytes!("../data/nuages0.bmp");
const NUAGES1: &[u8] = include_bytes!("../data/nuages1.bmp");
const NUAGES2: &[u8] = include_bytes!("../data/nuages2.bmp");
const NUAGES3: &[u8] = include_bytes!("../data/nuages3.bmp");
const NUAGES4: &[u8] = include_bytes!("../data/nuages4.bmp");
const BARRIERE: &[u8] = include_bytes!("../data/barriere.bmp");
const MONTAGNES: &[u8] = include_bytes!("../data/montagnes.bmp");
const LUNE: &[u8] = include_bytes!("../data/lune.bmp");

struct Textures<'a> {
    herbe0: Texture<'a>,
    herbe1: Texture<'a>,
    herbe2: Texture<'a>,
    herbe3: Texture<'a>,
    herbe4: Texture<'a>,
    nuages0: Texture<'a>,
    nuages1: Texture<'a>,
    nuages2: Texture<'a>,
    nuages3: Texture<'a>,
    nuages4: Texture<'a>,
    barriere: Texture<'a>,
    montagnes: Texture<'a>,
    lune: Texture<'a>,
}

struct App {
    canvas: WindowCanvas,
    events: EventPump,
    textures: Textures<'static>,
    scroll: i32,
    /// Accumulated time toward the next 20 ms scroll tick (web + native shared pace).
    tick_accum: Duration,
    last_frame: Instant,
}

fn load_texture<'a>(
    creator: &'a TextureCreator<WindowContext>,
    bytes: &[u8],
    color_key: bool,
) -> Result<Texture<'a>, String> {
    let mut io = IOStream::from_bytes(bytes).map_err(|e| e.to_string())?;
    let surface = Surface::load_bmp_rw(&mut io).map_err(|e| e.to_string())?;
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

fn load_textures(creator: &'static TextureCreator<WindowContext>) -> Result<Textures<'static>, String> {
    Ok(Textures {
        herbe0: load_texture(creator, HERBE0, false)?,
        herbe1: load_texture(creator, HERBE1, false)?,
        herbe2: load_texture(creator, HERBE2, false)?,
        herbe3: load_texture(creator, HERBE3, false)?,
        herbe4: load_texture(creator, HERBE4, false)?,
        nuages0: load_texture(creator, NUAGES0, true)?,
        nuages1: load_texture(creator, NUAGES1, true)?,
        nuages2: load_texture(creator, NUAGES2, true)?,
        nuages3: load_texture(creator, NUAGES3, true)?,
        nuages4: load_texture(creator, NUAGES4, true)?,
        barriere: load_texture(creator, BARRIERE, true)?,
        montagnes: load_texture(creator, MONTAGNES, true)?,
        lune: load_texture(creator, LUNE, true)?,
    })
}

fn init_app() -> Result<App, String> {
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
        .set_logical_size(
            WIDTH,
            HEIGHT,
            sdl3::sys::render::SDL_RendererLogicalPresentation::LETTERBOX,
        )
        .map_err(|e| e.to_string())?;

    println!("Shadow of the Blitz {VERSION}");
    println!("http://www.glop.org/software/sotb");
    println!();
    let (w, h) = canvas.output_size().map_err(|e| e.to_string())?;
    println!("Resolution used: {w}x{h}");

    // Leak the creator so textures can be 'static and live in App (avoids self-ref struct).
    let creator: &'static TextureCreator<WindowContext> =
        Box::leak(Box::new(canvas.texture_creator()));
    let textures = load_textures(creator)?;
    let events = sdl.event_pump().map_err(|e| e.to_string())?;

    Ok(App {
        canvas,
        events,
        textures,
        scroll: 0,
        tick_accum: Duration::ZERO,
        last_frame: Instant::now(),
    })
}

fn draw_frame(app: &mut App) {
    let canvas = &mut app.canvas;
    let t = &app.textures;
    let scroll = app.scroll;

    draw_sky(canvas);

    let _ = canvas.copy(&t.lune, None, Rect::new(184, 16, t.lune.width(), t.lune.height()));

    draw_plane(canvas, &t.montagnes, scroll, 1.0, 97);
    draw_plane(canvas, &t.herbe0, scroll, 2.0, 170);
    draw_plane(canvas, &t.herbe1, scroll, 3.0, 172);
    draw_plane(canvas, &t.herbe2, scroll, 4.0, 175);
    draw_plane(canvas, &t.herbe3, scroll, 5.0, 182);
    draw_plane(canvas, &t.herbe4, scroll, 6.0, 189);
    draw_plane(canvas, &t.barriere, scroll, 7.0, 179);

    draw_plane(canvas, &t.nuages0, scroll, 2.0, 0);
    draw_plane(canvas, &t.nuages1, scroll, 1.0, 22);
    draw_plane(canvas, &t.nuages2, scroll, 1.0 / 2.0, 63);
    draw_plane(canvas, &t.nuages3, scroll, 1.0 / 3.0, 82);
    draw_plane(canvas, &t.nuages4, scroll, 1.0 / 4.0, 91);

    canvas.present();
}

/// Advance scroll by one step per 20 ms of real time (same cadence as the original sleep loop).
fn advance_scroll(app: &mut App) {
    let now = Instant::now();
    let dt = now.saturating_duration_since(app.last_frame);
    app.last_frame = now;
    app.tick_accum += dt;

    while app.tick_accum >= TICK_INTERVAL {
        app.tick_accum -= TICK_INTERVAL;
        app.scroll += 1;
    }
}

/// Returns false when the app should exit.
fn handle_events(app: &mut App) -> bool {
    for event in app.events.poll_iter() {
        match event {
            Event::Quit { .. } => {
                println!("Got quit event, exiting.");
                return false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                println!("ESC key hit, exiting.");
                return false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                println!("switching fullscreen");
                let window = app.canvas.window_mut();
                let off = window.fullscreen_state() == sdl3::video::FullscreenType::Off;
                if let Err(e) = window.set_fullscreen(off) {
                    eprintln!("fullscreen toggle failed: {e}");
                }
            }
            _ => {}
        }
    }
    true
}

fn frame(app: &mut App) -> bool {
    draw_frame(app);
    advance_scroll(app);
    handle_events(app)
}

#[cfg(target_os = "emscripten")]
mod emscripten {
    use std::cell::RefCell;
    use std::os::raw::c_int;

    use super::App;

    thread_local! {
        static APP: RefCell<Option<App>> = const { RefCell::new(None) };
    }

    unsafe extern "C" {
        fn emscripten_set_main_loop(func: unsafe extern "C" fn(), fps: c_int, simulate_infinite_loop: c_int);
        fn emscripten_cancel_main_loop();
    }

    unsafe extern "C" fn main_loop() {
        let running = APP.with(|cell| {
            let mut slot = cell.borrow_mut();
            match slot.as_mut() {
                Some(app) => super::frame(app),
                None => false,
            }
        });
        if !running {
            APP.with(|cell| *cell.borrow_mut() = None);
            unsafe { emscripten_cancel_main_loop() };
        }
    }

    pub fn run(app: App) -> ! {
        APP.with(|cell| *cell.borrow_mut() = Some(app));
        // fps=0 → requestAnimationFrame; scroll pace is time-based (20 ms ticks).
        unsafe {
            emscripten_set_main_loop(main_loop, 0, 1);
        }
        // simulate_infinite_loop=1: this does not return when called from main.
        unreachable!("emscripten main loop returned");
    }
}

fn main() -> Result<(), String> {
    let app = init_app()?;

    #[cfg(target_os = "emscripten")]
    emscripten::run(app);

    #[cfg(not(target_os = "emscripten"))]
    {
        let mut app = app;
        // Pace at one frame presentation per tick interval when ahead of the clock.
        // Scroll still advances from real elapsed time so hitch recovery stays smooth.
        let mut next_present = Instant::now() + TICK_INTERVAL;
        loop {
            if !frame(&mut app) {
                break;
            }
            let now = Instant::now();
            if next_present > now {
                std::thread::sleep(next_present - now);
            }
            next_present += TICK_INTERVAL;
            if next_present < Instant::now() {
                // Fell behind (e.g. window drag) — resync so we don't busy-loop catch-up.
                next_present = Instant::now() + TICK_INTERVAL;
            }
        }
        Ok(())
    }
}
