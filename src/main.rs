#![allow(warnings)]
#![no_main]
#[macro_use]extern crate lazy_static;
extern crate winapi;
extern crate user32;
use winapi::{c_int,HINSTANCE,LPSTR};
use std::thread::sleep_ms;
mod ffi;
mod helpers;
mod window;
mod trayicon;
mod traymenu;
use trayicon::*;
use traymenu::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn WinMain(_: HINSTANCE, _: HINSTANCE, _: LPSTR, _: c_int) -> c_int {
    let trayicon = TrayIcon::new("My icon");
    trayicon.show();

    sleep_ms(60_000);

    trayicon.hide();
/*
    let menu = TrayMenu::new();
    menu.Add(TrayMenuItem::new("Test", ||{/*do something*/}));
    menu.Add(TrayMenuItem::new("Exit", DefaultHandlers::Exit));
    trayicon.menu = menu;
    trayicon.icon = Icon::new("c:/icon.ico");
*/
    0
}
