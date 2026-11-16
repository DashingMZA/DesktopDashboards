#[cfg(windows)]
mod imp {
    use tauri::WebviewWindow;
    use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::Shell::{DefSubclassProc, SetWindowSubclass};
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM,
        SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, WINDOWPOS,
        WM_NCDESTROY, WM_WINDOWPOSCHANGING, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
    };

    const SUBCLASS_ID_BOTTOM: usize = 0x4444_0001;

    unsafe extern "system" fn bottom_subclass_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        uid_subclass: usize,
        _dw_ref_data: usize,
    ) -> LRESULT {
        if msg == WM_WINDOWPOSCHANGING && lparam.0 != 0 {
            let wp = &mut *(lparam.0 as *mut WINDOWPOS);
            if (wp.flags & SWP_NOZORDER) == windows::Win32::UI::WindowsAndMessaging::SET_WINDOW_POS_FLAGS(0) {
                wp.hwndInsertAfter = HWND_BOTTOM;
            }
        } else if msg == WM_NCDESTROY {
            let _ = windows::Win32::UI::Shell::RemoveWindowSubclass(
                hwnd,
                Some(bottom_subclass_proc),
                uid_subclass,
            );
        }
        DefSubclassProc(hwnd, msg, wparam, lparam)
    }

    /// ガジェットウィンドウに `WS_EX_TOOLWINDOW`（Alt+Tab 除外）と最背面固定（`HWND_BOTTOM`）を適用する。
    pub fn apply_bottom_mode(window: &WebviewWindow) -> Result<(), String> {
        let hwnd = window
            .hwnd()
            .map_err(|e| format!("failed to get HWND: {e}"))?;
        let hwnd = HWND(hwnd.0);

        unsafe {
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let new_style =
                (ex_style & !(WS_EX_APPWINDOW.0 as isize)) | (WS_EX_TOOLWINDOW.0 as isize);
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);

            let _ = SetWindowSubclass(hwnd, Some(bottom_subclass_proc), SUBCLASS_ID_BOTTOM, 0);

            SetWindowPos(
                hwnd,
                Some(HWND_BOTTOM),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED,
            )
            .map_err(|e| format!("SetWindowPos failed: {e}"))?;
        }

        Ok(())
    }
}

#[cfg(not(windows))]
mod imp {
    use tauri::WebviewWindow;

    pub fn apply_bottom_mode(_window: &WebviewWindow) -> Result<(), String> {
        Ok(())
    }
}

pub use imp::apply_bottom_mode;
