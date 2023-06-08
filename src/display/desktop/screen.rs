//! Hardware display information.


/// Contains list of all hardware display device.
pub struct ScreenList {
    /// Screens combined width
    width : u32,

    /// Screens combined height
    height : u32,

    /// List of screens
    screen_list : Vec<Screen>,
}

impl ScreenList {
    /// Create a screen list from combined resolution and vector of screen.
    pub(crate) fn create(size : (u32,u32), screen_list : Vec<Screen>) -> ScreenList{
        ScreenList{ width: size.0, height: size.1, screen_list }
    }

    /// Returns desktop multi-screen combined width.
    pub fn get_desktop_width(&self) -> u32 {
        self.width
    }

    /// Returns desktop multi-screen combined height.
    pub fn get_desktop_height(&self) -> u32 {
        self.height
    }

    /// Get primary screen reference.
    /// 
    /// Returns Some([Screen]) with primary screen or None if no screen.
    pub fn get_primary_screen(&self) -> Option<&Screen> {
        for screen in &self.screen_list {
            if screen.is_primary() {
                return Some(screen);
            }
        }
        None
    }

    /// Get a reference to the list of screens.
    pub fn get_screen_list(&self) -> &Vec<Screen> {
        &self.screen_list
    }

}


/// Hardware display device details.
/// 
/// # Note(s)
/// Refresh rate is stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
pub struct Screen {
    /// Identifier of that screen
    identifier : String,

    /// Current position of that screen on extended desktop
    position : (i32, i32),

    /// Current resolution of that screen as (width, height)
    resolution : (u32, u32),

    /// Current refresh rate of that screen
    refresh_rate : u32,

    /// Is primary screen?
    primary : bool,

    /// Supported resolutions
    supported : Vec<ScreenResolution>,
}

impl Screen {
    /// Create a new [Screen] with fields.
    pub fn new(identifier : String, position : (i32,i32), resolution : (u32, u32), refresh_rate : u32, primary : bool, supported : Vec<ScreenResolution>) -> Screen{
        Screen { identifier, position, resolution, refresh_rate, primary, supported }
    }

    /// Returns screen unique identifier as [String].
    pub fn get_identifier(&self) -> &String{
        &self.identifier
    }

    /// Returns current resolution of screen as (width, height)
    pub fn get_current_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    /// Returns extended position of screen in extended desktop as (x, y)
    pub fn get_extended_position(&self) -> (i32, i32) {
        self.position
    }

    /// Returns current refresh rate of screen.
    /// 
    /// Note(s)
    /// Refresh rate is stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
    pub fn get_current_refresh_rate(&self) -> u32 {
        self.refresh_rate
    }

    /// Returns True if screen is currently the primare default screen.
    pub fn is_primary(&self) -> bool{
        self.primary
    }

    /// Returns list of supported resolutions and refresh rates.
    pub fn get_supported_resolutions(&self) -> &Vec<ScreenResolution>{
        &self.supported
    }
}

/// Hardware screen supported resolution with available refresh rate.
/// 
/// # Note(s)
/// Refresh rate is stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
#[derive(Clone)]
pub struct ScreenResolution {
    width : u32,
    height : u32,
    refresh_rate : Vec<u32>,
}

impl ScreenResolution {
    /// Create a new [ScreenResolution] with fields.
    pub fn new(width : u32, height : u32) -> ScreenResolution {
        ScreenResolution { width, height, refresh_rate: Vec::new() }
    }

    /// Add a supported refresh rate to resolution.
    pub(crate) fn add_refresh_rate(&mut self, rate : u32){

        // Don't add if already exists
        for rf in &self.refresh_rate {
            if *rf == rate {
                return;
            }
        }

        self.refresh_rate.push(rate);
    }

    /// Returns width of supported screen resolution.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Returns height of supported screen resolution.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Returns supported refresh rates of this screen resolution.
    /// 
    /// # Note(s)
    /// Refresh rates are stored as unsigned integer. A 60hz refresh rate is 6000 and a 144hz is 14400. 
    pub fn get_refresh_rates(&self) -> &Vec<u32> {
        &self.refresh_rate
    }

   
}
