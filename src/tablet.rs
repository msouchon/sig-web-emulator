#[derive(Default)]
pub struct Tablet {
    pub display_x_size: u32,
    pub display_y_size: u32,
    pub justify_mode: JustifyMode,
    pub state: bool,
    pub total_points: u32,
}

#[derive(Default)]
#[repr(u8)]
pub enum JustifyMode {
    #[default]
    None = 0,
    TopLeft = 1,
    TopRight = 2,
    BottomLeft = 3,
    BottomRight = 4,
    Center = 5,
}

