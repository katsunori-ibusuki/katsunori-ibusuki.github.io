extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use js_sys::Math;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Ball {
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
    color: String,
    size: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, vel_x: f64, vel_y: f64, r: u8, g: u8, b: u8, size: f64) -> Ball {
        Ball {
            x,
            y,
            vel_x,
            vel_y,
            color: format!("rgb({}, {}, {})", r, g, b),
            size,
        }
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.begin_path();
        let color_js_value = wasm_bindgen::JsValue::from_str(&self.color);
        ctx.set_fill_style(&color_js_value);
        ctx.arc(self.x, self.y, self.size, 0.0, 2.0 * std::f64::consts::PI)
            .expect("Failed to draw arc.");
        ctx.fill();
    }

    pub fn update(&mut self, width: f64, height: f64) {
        if self.x + self.size >= width || self.x - self.size <= 0.0 {
            self.vel_x = -self.vel_x;
        }
        if self.y + self.size >= height || self.y - self.size <= 0.0 {
            self.vel_y = -self.vel_y;
        }

        self.x += self.vel_x;
        self.y += self.vel_y;
    }
}

#[wasm_bindgen]
pub fn run() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let width = document.body().unwrap().client_width() as f64;
    let height = document.body().unwrap().client_height() as f64;
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut balls = Vec::new();

    while balls.len() < 100 {
        let size = unsafe { Math::random() * 10.0 } as f64 + 10.0;
        let x = unsafe { Math::random() } * width;
        let y = unsafe { Math::random() } * height;
        let vel_x = unsafe { Math::random() } * 5.0 - 5.0;
        let vel_y = unsafe { Math::random() } * 5.0 - 5.0;
        let r = (unsafe { Math::random() } * 255.0) as u8;
        let g = (unsafe { Math::random() } * 255.0) as u8;
        let b = (unsafe { Math::random() } * 255.0) as u8;

        let ball = Ball::new(x, y, vel_x, vel_y, r, g, b, size);
        balls.push(ball);
    }

    let window_clone = window.clone();
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));

    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.set_fill_style(&wasm_bindgen::JsValue::from_str("rgba(0,0,0,0.25)"));
        context.fill_rect(0.0, 0.0, width, height);
        for ball in balls.iter_mut() {
            ball.draw(&context);
            ball.update(width, height);
        }
        // 再帰的に次のアニメーションフレームをリクエスト
        window_clone
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }) as Box<dyn FnMut()>));

    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() {
    run(); // Call the `run` function (or whatever initialization logic you have)
}
