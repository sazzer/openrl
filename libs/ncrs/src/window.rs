/// The options to be used to create a window
#[deriving(Default, Show)]
pub struct WindowOptions {
    /// The X-Ordinate of the window
    pub x: u16,
    /// The Y-Ordinate of the window
    pub y: u16,
    /// The width of the window
    pub width: u16,
    /// The height of the window
    pub height: u16,
    /// The optional title of the window
    pub title: Option<String>
}

/// Representation of a single Window in the NCRS UI
pub struct Window {
    pub options: WindowOptions
}


impl Drop for Window {
    /// Destroy a particular window
    #[stable]
    fn drop(&mut self) {
        info!("Destroying window");
    }
}

impl Window {
    /// Cause the Window to be rendered to the screen
    pub fn render(self: &Window) {
    }
}