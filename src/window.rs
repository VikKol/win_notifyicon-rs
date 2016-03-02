extern crate winapi;
extern crate user32;
extern crate kernel32;
use winapi::winnt::LPCWSTR;
use winapi::windef::{HWND,HMENU,HBRUSH};
use winapi::minwindef::{HINSTANCE,UINT,DWORD,WPARAM,LPARAM};
use winapi::winuser::{WNDPROC,CW_USEDEFAULT,WS_OVERLAPPEDWINDOW,WS_VISIBLE,WNDCLASSW};
use std::ptr::{null_mut};
use helpers;

pub fn create_background(title: String, window_proc: WNDPROC) -> HWND {
    create_window(title, false, 0, 0, window_proc)
}

pub fn create_window(
    title: String,
    visible: bool,
    width: i32,
    height: i32,
    window_proc: WNDPROC)
    -> HWND {
    let class_name = "window_".to_string() + &title;
    let w_class_name = helpers::to_wstring(&class_name);
    let wnd = WNDCLASSW {
        style: 0,
        lpfnWndProc: window_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: 0 as HINSTANCE,
        hIcon: unsafe { user32::LoadIconW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION) },
        hCursor: unsafe { user32::LoadCursorW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION) },
        hbrBackground: 16 as HBRUSH,
        lpszMenuName: 0 as LPCWSTR,
        lpszClassName: w_class_name,
    };

    unsafe { user32::RegisterClassW(&wnd) };

    // Create window
    let h_wnd_desktop = unsafe { user32::GetDesktopWindow() };
    let window_flags = if visible { WS_OVERLAPPEDWINDOW | WS_VISIBLE } else { WS_OVERLAPPEDWINDOW };
    unsafe {
        user32::CreateWindowExW(
            0,
            helpers::to_wstring(&class_name),
            helpers::to_wstring(&title),
            window_flags,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width, height,
            h_wnd_desktop,
            0 as HMENU,
            0 as HINSTANCE,
            null_mut())
    }
}

pub fn create_window_msg() -> winapi::winuser::MSG {
    winapi::winuser::MSG {
        hwnd : 0 as HWND,
        message : 0 as UINT,
        wParam : 0 as WPARAM,
        lParam : 0 as LPARAM,
        time : 0 as DWORD,
        pt : winapi::windef::POINT { x: 0, y: 0, }
    }
}
