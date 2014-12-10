extern crate ncurses;

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
    /// The options for this window
    pub options: WindowOptions,
    /// The underlying NCurses Window object
    ncurses_window: ncurses::WINDOW
}


impl Drop for Window {
    /// Destroy a particular window
    #[stable]
    fn drop(&mut self) {
        info!("Destroying window");
        ncurses::delwin(self.ncurses_window);
    }
}

impl Window {
    /// Create a new Window according to the provided options
    pub fn new(opts: WindowOptions) -> Window {
        let win = ncurses::derwin(ncurses::stdscr, opts.height as i32, opts.width as i32, opts.y as i32, opts.x as i32);
        Window {
            options: opts,
            ncurses_window: win
        }
    }

    /// Cause the Window to be rendered to the screen
    pub fn render(self: &Window) {
        ncurses::wborder(self.ncurses_window, 0, 0, 0, 0, 0, 0, 0, 0);
    }
}