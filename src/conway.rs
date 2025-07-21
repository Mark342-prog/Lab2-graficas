use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

pub struct GameOfLife {
    width: usize,
    height: usize,
    current_generation: Vec<Vec<bool>>,
    next_generation: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let mut game = GameOfLife {
            width,
            height,
            current_generation: vec![vec![false; width]; height],
            next_generation: vec![vec![false; width]; height],
        };
        
        game.initialize_pattern();
        game
    }
    
 
    fn initialize_pattern(&mut self) {

        if self.width > 15 && self.height > 15 {
           
            self.current_generation[1][2] = true;
            self.current_generation[2][3] = true;
            self.current_generation[3][1] = true;
            self.current_generation[3][2] = true;
            self.current_generation[3][3] = true;
            
            
            let offset_x = self.width - 5;
            self.current_generation[1][offset_x] = true;
            self.current_generation[2][offset_x - 1] = true;
            self.current_generation[3][offset_x] = true;
            self.current_generation[3][offset_x + 1] = true;
            self.current_generation[3][offset_x + 2] = true;
            
           
            let offset_y = self.height - 5;
            self.current_generation[offset_y][1] = true;
            self.current_generation[offset_y][2] = true;
            self.current_generation[offset_y][3] = true;
            self.current_generation[offset_y - 1][3] = true;
            self.current_generation[offset_y - 2][2] = true;
        }
        
        
        if self.width > 40 && self.height > 20 {
            let gun_x = 10;
            let gun_y = 10;
            
        
            self.current_generation[gun_y][gun_x] = true;
            self.current_generation[gun_y][gun_x + 1] = true;
            self.current_generation[gun_y + 1][gun_x] = true;
            self.current_generation[gun_y + 1][gun_x + 1] = true;
            
        
            self.current_generation[gun_y][gun_x + 10] = true;
            self.current_generation[gun_y + 1][gun_x + 10] = true;
            self.current_generation[gun_y + 2][gun_x + 10] = true;
            self.current_generation[gun_y + 1][gun_x + 11] = true;
            self.current_generation[gun_y][gun_x + 12] = true;
            self.current_generation[gun_y + 2][gun_x + 12] = true;
            
          
            self.current_generation[gun_y + 1][gun_x + 16] = true;
            self.current_generation[gun_y][gun_x + 17] = true;
            self.current_generation[gun_y + 2][gun_x + 17] = true;
            self.current_generation[gun_y + 1][gun_x + 18] = true;
            
        
            self.current_generation[gun_y][gun_x + 24] = true;
            self.current_generation[gun_y][gun_x + 25] = true;
            self.current_generation[gun_y + 1][gun_x + 24] = true;
            self.current_generation[gun_y + 1][gun_x + 25] = true;
        }
        
 
        let positions = [(15, 30), (60, 20), (30, 70), (80, 80), (25, 85)];
        for (x, y) in positions.iter() {
            if *x + 2 < self.width && *y < self.height {
                self.current_generation[*y][*x] = true;
                self.current_generation[*y][*x + 1] = true;
                self.current_generation[*y][*x + 2] = true;
            }
        }
        
  
        let beacon_positions = [(40, 40), (70, 30), (20, 60)];
        for (x, y) in beacon_positions.iter() {
            if *x + 3 < self.width && *y + 3 < self.height {
                self.current_generation[*y][*x] = true;
                self.current_generation[*y][*x + 1] = true;
                self.current_generation[*y + 1][*x] = true;
                self.current_generation[*y + 2][*x + 2] = true;
                self.current_generation[*y + 2][*x + 3] = true;
                self.current_generation[*y + 3][*x + 2] = true;
                self.current_generation[*y + 3][*x + 3] = true;
            }
        }
        
        let center_x = self.width / 2;
        let center_y = self.height / 2;
        if center_x > 5 && center_y > 5 {
            self.current_generation[center_y][center_x] = true;
            self.current_generation[center_y][center_x + 1] = true;
            self.current_generation[center_y - 1][center_x] = true;
            self.current_generation[center_y + 1][center_x - 1] = true;
            self.current_generation[center_y + 1][center_x] = true;
        }
        
        let random_positions = [
            (10, 10), (20, 15), (30, 12), (40, 18), (50, 25),
            (990, 10), (980, 15), (970, 20), (960, 12), (950, 18),
            (10, 990), (20, 980), (15, 970), (25, 960), (30, 950),
            (990, 990), (980, 985), (975, 975), (965, 980), (970, 970),
            (500, 10), (510, 15), (490, 12), (520, 18), (480, 20),
            (10, 500), (15, 510), (20, 490), (12, 520), (18, 480),
            (990, 500), (985, 510), (975, 490), (980, 520), (970, 480),
            (500, 990), (510, 985), (490, 975), (520, 980), (480, 970),
            (100, 100), (150, 120), (200, 180), (250, 220), (300, 280),
            (350, 320), (400, 380), (450, 420), (500, 480), (550, 520),
            (600, 580), (650, 620), (700, 680), (750, 720), (800, 780),
            (850, 820), (900, 880), (120, 200), (180, 300), (220, 400),
            (280, 500), (320, 600), (380, 700), (420, 800), (480, 150),
            (520, 250), (580, 350), (620, 450), (680, 550), (720, 650),
            (780, 750), (820, 850), (880, 180), (160, 320), (240, 420),
            (340, 520), (440, 620), (540, 720), (640, 820), (740, 160),
            (840, 260), (140, 360), (260, 460), (360, 560), (460, 660),
            (560, 760), (660, 860), (760, 140), (860, 240), (80, 380),
            (380, 80), (180, 580), (580, 180), (280, 780), (780, 280),
            (400, 400), (450, 450), (550, 550), (600, 400), (400, 600),
            (350, 500), (500, 350), (650, 450), (450, 650), (525, 475),
            (475, 525), (425, 575), (575, 425), (375, 625), (625, 375),
            (75, 125), (125, 75), (225, 175), (175, 225), (325, 275),
            (275, 325), (425, 375), (375, 425), (525, 575), (575, 525),
            (675, 725), (725, 675), (825, 875), (875, 825), (925, 25),
            (25, 925), (775, 125), (125, 775), (675, 225), (225, 675),
            (505, 505), (150, 480), (480, 150), (700, 700), (450, 820),
            (820, 450), (150, 150), (750, 750), (300, 600), (600, 300),
            (350, 700), (700, 350), (800, 120), (120, 800), (480, 480),
            (520, 520), (260, 840), (840, 260), (630, 360), (360, 630),
            (250, 680), (680, 250), (320, 740), (740, 320), (590, 590),
            (410, 410), (810, 710), (710, 810), (170, 320), (330, 170)
        ];
        
        for (x, y) in random_positions.iter() {
            if *x < self.width && *y < self.height {
                self.current_generation[*y][*x] = true;
                if (*x + *y) % 3 == 0 {
                    if *x + 1 < self.width {
                        self.current_generation[*y][*x + 1] = true;
                    }
                    if *y + 1 < self.height {
                        self.current_generation[*y + 1][*x] = true;
                    }
                    if *x + 1 < self.width && *y + 1 < self.height {
                        self.current_generation[*y + 1][*x + 1] = true;
                    }
                }
                if (*x + *y) % 5 == 0 && *x + 2 < self.width && *y + 2 < self.height {
                    self.current_generation[*y][*x + 1] = true;
                    self.current_generation[*y][*x + 2] = true;
                    self.current_generation[*y + 1][*x] = true;
                    self.current_generation[*y + 2][*x] = true;
                }
            }
        }
        
       
        let pulsar_x = self.width - 20;
        let pulsar_y = 15;
        if pulsar_x > 15 && pulsar_y + 15 < self.height {
            
            let offsets = [2, 3, 4, 8, 9, 10];
            for offset in offsets.iter() {
              
                self.current_generation[pulsar_y + 6][pulsar_x + offset] = true;
                self.current_generation[pulsar_y + 8][pulsar_x + offset] = true;
                
                self.current_generation[pulsar_y + offset][pulsar_x + 6] = true;
                self.current_generation[pulsar_y + offset][pulsar_x + 8] = true;
            }
        }
        
        
        let gun_positions = [(100, 100), (700, 200), (200, 700), (800, 800)];
        for (base_x, base_y) in gun_positions.iter() {
            if *base_x + 36 < self.width && *base_y + 9 < self.height {
               
                self.current_generation[*base_y + 4][*base_x] = true;
                self.current_generation[*base_y + 4][*base_x + 1] = true;
                self.current_generation[*base_y + 5][*base_x] = true;
                self.current_generation[*base_y + 5][*base_x + 1] = true;
                
                
                self.current_generation[*base_y + 3][*base_x + 10] = true;
                self.current_generation[*base_y + 5][*base_x + 10] = true;
                self.current_generation[*base_y + 2][*base_x + 11] = true;
                self.current_generation[*base_y + 6][*base_x + 11] = true;
                self.current_generation[*base_y + 1][*base_x + 12] = true;
                self.current_generation[*base_y + 7][*base_x + 12] = true;
                self.current_generation[*base_y + 1][*base_x + 13] = true;
                self.current_generation[*base_y + 7][*base_x + 13] = true;
                self.current_generation[*base_y + 4][*base_x + 14] = true;
                
               
                self.current_generation[*base_y + 2][*base_x + 15] = true;
                self.current_generation[*base_y + 6][*base_x + 15] = true;
                self.current_generation[*base_y + 3][*base_x + 16] = true;
                self.current_generation[*base_y + 5][*base_x + 16] = true;
                self.current_generation[*base_y + 4][*base_x + 17] = true;
                
               
                self.current_generation[*base_y + 3][*base_x + 20] = true;
                self.current_generation[*base_y + 2][*base_x + 20] = true;
                self.current_generation[*base_y + 1][*base_x + 20] = true;
                self.current_generation[*base_y + 3][*base_x + 21] = true;
                self.current_generation[*base_y + 2][*base_x + 21] = true;
                self.current_generation[*base_y + 1][*base_x + 21] = true;
                self.current_generation[*base_y][*base_x + 22] = true;
                self.current_generation[*base_y + 4][*base_x + 22] = true;
                
                
                self.current_generation[*base_y][*base_x + 24] = true;
                self.current_generation[*base_y + 4][*base_x + 24] = true;
                
               
                self.current_generation[*base_y + 1][*base_x + 34] = true;
                self.current_generation[*base_y + 2][*base_x + 34] = true;
                self.current_generation[*base_y + 1][*base_x + 35] = true;
                self.current_generation[*base_y + 2][*base_x + 35] = true;
            }
        }
        

        let r_pent_positions = [(300, 300), (600, 600), (150, 850), (850, 150)];
        for (x, y) in r_pent_positions.iter() {
            if *x + 2 < self.width && *y + 2 < self.height {
                self.current_generation[*y][*x + 1] = true;
                self.current_generation[*y][*x + 2] = true;
                self.current_generation[*y + 1][*x] = true;
                self.current_generation[*y + 1][*x + 1] = true;
                self.current_generation[*y + 2][*x + 1] = true;
            }
        }
        
       
        let diehard_positions = [(400, 200), (600, 800)];
        for (x, y) in diehard_positions.iter() {
            if *x + 7 < self.width && *y + 2 < self.height {
                self.current_generation[*y][*x + 6] = true;
                self.current_generation[*y + 1][*x] = true;
                self.current_generation[*y + 1][*x + 1] = true;
                self.current_generation[*y + 2][*x + 1] = true;
                self.current_generation[*y + 2][*x + 5] = true;
                self.current_generation[*y + 2][*x + 6] = true;
                self.current_generation[*y + 2][*x + 7] = true;
            }
        }
    }
    
   
    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        
        
        for dy in -1i32..=1i32 {
            for dx in -1i32..=1i32 {
                if dx == 0 && dy == 0 {
                    continue; 
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
           
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    if self.current_generation[ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }
    
   
    pub fn update(&mut self) {
       
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let is_alive = self.current_generation[y][x];
                
            
                self.next_generation[y][x] = match (is_alive, neighbors) {
                    
                    (true, n) if n < 2 => false,
                  
                    (true, 2) | (true, 3) => true,
             
                    (true, n) if n > 3 => false,
              
                    (false, 3) => true,
                    
                    (state, _) => state,
                };
            }
        }
        
   
        std::mem::swap(&mut self.current_generation, &mut self.next_generation);
    }
    
  
    pub fn render(&self, framebuffer: &mut Framebuffer) {

        for y in 0..self.height {
            for x in 0..self.width {
                if self.current_generation[y][x] {
                    framebuffer.set_current_color(Color::WHITE);
                } else {
                    framebuffer.set_current_color(Color::BLACK);
                }
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
    

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.current_generation[y][x] = !self.current_generation[y][x];
        }
    }
    

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.current_generation[y][x] = false;
                self.next_generation[y][x] = false;
            }
        }
    }
    
    
    pub fn reset(&mut self) {
        self.clear();
        self.initialize_pattern();
    }
}
