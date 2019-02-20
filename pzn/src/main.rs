
extern crate sdl2; 

use std::borrow::BorrowMut;
use std::cell::RefCell;

use sdl2::video::Window;
use wasmer_runtime::Ctx;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::EventPump;
use std::time::Duration;


use wasmer_runtime::{
    instantiate,
    Value,
    imports,
    error,
};

use std::io;
use std::io::prelude::*;
use std::fs::File;
/*
static WASM: &'static [u8] = &[
    // The module above compiled to bytecode goes here.
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x06, 0x01, 0x60,
    0x01, 0x7f, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x0b, 0x01, 0x07,
    0x61, 0x64, 0x64, 0x5f, 0x6f, 0x6e, 0x65, 0x00, 0x00, 0x0a, 0x09, 0x01,
    0x07, 0x00, 0x20, 0x00, 0x41, 0x01, 0x6a, 0x0b, 0x00, 0x1a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x01, 0x0a, 0x01, 0x00, 0x07, 0x61, 0x64, 0x64, 0x5f,
    0x6f, 0x6e, 0x65, 0x02, 0x07, 0x01, 0x00, 0x01, 0x00, 0x02, 0x70, 0x30,
];
*/


thread_local!(
static PZN_RUNTIME: RefCell<PznRuntime> = RefCell::new(PznRuntime::new())
);

// static PZN_RUNTIME: RefCell<Option<PznRuntime>> = RefCell::default();
// }

extern fn f64_print(n: f64, _: &mut Ctx) {
    println!("{}", n);
}


pub struct PznRuntime {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl PznRuntime {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        PznRuntime { canvas, event_pump }
    }

    pub fn clear(&self) {
    }

    pub extern fn draw_rectangle(&self, x: f64, y: f64, width: f64, height: f64, _: &mut Ctx) {
        println!("rect: {}, {}, {}, {}", x, y, width, height);

    
    /*
                canvas.set_draw_color(Color::RGB(255, 0, 0));
            // let color = Color::RGB(255, 0, 0);
            let rect = Rect::new(20, 20, 400, 200);
            canvas.fill_rect(rect).unwrap();
    */
    }

}

pub extern fn draw_rectangle(x: f64, y: f64, width: f64, height: f64, _: &mut Ctx) {
    println!("rect: {}, {}, {}, {}", x, y, width, height);
    PZN_RUNTIME.with(|runtime_cell| {
        let mut runtime = runtime_cell.borrow_mut();
        
        runtime.canvas.set_draw_color(Color::RGB(255, 0, 0));
        // let color = Color::RGB(255, 0, 0);
        let rect = Rect::new(x as i32, y as i32, width as u32, height as u32);
        runtime.canvas.fill_rect(rect).unwrap();
    });
}

pub fn main() -> error::Result<()> {
    println!("Hello, world!");

    // let runtime = PznRuntime::new();

    // PZN_RUNTIME.borrow_mut().replace(runtime);

    // runtime.load('myshader')

    let mut f = File::open("shader.wasm").unwrap();
    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer).unwrap();
    let WASM = buffer.clone();

    
    let mut i = 0;

/*
    fn func(x: f64, ctx: &mut Ctx) {
        runtime.draw_rectangle(x, x, x, x, ctx)
    };
*/
    //let import_object = imports! {};
    // let func = &runtime.draw_rectangle;
    let import_object = imports! {
        "env" => {
            "f64_print" => f64_print<[f64] -> []>,
            "draw_rectangle" => draw_rectangle<[f64, f64, f64, f64] -> []>,
        },
    };

    let mut instance = instantiate(&WASM, import_object)?;

    'running: loop {

        PZN_RUNTIME.with(|runtime_cell| {
            let mut runtime = runtime_cell.borrow_mut();
            i = (i + 1) % 255;
            runtime.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            runtime.canvas.clear();

            for event in runtime.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        // break 'running
                        unimplemented!("TODO");
                    },
                    _ => {}
                }
            }
        });
        // The rest of the game loop goes here...

/*
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        // let color = Color::RGB(255, 0, 0);
        let rect = Rect::new(20, 20, 400, 200);
        canvas.fill_rect(rect).unwrap();
  */      
        let values = instance
        .func("draw")?
        .call(&[])?;

        // assert_eq!(values[0], Value::I32(43));

        PZN_RUNTIME.with(|runtime_cell| {
            let mut runtime = runtime_cell.borrow_mut();
            runtime.canvas.present();
        });
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}



/*

fn main() -> error::Result<()> {
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    let mut instance = instantiate(WASM, import_object)?;

    let values = instance
        .func("add_one")?
        .call(&[Value::I32(42)])?;

    assert_eq!(values[0], Value::I32(43));
    
    Ok(())
}

*/