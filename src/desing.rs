use crate::framebuffer::Framebuffer;


// x y y puntos iniciales del diseño
impl Framebuffer {
    pub fn glider(&mut self, x: usize, y: usize) {
        self.point(x, y);
        self.point(x + 1, y + 1);
        self.point(x + 2, y + 1);
        self.point(x, y + 2);
        self.point(x + 1, y + 2);
    }

    pub fn blinker(&mut self, x: usize, y: usize) {
        self.point(x, y);
        self.point(x + 1, y);
        self.point(x + 2, y);
    }

    pub fn boat(&mut self, x: usize, y: usize) {
        self.point(x, y);
        self.point(x + 1, y);
        self.point(x, y + 1);
        self.point(x + 2, y + 1);
        self.point(x + 1, y + 2);
    }

    pub fn hwss(&mut self, x: usize, y: usize) {
        self.point(x, y);
        self.point(x + 1, y);
        self.point(x + 2, y);
        self.point(x + 3, y);
        self.point(x + 4, y);
        self.point(x + 5, y);
        self.point(x - 1, y + 1);
        self.point(x - 1, y + 3);
        self.point(x + 1, y + 4);
        self.point(x + 2, y + 4);
        self.point(x + 4, y + 3);
        self.point(x + 5, y + 1);
        self.point(x + 5, y + 2);
    }

    pub fn tub(&mut self, x: usize, y: usize) {
        self.point(x, y);
        self.point(x + 1, y - 1);
        self.point(x + 1, y + 1);
        self.point(x + 2, y);
    }

    pub fn pulsar(&mut self, x: usize, y: usize) {
        self.point(x,y);
        self.point(x + 1, y);
        self.point(x + 2, y);
        self.point(x + 2, y - 1);
        
        self.point(x,y - 6);
        self.point(x + 1, y - 6);
        self.point(x + 2, y - 6 );
        self.point(x + 2, y - 5);

        self.point(x + 12,y);
        self.point(x + 13, y);
        self.point(x + 14, y);
        self.point(x + 12, y - 1);
        
        self.point(x + 12,y - 6);
        self.point(x + 13, y - 6);
        self.point(x + 14, y - 6 );
        self.point(x + 12, y - 5);

        self.point(x + 4, y + 2);
        self.point(x + 5, y + 2);
        self.point(x + 4, y + 3);
        self.point(x + 4, y + 4);

        self.point(x + 9, y + 2);
        self.point(x + 10, y + 2);
        self.point(x + 10, y + 3);
        self.point(x + 10, y + 4);

        self.point(x + 4, y - 8);
        self.point(x + 5, y - 8);
        self.point(x + 4, y - 9);
        self.point(x + 4, y - 10);

        self.point(x + 9, y - 8);
        self.point(x + 10, y - 8);
        self.point(x + 10, y - 9);
        self.point(x + 10, y - 10);

        self.point(x + 5, y);
        self.point(x + 6, y);
        self.point(x + 6, y - 1);
        self.point(x + 4, y - 1);
        self.point(x + 4, y - 2);
        self.point(x + 5, y - 2);

        self.point(x + 8, y);
        self.point(x + 9, y);
        self.point(x + 8, y - 1);
        self.point(x + 10, y - 1);
        self.point(x + 10, y - 2);
        self.point(x + 9, y - 2);
        
        self.point(x + 9, y - 4);
        self.point(x + 10, y - 4);
        self.point(x + 10, y - 5);
        self.point(x + 8, y - 5);
        self.point(x + 8, y - 6);
        self.point(x + 9, y - 6);
        
        self.point(x + 4, y - 4);
        self.point(x + 5, y - 4);
        self.point(x + 4, y - 5);
        self.point(x + 6, y - 5);
        self.point(x + 6, y - 6);
        self.point(x + 6, y - 6);
        self.point(x + 5, y - 6);
    }

    pub fn goe(&mut self, x: usize, y: usize) {
        self.point(x,y);
        self.point(x,y - 1);
        self.point(x,y - 2);
        self.point(x,y - 3);
        self.point(x,y - 4);

        self.point(x + 1,y + 1);
        self.point(x + 1,y);
        self.point(x + 1,y - 2);
        self.point(x + 1,y - 4);
        self.point(x + 1,y - 6);

        self.point(x + 2,y + 1);
        self.point(x + 2,y - 1);
        self.point(x + 2,y - 4);
        self.point(x + 2,y - 5 );

        self.point(x + 3,y + 1);
        self.point(x + 3,y - 2);
        self.point(x + 3,y - 4);
        self.point(x + 3,y - 6 );

        self.point(x + 4,y + 1);
        self.point(x + 4,y + 2);
        self.point(x + 4,y - 1);
        self.point(x + 4,y - 5);
        self.point(x + 4,y - 6 );

        self.point(x + 5,y + 2);
        self.point(x + 5,y - 1);
        self.point(x + 5,y - 3);
        self.point(x + 5,y - 5);

        self.point(x + 6,y + 2);
        self.point(x + 6,y - 1);
        self.point(x + 6,y - 2);
        self.point(x + 6,y - 4);
        self.point(x + 6,y - 6);

        self.point(x + 7,y + 2);
        self.point(x + 7,y + 1);
        self.point(x + 7,y);
        self.point(x + 7,y - 2);
        self.point(x + 7,y - 3);
        self.point(x + 7,y - 5);

        self.point(x + 8,y + 2);
        self.point(x + 8,y);
        self.point(x + 8,y - 1);
        self.point(x + 8,y - 3);
        self.point(x + 8,y - 5);

        self.point(x + 9,y + 2);
        self.point(x + 9,y + 1);
        self.point(x + 9,y - 2);
        self.point(x + 9,y - 4);
        self.point(x + 9,y - 5);
        self.point(x + 9,y - 6);

        self.point(x + 10,y + 1);
        self.point(x + 10,y);
        self.point(x + 10,y - 1);
        self.point(x + 10,y - 3);
        self.point(x + 10,y - 6);

        self.point(x + 11,y);
        self.point(x + 11,y - 2);
        self.point(x + 11,y - 3);
        self.point(x + 11,y - 4);
        self.point(x + 11,y - 6);
    }

    pub fn clock(&mut self, x: usize, y: usize) {
        self.point(x,y);
        self.point(x + 1,y - 1);
        self.point(x + 1,y - 2);
        self.point(x + 2,y + 1);
        self.point(x + 2,y );
        self.point(x + 3,y -1);
    }

    pub fn bunnies(&mut self, x: usize, y: usize) {
        self.point(x,y);
        self.point(x + 1,y - 3);
        self.point(x + 2,y - 1);
        self.point(x + 2,y - 2);
        self.point(x + 3,y - 3);
        self.point(x + 5,y - 2);
        self.point(x + 6,y);
        self.point(x + 6,y - 1);
        self.point(x + 7,y - 2);
        
    }

    pub fn ggg(&mut self, x: usize, y: usize) {
        self.point(x,y);
        self.point(x + 1,y);
        self.point(x + 1,y + 1);
        self.point(x ,y + 1);

        self.point(x + 10, y);
        self.point(x + 10, y + 1);
        self.point(x + 10, y + 2);
        self.point(x + 11, y - 1);
        self.point(x + 11, y + 3);
        self.point(x + 12, y - 2 );
        self.point(x + 12, y + 4 );
        self.point(x + 13, y - 2 );
        self.point(x + 13, y + 4 );
        self.point(x + 14, y + 1 );
        self.point(x + 15, y - 1 );
        self.point(x + 15, y + 3 );
        self.point(x + 16, y );
        self.point(x + 16, y + 1 );
        self.point(x + 16, y + 2 );
        self.point(x + 17, y + 1 );

        self.point(x + 20, y - 2);
        self.point(x + 20, y - 1);
        self.point(x + 20, y);
        self.point(x + 21, y - 2);
        self.point(x + 21, y - 1);
        self.point(x + 21, y);
        self.point(x + 22, y - 3);
        self.point(x + 22, y + 1);
        self.point(x + 24, y - 3);
        self.point(x + 24, y - 4);
        self.point(x + 24, y + 1);
        self.point(x + 24, y + 2);

        self.point(x + 34, y - 1);
        self.point(x + 34, y - 2);
        self.point(x + 35, y - 1);
        self.point(x + 35, y - 2);
        
    }
}