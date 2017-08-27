use piston_window::*;
use texture::*;
use image;

pub const SCREEN_WIDTH: usize = 144;
pub const SCREEN_HEIGHT: usize = 160;
pub const SCREEN_DIMS: [u32; 2] = [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32];
const FRAMEBUFFER_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize;
const SCREEN_SCALE : f64 = 2.0;

pub struct Screen {    
    framebuffer: [u8; FRAMEBUFFER_SIZE],
    texture_settings: TextureSettings,
    is_on: bool,
    texture: G2dTexture
}

impl Screen {
    pub fn new(window: &mut PistonWindow) -> Self {
        let buffer = [255; FRAMEBUFFER_SIZE];
        let ts = TextureSettings::new().filter(Filter::Nearest).compress(false).generate_mipmap(false);
        let texture = Texture::create(
            &mut window.factory,
            Format::Rgba8,
            &buffer,
            SCREEN_DIMS,
            &ts
        ).unwrap_or_else(|err|{panic!()});
        
        Screen {
            framebuffer: buffer,
            texture_settings: ts,
            texture: texture,
            is_on: false,
        }
    }

    pub fn update(&mut self, window: &mut PistonWindow, evt: Event) {
        if !self.is_on {
            UpdateTexture::update(
                &mut self.texture,
                &mut window.encoder, 
                Format::Rgba8,
                &self.framebuffer,
                [0; 2],
                SCREEN_DIMS
            ).unwrap();    

            window.draw_2d(&evt, |c, g| { 
                image(&self.texture, c.transform.scale(SCREEN_SCALE, SCREEN_SCALE), g);
            });
        } else {
            window.draw_2d(&evt, |c, g| { 
                clear([0.0, 0.0, 0.0, 1.0], g);
            });
        }
        
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, rgb: [u8; 3]) {        
        let first_index = 4 * (x as usize + (y as usize * SCREEN_WIDTH));
        self.framebuffer[first_index]       = rgb[0];
        self.framebuffer[first_index + 1]   = rgb[1];
        self.framebuffer[first_index + 2]   = rgb[2];
        self.framebuffer[first_index + 3]   = 255;
    }

    pub fn turn_on_off(&mut self, is_on: bool) {
        self.is_on = is_on;
    }
}