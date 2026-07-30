use minifb::{Key, Window, WindowOptions};

mod framebuffer;

use crate::framebuffer::Framebuffer;

fn main() {
    // Dimensiones de la pantalla
    let window_width = 1280;
    let window_height = 720;

    // Dimensiones delFramebuffer
    let framebuffer_width = 100;
    let framebuffer_height = 100;

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
    
    // Configuración y creación de un punto usando el framebuffer
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.clear();

    framebuffer.set_background_color(0x000000);
    framebuffer.point(50, 50);
    
    // Bucle principal de la aplicación
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();
    }
}
