#![no_std]

use gstd::{debug, exec, msg};
use pebbles_game_io::*;

static mut GAME_CONTEXT: Option<GameState> = None;



#[no_mangle]
extern "C" fn init() {
 
}

#[no_mangle]
extern "C" fn handle() {

}

#[no_mangle]
extern "C" fn state() {

}
