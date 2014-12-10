#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate ncurses;
use std::collections::TreeMap;
use std::collections::TreeSet;

pub mod window;

#[experimental]
pub struct NCRS {
    windows: TreeMap<String, window::Window>
}
    
#[experimental]
pub enum CBreakMode {
    On,
    Raw,
    Off
}

#[experimental]
pub enum Options {
    Echo(bool),
    CBreak(CBreakMode),
    Keypad(bool),
    Meta(bool),
    QiFlush(bool)
}

impl Drop for NCRS {
    /// Destroy the NCRS system when it goes out of scope
    #[stable]
    fn drop(&mut self) {
        self.windows.clear();
        info!("Destroying the NCRS system");
        ncurses::endwin();
    }
}

impl NCRS {
    /// Change options about how the user interface reacts to either rendering or input 
    pub fn option(self: &mut NCRS, option: Options) {
        match option {
            Options::Echo(true) => {
                ncurses::echo();
                debug!("Enabling Echo");
            },
            Options::Echo(false) => {
                ncurses::noecho();
                debug!("Disabling Echo");
            },
            Options::Keypad(true) => {
                ncurses::keypad(ncurses::stdscr, true);
                debug!("Enabling Keypad");
            },
            Options::Keypad(false) => {
                ncurses::keypad(ncurses::stdscr, false);
                debug!("Disabling Keypad");
            },
            Options::Meta(true) => {
                ncurses::meta(ncurses::stdscr, true);
                debug!("Enabling Meta");
            },
            Options::Meta(false) => {
                ncurses::meta(ncurses::stdscr, false);
                debug!("Disabling Meta");
            },
            Options::QiFlush(true) => {
                ncurses::qiflush();
                debug!("Enabling QiFlush");
            },
            Options::QiFlush(false) => {
                ncurses::noqiflush();
                debug!("Disabling QiFlush");
            },
            Options::CBreak(CBreakMode::On) => {
                ncurses::noraw();
                ncurses::cbreak();
                debug!("Setting CBreak Mode to On");
            },
            Options::CBreak(CBreakMode::Raw) => {
                ncurses::nocbreak();
                ncurses::raw();
                debug!("Setting CBreak Mode to Raw");
            },
            Options::CBreak(CBreakMode::Off) => {
                ncurses::nocbreak();
                ncurses::noraw();
                debug!("Setting CBreak Mode to Off");
            }
        }
    }
}

impl NCRS {
    #[experimental]
    /// Create a new NCRS system for rendering to the screen
    pub fn new() -> NCRS {
        info!("Creating a new NCRS system");
        ncurses::initscr();
        NCRS {
            windows: TreeMap::new()
        }
    }

    /// Get the width of the NCRS UI
    pub fn width(self: &NCRS) -> u16 {
        ncurses::COLS as u16
    }

    /// Get the height of the NCRS UI
    pub fn height(self: &NCRS) -> u16 {
        ncurses::LINES as u16
    }

    /// Create a new window and cause it to be displayed
    /// # Parameters:
    /// * name The internal name of the window
    /// * opts The options to use for creating the window
    /// # Returns The new window
    pub fn new_window<'a>(self: &mut NCRS, name: &'a str, opts: window::WindowOptions) -> Option<&mut window::Window> {
        info!("Creating new window called {}", name);
        self.windows.insert(name.to_string(), window::Window::new(opts));
        self.windows.get_mut(&name.to_string())
    }

    /// Get a window with the given name
    /// # Parameters:
    /// * name The internal name of the window
    /// # Returns the window, if it exists
    pub fn get_window<'a>(self: &NCRS, name: &'a str) -> Option<&window::Window> {
        self.windows.get(&name.to_string())
    }

    /// Get a window with the given name in a mutable manner
    /// # Parameters:
    /// * name The internal name of the window
    /// # Returns the window, if it exists
    pub fn get_mut_window<'a>(self: &mut NCRS, name: &'a str) -> Option<&mut window::Window> {
        self.windows.get_mut(&name.to_string())
    }

    /// Get a list of all the window names that exist
    /// # Returns the set of all window names
    pub fn get_window_names<'a>(self: &'a NCRS) -> TreeSet<&'a String> {
        let mut names = TreeSet::new();
        for name in self.windows.keys() {
            names.insert(name);
        }
        names
    }

    /// Cause the UI to be rendered to the screen
    pub fn render(self: &NCRS) {
        debug!("Rendering the UI");
        ncurses::clear();
        for window in self.windows.values() {
            window.render();
        }
        ncurses::doupdate();
    }
}
