extern crate winapi;
extern crate user32;
extern crate lazy_static;
use winapi::windef::{HWND,RECT,POINT,HMENU};
use winapi::minwindef::{UINT,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{WNDPROC};
use std::sync::atomic::{AtomicBool,Ordering,AtomicPtr};
use ffi::*;

mod window;
use window::*;
use helpers;

const TRAYICON_ID: u32 = 1;
const EXIT_CMD_PARAM: WPARAM = 123;
const EXIT_MENU_ITEM_ID: u64 = 2000;
/*
static mut INITIALIZED: bool = false;
static mut MENU_PTR: Option<AtomicPtr<HMENU>> = None;
static mut MENU_LOCK: Option<AtomicBool> = None;
static mut APPSETTINGS: Option<AppSettings> = None;
MENU_LOCK = Some(AtomicBool::new(false));
MENU_PTR = Some(AtomicPtr::new(0 as *mut HMENU));
lazy_static! {
    static ref SETTINGS: AppSettings = {
        unsafe { APPSETTINGS.as_ref().unwrap().clone() }
    };
}
*/

//static menu_handlers = HashSet<guid, HashSet<name, handler>>;

pub struct TrayIcon {
    tip: 'static str,
    menu: TrayMenu,
    wnd_handle: HANDLE
}

impl TrayIcon {
    pub fn new('static str tip) -> Self {
        let guid = generate_guid();
        let w_handle = window::create_background(guid.to_string(), Some(window_proc));
        TrayIcon {
            tip: tip,

        }
    }

    pub fn show(&self) {
        self.register_trayicon();
        thread::spawn(||{
            let mut msg = window::create_window_msg();
            unsafe {
                while self.active && user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
                    user32::TranslateMessage(&mut msg);
                    user32::DispatchMessageW(&mut msg);
                }
            }
        });
    }

    pub fn hide(&self) {
        self.active = false;
        self.unregister_trayicon();
    }

    fn register_trayicon(&self) {
        let tip_v: Vec<char> = self.tip.chars()
            .chain(Some(0 as char).into_iter())
            .collect();
        let mut nid = NOTIFYICONDATA {
            cbSize: size_of::<NOTIFYICONDATA>() as i32,
            hWnd: self.wnd_handle,
            uID: TRAYICON_ID,
            uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
            uCallbackMessage: winapi::winuser::WM_APP,
            hIcon: unsafe { user32::LoadIconW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION) },
            szTip: tip_v.as_ptr() as * const _,
            dwState: 0,
            dwStateMask: 0,
            szInfo: 0 as *const _,
            uVersion: 0,
            szInfoTitle: 0 as *const _,
            dwInfoFlags: 0
        };
        unsafe { Shell_NotifyIcon(NIM_ADD, &mut nid); }
    }

    fn unregister_trayicon(&self) {
        let mut nid = NOTIFYICONDATA {
            cbSize: size_of::<NOTIFYICONDATA>() as i32,
            hWnd: self.wnd_handle,
            uID: TRAYICON_ID,
            uFlags: 0,
            uCallbackMessage: winapi::winuser::WM_APP,
            hIcon: 0 as HICON,
            szTip: 0 as *const _,
            dwState: 0,
            dwStateMask: 0,
            szInfo: 0 as *const _,
            uVersion: 0,
            szInfoTitle: 0 as *const _,
            dwInfoFlags: 0
        };
        unsafe { Shell_NotifyIcon(NIM_DELETE, &mut nid); }
    }
}

unsafe extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        winapi::winuser::WM_APP if l_param == (winapi::winuser::WM_LBUTTONUP as i64) => {
            let menu_locked = MENU_LOCK.as_ref().unwrap().load(Ordering::Relaxed);
            if menu_locked {
                let h_pop = MENU_PTR.as_ref().unwrap().load(Ordering::Relaxed);
                user32::DestroyMenu(*h_pop);
            }
        },
        winapi::winuser::WM_APP if l_param == (winapi::winuser::WM_RBUTTONUP as i64) => {
            handle_popup_menu(h_wnd);
        },
        winapi::winuser::WM_COMMAND if w_param == EXIT_CMD_PARAM => {
            user32::DestroyWindow(h_wnd);
        },
        winapi::winuser::WM_CLOSE => { user32::DestroyWindow(h_wnd); },
        winapi::winuser::WM_DESTROY => { user32::PostQuitMessage(0); },
        _ => return user32::DefWindowProcW(h_wnd, msg, w_param, l_param)
    };
    0
}

unsafe fn handle_popup_menu(h_wnd: HWND) {
    let mut h_pop = user32::CreatePopupMenu();
    MENU_PTR.as_ref().unwrap().store(&mut h_pop, Ordering::Relaxed);
    MENU_LOCK.as_ref().unwrap().store(true, Ordering::Relaxed);
    user32::InsertMenuW(
        h_pop,
        0,
        MF_BYPOSITION|MF_STRING, EXIT_MENU_ITEM_ID,
        helpers::to_wstring(&"Exit"));
    user32::SendMessageW(h_wnd, winapi::winuser::WM_INITMENUPOPUP, h_pop as WPARAM, 0);

    let mut p: POINT = POINT { x: 0, y: 0 };
    user32::GetCursorPos(&mut p);

    let cmd_performed = TrackPopupMenu(
        h_pop,
        TPM_LEFTALIGN|TPM_RIGHTBUTTON|TPM_RETURNCMD|TPM_NONOTIFY,
        p.x,
        p.y,
        0,
        h_wnd,
        0 as *const RECT);
    if cmd_performed > 0 {
        MENU_LOCK.as_ref().unwrap().store(false, Ordering::Relaxed);
        user32::SendMessageW(h_wnd, winapi::winuser::WM_COMMAND, EXIT_CMD_PARAM, 0);
    }
    user32::DestroyMenu(h_pop);
}
