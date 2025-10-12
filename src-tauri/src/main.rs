// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use spacepix_lib::Parser;


fn main() {
    let parser = Parser::default();
    println!("{}", parser.apod_url());
    println!("{}", parser.neows_url("2025-10-10"));
    spacepix_lib::run();
}
