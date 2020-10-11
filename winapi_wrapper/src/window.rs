use winapi::ctypes::c_int;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{self, CS_HREDRAW, CS_OWNDC, CS_VREDRAW};

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

const ZERO: u16 = 0;
///A handle to a window.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Window {
    hwnd: HWND,
}
impl From<Level> for HWND {
    fn from(l: Level) -> Self {
        match l {
            Level::TopMost => winuser::HWND_TOPMOST,
            Level::Top => winuser::HWND_TOP,
            Level::Normal => winuser::HWND_NOTOPMOST,
            Level::Bottom => winuser::HWND_BOTTOM,
        }
    }
}
///Represents a Window Position.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WindowPos {
    x: c_int,
    y: c_int,
    cx: c_int,
    cy: c_int,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Level {
    ///This window will *always* remain at the top, unless otherwise specified.
    TopMost,
    ///Put to top of Z order.
    Top,
    ///Top of all non-topmost or top windows.
    Normal,
    ///Bottom of Z order.
    Bottom,
}

impl Window {
    pub fn new(
        name: &str,
        size_x: c_int,
        size_y: c_int,
        pos_x: c_int,
        pos_y: c_int,
    ) -> Result<Self, crate::WinApiError> {
        let name = win32_string(name);
        unsafe {
            //This is going to be a pointer handed by windows, should be valid as long as the program is.
            //It represents a handle to the current program.
            let hinstance = winapi::um::libloaderapi::GetModuleHandleW(ZERO as *const u16);
            let wnd_class = winapi::um::winuser::WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
                lpfnWndProc: Some(winuser::DefWindowProcW),
                hInstance: hinstance,
                lpszClassName: name.as_ptr(),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: std::ptr::null_mut(),
                hCursor: std::ptr::null_mut(),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: std::ptr::null_mut(),
            };
            //Register the window's settings.
            winuser::RegisterClassW(&wnd_class);
            //Create the window.
            let handle = winuser::CreateWindowExW(
                0,
                name.as_ptr(),
                std::ptr::null_mut(),
                winuser::WS_VISIBLE,
                pos_x,
                pos_y,
                size_x,
                size_y,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                hinstance,
                std::ptr::null_mut(),
            );
            //If it is null, some error happened.
            if handle.is_null() {
                let last_err = winapi::um::errhandlingapi::GetLastError();
                panic!("Got a null handle. error num: {}", last_err);
            }
            if winuser::SetWindowLongW(handle, winuser::GWL_EXSTYLE, winuser::WS_EX_LAYERED as i32)
                == 0
            {
                return Err(crate::WinApiError::FailedToSetExtendedWinStyle);
            };
            Ok(Self { hwnd: handle })
        }
    }
    pub fn remove_window_styling(&self) -> Result<(), crate::WinApiError> {
        unsafe {
            if winuser::SetWindowLongW(self.hwnd, winuser::GWL_STYLE, 0) == 0 {
                return Err(crate::WinApiError::FailedToSetGWL_Style);
            };
            Ok(())
        }
    }
    pub fn set_level(&self, level: Level) -> Result<(), crate::WinApiError> {
        let window_pos = self.get_window_pos()?;
        unsafe {
            if winuser::SetWindowPos(
                self.hwnd,
                level.into(),
                window_pos.x,
                window_pos.y,
                window_pos.cx,
                window_pos.cy,
                0,
            ) == 0
            {
                return Err(crate::WinApiError::FailedToSetWindowPos(
                    window_pos.x,
                    window_pos.y,
                    window_pos.cx,
                    window_pos.cy,
                    level.into(),
                ));
            }
        }
        Ok(())
    }
    
    pub fn set_window_color(&self, color: crate::util::RGBA) -> Result<(), crate::WinApiError> {
        unsafe {
            let (rgb, alpha) = color.into();
            if winuser::SetLayeredWindowAttributes(self.hwnd, rgb, alpha, 1) == 0 {
                return Err(crate::WinApiError::CouldntSetBgColor);
            }
        }
        Ok(())
    }

    pub fn get_window_pos(&self) -> Result<WindowPos, crate::WinApiError> {
        let mut rect = winapi::shared::windef::RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        unsafe {
            if winuser::GetWindowRect(self.hwnd, &mut rect as *mut winapi::shared::windef::RECT)
                == 0
            {
                return Err(crate::WinApiError::FailedToGetWindowPos);
            }
        }
        Ok(WindowPos {
            x: rect.left,
            y: rect.top,
            cx: rect.right - rect.left,
            cy: rect.bottom - rect.top,
        })
    }
}

fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
