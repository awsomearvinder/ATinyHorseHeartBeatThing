use std::{error::Error, fmt::Display};
use winapi::ctypes::c_int;

#[derive(Debug, Eq, PartialEq)]
pub enum WinApiError {
    FailedToSetWindowPos(c_int, c_int, c_int, c_int, crate::window::Level),
    FailedToGetWindowPos,
    FailedToSetExtendedWinStyle,
    CouldntSetBgColor,
    FailedToSetGWL_Style,
}

impl Display for WinApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToSetWindowPos(x, y, cx, cy, level) => write!(
                f,
                "Failed to write window pos ({},{}) with size {}x{} on level {:?}",
                x, y, cx, cy, level
            ),
            Self::FailedToGetWindowPos => write!(f, "Failed to get window position"),
            Self::FailedToSetExtendedWinStyle => write!(f, "Failed to set window style"),
            Self::CouldntSetBgColor => write!(f, "Failed to set Bg Color"),
            Self::FailedToSetGWL_Style => write!(f, "failed to set GWL_Style"),
        }
    }
}

impl Error for WinApiError {}
