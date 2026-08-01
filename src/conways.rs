use crate::framebuffer::Framebuffer;

impl Framebuffer {
    pub fn get_neighbors(&self, x: usize, y: usize) -> u32 {
        let mut vecinos = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0
                    && nx < self.width as isize
                    && ny >= 0
                    && ny < self.height as isize
                {
                    let idx = ny as usize * self.width + nx as usize;

                    if self.buffer[idx] == self.get_current_color() {
                        vecinos += 1;
                    }
                }
            }
        }
        vecinos
    }

    pub fn update(&mut self) {
        // Crear el siguiente estado del tablero
        let mut next = self.buffer.clone();

        // Obtener los colores
        let vivo = self.get_current_color();
        let muerto = self.get_background_color();

        // Iterar sobre cada célula
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                let celula = self.get_cell(x, y);
                let vecinos = self.get_neighbors(x, y);

                // Si la célula está viva
                if celula == vivo {
                    // Muere por soledad o sobrepoblación
                    if vecinos < 2 || vecinos > 3 {
                        next[idx] = muerto;
                    }
                    // Si tiene 2 o 3 vecinos, sobrevive.
                    // No hace falta hacer nada porque `next`
                    // ya es una copia del tablero actual.
                }
                // Si la célula está muerta
                else if vecinos == 3 {
                    // Nace una nueva célula
                    next[idx] = vivo;
                }
            }
        }
        // Reemplazar el tablero por la nueva generación
        self.buffer = next;
    }
}