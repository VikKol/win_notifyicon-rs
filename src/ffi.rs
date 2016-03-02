extern crate winapi;
extern crate user32;
use winapi::winnt::{LPCSTR,LPCWSTR};
use winapi::minwindef::{UINT,BOOL};
use winapi::windef::{HWND,HMENU,HICON,RECT};

pub const NIM_ADD: u32 = 0x00000000;
pub const NIM_DELETE: u32 = 0x00000002;
pub const NIF_MESSAGE: u32 = 0x00000001;
pub const NIF_ICON: u32 = 0x00000002;
pub const NIF_TIP: u32 = 0x00000004;

pub const MF_STRING: u32 = 0x00000000;
pub const MF_BYPOSITION: u32 = 0x00000400;

pub const TPM_LEFTALIGN: u32 = 0x0000;
pub const TPM_NONOTIFY: u32 = 0x0080;
pub const TPM_RETURNCMD: u32 = 0x0100;
pub const TPM_RIGHTBUTTON: u32 = 0x0002;

#[repr(C)]
#[allow(non_snake_case)]
pub struct NOTIFYICONDATA {
    pub cbSize: i32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: LPCWSTR,
    pub dwState: i32,
    pub dwStateMask: i32,
    pub szInfo: LPCWSTR,
    pub uVersion: u32,
    pub szInfoTitle: LPCWSTR,
    pub dwInfoFlags: i32
}

#[allow(non_snake_case)]
extern "system" {
    pub fn Shell_NotifyIcon(dwMessage: u32, pnid: &mut NOTIFYICONDATA) -> BOOL;
    pub fn Shell_NotifyIconA(dwMessage: u32, pnid: &mut NOTIFYICONDATA) -> BOOL;
    pub fn Shell_NotifyIconW(dwMessage: u32, pnid: &mut NOTIFYICONDATA) -> BOOL;
    pub fn TrackPopupMenu(
        h_menu: HMENU,
        u_flags: UINT,
        x: i32,
        y: i32,
        n_reserved: i32,
        h_wnd: HWND,
        prc_rect: *const RECT
    ) -> BOOL;
}
