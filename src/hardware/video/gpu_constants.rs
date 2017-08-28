pub const TILE_NUMBER                   : usize = 384;

pub const SPRITE_OAM_START              : u16 = 0xFE00;
pub const SPRITE_OAM_END                : u16 = 0xFE9F;

// The VRAM is subdivided into two sections: Tile maps and tile datas.
// Still, it is useful to know the boundaries
// just for interfacing purpouses
pub const VRAM_START                    : u16 = 0x8000;
pub const VRAM_END                      : u16 = 0x9FFF;
// Granular VRAM regions

// Tile maps
pub const TILE_MAPS_START               : u16 = 0x9800;
pub const TILE_MAPS_END                 : u16 = 0x9FFF;

pub const TILE_MAP_0_START              : u16 = 0x9800;
pub const TILE_MAP_1_START              : u16 = 0x9C00;

// Tile data
pub const TILE_DATA_START               : u16 = 0x8000;
pub const TILE_DATA_END                 : u16 = 0x97FF;
pub const BG_WIN_TILE_DATA_0_START      : u16 = 0x8800;
pub const BG_WIN_TILE_DATA_1_START      : u16 = 0x8000;

// Special registers
pub const LCD_CONTROL_ADDR              : u16 = 0xFF40;
pub const LCD_STATUS_ADDR               : u16 = 0xFF41;
pub const SCROLL_Y_ADDR                 : u16 = 0xFF42;
pub const SCROLL_X_ADDR                 : u16 = 0xFF43;
pub const LY_COORD_ADDR                 : u16 = 0xFF44;
pub const LYC_COMPLARE_ADDR             : u16 = 0xFF45;
pub const DMA_START_ADDR                : u16 = 0xFF46;
pub const BG_PALLETE_ADDR               : u16 = 0xFF47;
pub const OBJECT_PALETTE_1_ADDR         : u16 = 0xFF48;
pub const OBJECT_PALETTE_2_ADDR         : u16 = 0xFF49;
pub const WINDOW_Y_ADDR                 : u16 = 0xFF4A;
pub const WINDOW_X_ADDR                 : u16 = 0xFF4B;

// Timing stuff
pub const CYCLES_PER_LINE               : u32 =  456;
pub const HBLANK_CYCLES                 : u32 =  204;
pub const VRAM_CYCLES                   : u32 =  172;
pub const OAM_CYCLES                    : u32 =   80;
pub const VBLANK_CYCLES                 : u32 = 4560;

pub const VBLANK_START_LINE             : u8  = 144;
pub const VBLANK_END_LINE               : u8  = 154;

// Relevant bits
// LCD Control Register
pub const B_LCD_DISPLAY_ENABLED         : u8 = 7;
pub const B_BG_WIN_TILE_DATA_SELECT     : u8 = 4;
pub const B_BG_TILE_MAP_SELECT          : u8 = 3;

// LCD Status Register
pub const B_LYC_COINCIDENCE_INTERRUPT   : u8 = 6;

// Palettes
pub const PALETTE_PINKU: [[u8; 4]; 4] = [
    [255, 158, 250, 255],
    [229, 137, 224, 255],
    [219, 127, 214, 255],
    [ 45,  11,  45, 255],
];
