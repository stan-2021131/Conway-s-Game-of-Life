use minifb::{Key, Window, WindowOptions};
use std::{thread, time::Duration};

mod framebuffer;
mod conways;
mod desing;

use crate::framebuffer::Framebuffer;

fn main() {
    // Dimensiones de la pantalla
    let window_width = 1280;
    let window_height = 720;

    // Dimensiones delFramebuffer
    let framebuffer_width = 300;
    let framebuffer_height = 200;

    // Crear framebuffer
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    // Crear ventana
    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();
    
    // Configuración de los colores del framebuffer
    framebuffer.set_background_color(0x06294c);
    framebuffer.set_current_color(0xe5ec48);

    framebuffer.clear();

    // Blinkers
    framebuffer.blinker(120, 20);
    framebuffer.blinker(110, 35);
    framebuffer.blinker(250, 40);
    framebuffer.blinker(180, 170);

    // Boats
    framebuffer.boat(45, 60);
    framebuffer.boat(200, 20);
    framebuffer.boat(260, 150);
    framebuffer.boat(100, 130);

    // Tubs
    framebuffer.tub(20, 120);
    framebuffer.tub(150, 80);
    framebuffer.tub(280, 60);
    framebuffer.tub(230, 170);

    // Gliders
    framebuffer.glider(30, 10);
    framebuffer.glider(140, 25);
    framebuffer.glider(260, 100);
    framebuffer.glider(90, 170);

    // HWSS
    framebuffer.hwss(15, 150);
    framebuffer.hwss(180, 50);
    framebuffer.hwss(240, 120);

    // Clocks
    framebuffer.clock(60, 90);
    framebuffer.clock(170, 150);
    framebuffer.clock(250, 15);

    // Bunnies
    framebuffer.bunnies(110, 110);
    framebuffer.bunnies(210, 160);
    framebuffer.bunnies(150, 170);
    framebuffer.bunnies(150, 50);

    // Pulsars
    framebuffer.pulsar(80, 60);
    framebuffer.pulsar(210, 70);
    framebuffer.pulsar(20, 170);

    // Garden of Eden
    framebuffer.goe(20, 20);
    framebuffer.goe(180, 20);
    framebuffer.goe(200, 100);

    // Gosper Glider Gun
    framebuffer.ggg(150, 100);
    framebuffer.ggg(75, 120);
    
    
    // Bucle principal de la aplicación
    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.update();
        window.update_with_buffer(
            &framebuffer.buffer,
            framebuffer.width,
            framebuffer.height,
        ).unwrap();

        thread::sleep(Duration::from_millis(200));
    }
}
