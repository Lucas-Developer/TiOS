/*  Keyboard device driver
 *  Written by Andrew Jianzhong Liu
 *  All Rights Reserved
 */

use super::*;

pub static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});

pub fn init_kbd(){

}