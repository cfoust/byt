//! byt - os_unix
//!
//! Implement functions by targeting the terminal functionality
//! commonly offered by unix terminals.

// EXTERNS

// LIBRARY INCLUDES
use std::io::{self, Write};
use byt::libc::{
    self,
    ioctl,
    c_void,
    TIOCGWINSZ,
    winsize
};
use std::mem;

// SUBMODULES

// LOCAL INCLUDES
use byt::envs::*;

// Constants for manipulating the terminal.
const CMD_LEAD           : &str = "\x1B[";  // Leader for all commands.
const CMD_CLEAR          : &str = "2J";     // Clears the display.
const CMD_NORM           : &str = "?47l";   // Change to normal buffer
const CMD_ALT            : &str = "?47h";   // Change to alternate buffer
const CMD_SAVE_CURSOR    : &str = "6";      // Save the cursor's position.
const CMD_RESTORE_CURSOR : &str = "7";      // Restore the cursor's position.

pub struct Term {
    saved_config : libc::termios,
}

impl Term {
    // #################################
    // P R I V A T E  F U N C T I O N S 
    // #################################
    /// Print a command to the terminal.
    fn cmd(&self, command : &str) {
        let mut handle = io::stdout();
        handle.write(CMD_LEAD.as_bytes())
              .expect("Writing command leader to stdout failed");
        handle.write(command.as_bytes())
              .expect("Writing command to stdout failed");
        handle.flush()
              .expect("Flushing stdout failed");
    }

    /// Set the terminal to the alternate full screen buffer.
    fn set_alternate(&self) {
        self.cmd(CMD_ALT);
    }

    /// Set the terminal back to the normal buffer.
    fn set_normal(&self) {
        self.cmd(CMD_NORM);
    }

    /// Save the cursor position.
    fn save_cursor(&self) {
        self.cmd(CMD_SAVE_CURSOR);
    }

    /// Restore the cursor position.
    fn restore_cursor(&self) {
        self.cmd(CMD_RESTORE_CURSOR);
    }

    /// Get the current termios config.
    fn get_mode() -> libc::termios {
        unsafe {
            let mut temp : libc::termios = mem::zeroed();
            libc::tcgetattr(0, &mut temp);
            temp
        }
    }

    // ###############################
    // P U B L I C  F U N C T I O N S 
    // ###############################
    pub fn new() -> Term {
        Term {
            saved_config : Term::get_mode(),
        }
    }

    /// Open the application.
    pub fn start(&self) {
        self.save_cursor();
        self.set_alternate();
    }

    /// Close the application.
    pub fn stop(&self) {
        self.set_normal();
        self.restore_cursor();
    }


    /// Get the size of the terminal in rows and columns.
    pub fn get_size(&self) -> (u16, u16) {
        let (rows, cols);

        // Get the size of the terminal by calling ioctl.
        unsafe {
            let mut ws = winsize {
                ws_row: 0,
                ws_col: 0,
                ws_xpixel: 0,
                ws_ypixel: 0,
            };

            // Grab a pointer to the struct.
            let ptr : *mut c_void = &mut ws as *mut _ as *mut c_void;

            // Run ioctl to check the window size.
            // TODO: If this fails, try TIOCGSIZE.
            let result = ioctl(1, TIOCGWINSZ, ptr);

            match result {
                0 => {rows = ws.ws_row; cols = ws.ws_col},
                _ => panic!("Grabbing terminal size from ioctl failed")
            }
        }

        (rows, cols)
    }

    /// Move the terminal's cursor to a particular location.
    ///
    /// Takes input in (row, col).
    pub fn move_cursor(&self, row : u16, col : u16) {
        self.cmd(format!("{};{}H", row, col).as_str());
    }

    /// Clear the terminal's screen.
    pub fn clear_screen(&self) {
        self.cmd(CMD_CLEAR);
    }

    /// Write a string to the terminal at the cursor.
    pub fn write(&self, out : &str) {
        let mut handle = io::stdout();
        handle.write(out.as_bytes())
              .expect("Failed to write to stdout");
        handle.flush()
              .expect("Flushing to stdout failed");
    }
    /// Set the terminal's mode between `Raw` and `Cooked`
    /// modes.
    pub fn set_mode(&self, mode : TermMode) {
        let mut copy = self.saved_config;
        match mode {
            TermMode::Cooked => {
                // Don't do anything, just set it back to normal
            },
            TermMode::Raw => {
                // Most of this is borrowed from:
                // http://www.cs.uleth.ca/~holzmann/C/system/ttyraw.c
                copy.c_iflag &=
                    !(libc::BRKINT | // Turn off line break
                      libc::ICRNL  | // Don't check for parity
                      libc::INPCK  | // Don't strip characters
                      libc::ISTRIP |
                      libc::IXON);

                // give full 8 bits
                copy.c_cflag |= libc::CS8;

                // Also fix local modes
                copy.c_lflag &= !(libc::ECHO |
                                  libc::ICANON |
                                  libc::IEXTEN |
                                  libc::ISIG);

                // We only want to interpret one byte at a time
                copy.c_cc[libc::VMIN] = 1;

                // Don't wait for anything
                copy.c_cc[libc::VTIME] = 0;
            }
        }

        unsafe {
            libc::tcsetattr(0, libc::TCSAFLUSH, &copy);
        }
    }
}
