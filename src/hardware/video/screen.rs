use piston_window::*;
use texture::*;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_DIMS: [u32; 2] = [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32];
const FRAMEBUFFER_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize;
const SCREEN_SCALE : f64 = 2.0;

pub struct Screen {
    green: f32,
    delta: f32,
    framebuffer: [u8; FRAMEBUFFER_SIZE],
    image: Image,
    texture_settings: TextureSettings,
}

impl Screen {
    pub fn new(window: &mut PistonWindow) -> Self {
        let buffer = [255; FRAMEBUFFER_SIZE];
        let ts = TextureSettings::new().filter(Filter::Nearest).compress(false).generate_mipmap(false);
        
        Screen {
            green: 0.0,
            delta: 0.01,
            framebuffer: buffer,
            image: Image::new(),
            texture_settings: ts,
        }
    }

    pub fn update(&mut self, window: &mut PistonWindow, evt: Event) {
        if self.green >= 1.0 {self.delta = -0.01;} 
        else if self.green <= 0.0 {self.delta = 0.01;};
        self.green = self.green + self.delta;

        // TODO: Really would like to not have to create a new texture every frame
        let texture = Texture::create(
            &mut window.factory,
            Format::Rgba8,
            &self.framebuffer,
            SCREEN_DIMS,
            &self.texture_settings
        ).unwrap_or_else(|err|{panic!()});
        
        window.draw_2d(&evt, |c, g| { 
            //clear([0.5, self.green, 0.5, 1.0], g);
            self.image.draw(&texture, &c.draw_state, c.transform.scale(SCREEN_SCALE, SCREEN_SCALE), g);
        });
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, r: u8, g: u8, b: u8) {
        let first_index = 4 * (x as usize + (y as usize * SCREEN_WIDTH));
        self.framebuffer[first_index]       = r;
        self.framebuffer[first_index + 1]   = g;
        self.framebuffer[first_index + 2]   = b;
        self.framebuffer[first_index + 3]   = 255;
    }
}