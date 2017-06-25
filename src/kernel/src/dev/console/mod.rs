/*  Console Driver Module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ConsoleColor {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: ConsoleColor, background: ConsoleColor) -> ColorCode{
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const CONSOLE_WIDTH:usize = 80;
const CONSOLE_HEIGHT:usize = 25;

use volatile::Volatile;
struct ConsoleBuffer {    
    chars: [[Volatile<ScreenChar>; CONSOLE_WIDTH]; CONSOLE_HEIGHT],
}

use core::ptr::Unique;
pub struct Console{
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<ConsoleBuffer>,
}

impl Console{
    pub fn new(cc: ColorCode, addr: u64) -> Console {
        Console{
            column_position: 0,
            color_code: cc,
            buffer: unsafe{ Unique::new(addr as *mut _) },
        }
    }

    pub fn change_color_code(&mut self, cc: ColorCode){
        self.color_code = cc;
    }

     pub fn clear_screen(&mut self){
        for i in 0..CONSOLE_HEIGHT {
            for j in 0..CONSOLE_WIDTH {
                self.get_buffer().chars[i][j].write(ScreenChar{
                    ascii_character: b'\0',
                    color_code : ColorCode::new(ConsoleColor::Black, ConsoleColor::Black),
                });
            }
        }
     }

     pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.print_line_feed(),
            b'\r' => self.print_carriage_return(),
            b => {
                self.get_buffer().chars[CONSOLE_HEIGHT -1 ][self.column_position] = Volatile::new(ScreenChar{
                    ascii_character: b,
                    color_code: self.color_code,
                });
                self.column_position += 1;
                if self.column_position >= CONSOLE_WIDTH {
                    self.scroll_one_line();
                }
            }
        }
     }

     pub fn write_str(&mut self, s: &str) {
         for c in s.chars() {
            self.write_byte(c as u8);
         }
     }

     fn print_line_feed(&mut self){
        self.scroll_one_line();
     }

     fn print_carriage_return(&mut self){
        self.column_position = 0;
     }

     fn scroll_one_line(&mut self){
        for i in 1..CONSOLE_HEIGHT {
            for j in 0..CONSOLE_WIDTH {
                let data = self.get_buffer().chars[i][j].read();
                self.get_buffer().chars[i-1][j].write(data);
                self.get_buffer().chars[i][j].write(ScreenChar{
                    ascii_character: b'\0',
                    color_code : ColorCode::new(ConsoleColor::Black, ConsoleColor::Black),
                });
            }
        }
        self.print_carriage_return();
     }

     fn get_buffer(&mut self) -> &mut ConsoleBuffer {
         unsafe{self.buffer.as_mut()}
     }

}