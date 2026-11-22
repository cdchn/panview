#![windows_subsystem = "windows"]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::mem::{size_of, zeroed};
use std::path::PathBuf;
use std::ptr::null;

type HWND = isize;
type HMENU = isize;
type HTHUMBNAIL = isize;
type BOOL = i32;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct POINT {
    x: i32,
    y: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct SIZE {
    cx: i32,
    cy: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct RECT {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C, packed(1))]
#[derive(Clone, Copy)]
struct DWM_THUMBNAIL_PROPERTIES {
    dw_flags: u32,
    rc_destination: RECT,
    rc_source: RECT,
    opacity: u8,
    f_visible: BOOL,
    f_source_client_area_only: BOOL,
}

#[repr(C)]
struct WNDCLASSEXW {
    cb_size: u32,
    style: u32,
    lpfn_wnd_proc: Option<unsafe extern "system" fn(HWND, u32, usize, isize) -> isize>,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: isize,
    h_icon: isize,
    h_cursor: isize,
    hbr_background: isize,
    lpsz_menu_name: *const u16,
    lpsz_class_name: *const u16,
    h_icon_sm: isize,
}

#[repr(C)]
struct MSG {
    hwnd: HWND,
    message: u32,
    w_param: usize,
    l_param: isize,
    time: u32,
    pt: POINT,
}

#[repr(C)]
struct PAINTSTRUCT {
    hdc: isize,
    f_erase: BOOL,
    rc_paint: RECT,
    f_restore: BOOL,
    f_inc_update: BOOL,
    rgb_reserved: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct GUID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct NOTIFYICONDATAW {
    cb_size: u32,
    hwnd: HWND,
    u_id: u32,
    u_flags: u32,
    u_callback_message: u32,
    h_icon: isize,
    sz_tip: [u16; 128],
    dw_state: u32,
    dw_state_mask: u32,
    sz_info: [u16; 256],
    u_timeout_or_version: u32,
    sz_info_title: [u16; 64],
    dw_info_flags: u32,
    guid_item: GUID,
    h_balloon_icon: isize,
}

const WS_OVERLAPPEDWINDOW: u32 = 0x00CF0000;
const WS_POPUP: u32 = 0x80000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_CHILD: u32 = 0x40000000;
const WS_BORDER: u32 = 0x00800000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_EX_TOPMOST: u32 = 0x00000008;
const WS_EX_TRANSPARENT: u32 = 0x00000020;
const WS_EX_TOOLWINDOW: u32 = 0x00000080;
const WS_EX_LAYERED: u32 = 0x00080000;

const SW_SHOW: i32 = 5;
const SW_MINIMIZE: i32 = 6;
const SW_RESTORE: i32 = 9;

const WM_DESTROY: u32 = 0x0002;
const WM_SIZE: u32 = 0x0005;
const WM_PAINT: u32 = 0x000F;
const WM_CLOSE: u32 = 0x0010;
const WM_SETCURSOR: u32 = 0x0020;
const WM_NCHITTEST: u32 = 0x0084;
const WM_NCLBUTTONDOWN: u32 = 0x00A1;
const WM_KEYDOWN: u32 = 0x0100;
const WM_COMMAND: u32 = 0x0111;
const WM_MOUSEMOVE: u32 = 0x0200;
const WM_LBUTTONDOWN: u32 = 0x0201;
const WM_LBUTTONUP: u32 = 0x0202;
const WM_RBUTTONUP: u32 = 0x0205;
const WM_MBUTTONDOWN: u32 = 0x0207;
const WM_MBUTTONUP: u32 = 0x0208;
const WM_MOUSEWHEEL: u32 = 0x020A;
const WM_SIZING: u32 = 0x0214;
const WM_TRAY: u32 = 0x8001;

const HTCLIENT: isize = 1;
const HTCAPTION: isize = 2;
const HTLEFT: isize = 10;
const HTRIGHT: isize = 11;
const HTTOP: isize = 12;
const HTTOPLEFT: isize = 13;
const HTTOPRIGHT: isize = 14;
const HTBOTTOM: isize = 15;
const HTBOTTOMLEFT: isize = 16;
const HTBOTTOMRIGHT: isize = 17;

const VK_RETURN: usize = 0x0D;
const VK_CONTROL: i32 = 0x11;
const VK_SHIFT: i32 = 0x10;
const VK_ESCAPE: usize = 0x1B;
const VK_LEFT: usize = 0x25;
const VK_UP: usize = 0x26;
const VK_RIGHT: usize = 0x27;
const VK_DOWN: usize = 0x28;

const DWM_TNP_RECTDESTINATION: u32 = 0x01;
const DWM_TNP_RECTSOURCE: u32 = 0x02;
const DWM_TNP_OPACITY: u32 = 0x04;
const DWM_TNP_VISIBLE: u32 = 0x08;
const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 0x10;
const DWMWA_CLOAKED: u32 = 14;

const GWL_STYLE: i32 = -16;
const GWL_EXSTYLE: i32 = -20;
const GWLP_USERDATA: i32 = -21;
const HWND_TOPMOST: HWND = -1;
const HWND_NOTOPMOST: HWND = -2;
const SWP_NOSIZE: u32 = 0x0001;
const SWP_NOMOVE: u32 = 0x0002;
const SWP_NOZORDER: u32 = 0x0004;
const SWP_NOACTIVATE: u32 = 0x0010;
const SWP_FRAMECHANGED: u32 = 0x0020;
const SWP_SHOWWINDOW: u32 = 0x0040;
const SWP_HIDEWINDOW: u32 = 0x0080;
const LWA_ALPHA: u32 = 0x02;

const MF_STRING: u32 = 0x0000;
const MF_CHECKED: u32 = 0x0008;
const MF_POPUP: u32 = 0x0010;
const MF_GRAYED: u32 = 0x0001;
const MF_SEPARATOR: u32 = 0x0800;
const TPM_RIGHTBUTTON: u32 = 0x0002;
const TPM_RETURNCMD: u32 = 0x0100;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(lp_module_name: *const u16) -> isize;
}

#[link(name = "dwmapi")]
extern "system" {
    fn DwmRegisterThumbnail(dest: HWND, src: HWND, thumb: *mut HTHUMBNAIL) -> i32;
    fn DwmUnregisterThumbnail(thumb: HTHUMBNAIL) -> i32;
    fn DwmQueryThumbnailSourceSize(thumb: HTHUMBNAIL, size: *mut SIZE) -> i32;
    fn DwmUpdateThumbnailProperties(thumb: HTHUMBNAIL, props: *const DWM_THUMBNAIL_PROPERTIES) -> i32;
    fn DwmGetWindowAttribute(hwnd: HWND, attr: u32, data: *mut u32, size: u32) -> i32;
}

#[link(name = "gdi32")]
extern "system" {
    fn CreateSolidBrush(color: u32) -> isize;
    fn DeleteObject(obj: isize) -> BOOL;
    fn SetBkMode(hdc: isize, mode: i32) -> i32;
    fn SetTextColor(hdc: isize, color: u32) -> u32;
    fn GetStockObject(i: i32) -> isize;
}

#[link(name = "shell32")]
extern "system" {
    fn Shell_NotifyIconW(msg: u32, data: *const NOTIFYICONDATAW) -> BOOL;
}

#[link(name = "user32")]
extern "system" {
    fn SetProcessDpiAwarenessContext(value: isize) -> BOOL;
    fn RegisterClassExW(wc: *const WNDCLASSEXW) -> u16;
    fn CreateWindowExW(
        ex_style: u32, class: *const u16, title: *const u16, style: u32,
        x: i32, y: i32, w: i32, h: i32, parent: HWND, menu: HMENU, inst: isize, param: *const (),
    ) -> HWND;
    fn DestroyWindow(hwnd: HWND) -> BOOL;
    fn ShowWindow(hwnd: HWND, cmd: i32) -> BOOL;
    fn SetWindowPos(hwnd: HWND, after: HWND, x: i32, y: i32, cx: i32, cy: i32, flags: u32) -> BOOL;
    fn GetClientRect(hwnd: HWND, rect: *mut RECT) -> BOOL;
    fn AdjustWindowRectEx(rect: *mut RECT, style: u32, menu: BOOL, ex_style: u32) -> BOOL;
    fn ClientToScreen(hwnd: HWND, pt: *mut POINT) -> BOOL;
    fn ScreenToClient(hwnd: HWND, pt: *mut POINT) -> BOOL;
    fn MapWindowPoints(from: HWND, to: HWND, pts: *mut POINT, count: u32) -> i32;
    fn GetMessageW(msg: *mut MSG, hwnd: HWND, min: u32, max: u32) -> BOOL;
    fn TranslateMessage(msg: *const MSG) -> BOOL;
    fn DispatchMessageW(msg: *const MSG) -> isize;
    fn PostQuitMessage(code: i32);
    fn DefWindowProcW(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize;
    fn PostMessageW(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> BOOL;
    fn SendMessageW(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize;
    fn SetWindowLongPtrW(hwnd: HWND, idx: i32, val: isize) -> isize;
    fn GetWindowLongPtrW(hwnd: HWND, idx: i32) -> isize;
    fn SetLayeredWindowAttributes(hwnd: HWND, key: u32, alpha: u8, flags: u32) -> BOOL;
    fn GetCursorPos(pt: *mut POINT) -> BOOL;
    fn SetCapture(hwnd: HWND) -> HWND;
    fn ReleaseCapture() -> BOOL;
    fn GetKeyState(vk: i32) -> i16;
    fn LoadCursorW(inst: isize, name: *const u16) -> isize;
    fn SetCursor(cursor: isize) -> isize;
    fn LoadIconW(inst: isize, name: *const u16) -> isize;
    fn BeginPaint(hwnd: HWND, ps: *mut PAINTSTRUCT) -> isize;
    fn EndPaint(hwnd: HWND, ps: *const PAINTSTRUCT) -> BOOL;
    fn FillRect(hdc: isize, rc: *const RECT, br: isize) -> i32;
    fn DrawTextW(hdc: isize, text: *const u16, len: i32, rc: *mut RECT, fmt: u32) -> i32;
    fn InvalidateRect(hwnd: HWND, rc: *const RECT, erase: BOOL) -> BOOL;
    fn CreatePopupMenu() -> HMENU;
    fn AppendMenuW(menu: HMENU, flags: u32, id: usize, item: *const u16) -> BOOL;
    fn TrackPopupMenu(menu: HMENU, flags: u32, x: i32, y: i32, res: i32, hwnd: HWND, rc: *const RECT) -> i32;
    fn DestroyMenu(menu: HMENU) -> BOOL;
    fn SetForegroundWindow(hwnd: HWND) -> BOOL;
    fn EnumWindows(func: Option<unsafe extern "system" fn(HWND, isize) -> BOOL>, lp: isize) -> BOOL;
    fn IsWindow(hwnd: HWND) -> BOOL;
    fn IsWindowVisible(hwnd: HWND) -> BOOL;
    fn IsIconic(hwnd: HWND) -> BOOL;
    fn GetWindowTextLengthW(hwnd: HWND) -> i32;
    fn GetWindowTextW(hwnd: HWND, buf: *mut u16, max: i32) -> i32;
    fn SetWindowTextW(hwnd: HWND, text: *const u16) -> BOOL;
    fn EnableWindow(hwnd: HWND, enable: BOOL) -> BOOL;
    fn SetFocus(hwnd: HWND) -> HWND;
    fn ChildWindowFromPointEx(parent: HWND, pt: POINT, flags: u32) -> HWND;
}

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
struct RegionRect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    saved_regions: BTreeMap<String, RegionRect>,
}

#[derive(Clone, Copy, PartialEq)]
enum MouseMode {
    Normal,
    ClickForwarding,
    ClickThrough,
}

struct AppState {
    sel_overlay_hwnd: HWND,
    target_hwnd: HWND,
    thumb_id: HTHUMBNAIL,
    source_size: (i32, i32),
    viewport: RegionRect,
    prev_viewport: RegionRect,
    opacity: u8,
    always_on_top: bool,
    borderless: bool,
    client_area_only: bool,
    mouse_mode: MouseMode,
    is_panning: bool,
    selecting_region: bool,
    is_dragging_box: bool,
    drag_start: POINT,
    pan_initial: RegionRect,
    windows_list: Vec<(HWND, String)>,
    config: AppConfig,
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn is_key_down(vk: i32) -> bool {
    unsafe { (GetKeyState(vk) as u16 & 0x8000) != 0 }
}

fn config_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("regions.json")))
        .unwrap_or_else(|| PathBuf::from("regions.json"))
}

fn load_config() -> AppConfig {
    fs::read_to_string(config_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_config(cfg: &AppConfig) {
    let path = config_path();
    let tmp = path.with_extension("json.tmp");
    if let Ok(json) = serde_json::to_string_pretty(cfg) {
        if fs::write(&tmp, json).is_ok() {
            let _ = fs::rename(&tmp, &path);
        }
    }
}

unsafe fn get_state<'a>(hwnd: HWND) -> Option<&'a mut AppState> {
    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
    ptr.as_mut()
}

unsafe fn query_source_bounds(st: &AppState) -> (i32, i32) {
    if st.client_area_only {
        let mut rc = RECT::default();
        if GetClientRect(st.target_hwnd, &mut rc) != 0 && rc.right > 0 && rc.bottom > 0 {
            return (rc.right, rc.bottom);
        }
    }
    let mut sz = SIZE::default();
    if st.thumb_id != 0 && DwmQueryThumbnailSourceSize(st.thumb_id, &mut sz) == 0 && sz.cx > 0 && sz.cy > 0 {
        return (sz.cx, sz.cy);
    }
    (800, 600)
}

fn clamp_rect(mut r: RegionRect, (sw, sh): (i32, i32)) -> RegionRect {
    let sw = sw.max(32);
    let sh = sh.max(32);
    r.width = r.width.clamp(32, sw);
    r.height = r.height.clamp(32, sh);
    r.x = r.x.clamp(0, (sw - r.width).max(0));
    r.y = r.y.clamp(0, (sh - r.height).max(0));
    r
}

unsafe fn refresh_thumbnail(hwnd: HWND) {
    if IsIconic(hwnd) != 0 {
        return;
    }
    let Some(st) = get_state(hwnd) else { return };
    if st.thumb_id == 0 || st.target_hwnd == 0 || IsWindow(st.target_hwnd) == 0 {
        return;
    }
    let bounds = query_source_bounds(st);
    st.source_size = bounds;
    let src = if st.selecting_region {
        RegionRect { x: 0, y: 0, width: bounds.0, height: bounds.1 }
    } else {
        st.viewport = clamp_rect(st.viewport, bounds);
        st.viewport
    };
    let mut client = RECT::default();
    GetClientRect(hwnd, &mut client);
    let props = DWM_THUMBNAIL_PROPERTIES {
        dw_flags: DWM_TNP_RECTDESTINATION
            | DWM_TNP_RECTSOURCE
            | DWM_TNP_OPACITY
            | DWM_TNP_VISIBLE
            | DWM_TNP_SOURCECLIENTAREAONLY,
        rc_destination: client,
        rc_source: RECT {
            left: src.x,
            top: src.y,
            right: src.x + src.width,
            bottom: src.y + src.height,
        },
        opacity: 255,
        f_visible: 1,
        f_source_client_area_only: if st.client_area_only { 1 } else { 0 },
    };
    DwmUpdateThumbnailProperties(st.thumb_id, &props);
}

unsafe fn apply_window_styles(hwnd: HWND) {
    let (borderless, topmost, opacity, mode) = match get_state(hwnd) {
        Some(s) => (s.borderless, s.always_on_top, s.opacity, s.mouse_mode),
        None => return,
    };
    let style = if borderless {
        WS_POPUP | WS_VISIBLE | WS_MINIMIZEBOX
    } else {
        WS_OVERLAPPEDWINDOW | WS_VISIBLE
    };
    let mut ex = WS_EX_LAYERED;
    if topmost {
        ex |= WS_EX_TOPMOST;
    }
    if mode == MouseMode::ClickThrough {
        ex |= WS_EX_TRANSPARENT;
    }
    SetWindowLongPtrW(hwnd, GWL_STYLE, style as isize);
    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex as isize);
    SetLayeredWindowAttributes(hwnd, 0, opacity, LWA_ALPHA);
    let z = if topmost { HWND_TOPMOST } else { HWND_NOTOPMOST };
    SetWindowPos(hwnd, z, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED);
}

unsafe fn fit_window_aspect(hwnd: HWND) {
    if IsIconic(hwnd) != 0 {
        return;
    }
    let (vw, vh, borderless, topmost) = match get_state(hwnd) {
        Some(s) if s.viewport.width > 0 && s.viewport.height > 0 => {
            (s.viewport.width, s.viewport.height, s.borderless, s.always_on_top)
        }
        _ => return,
    };
    let mut client = RECT::default();
    GetClientRect(hwnd, &mut client);
    let cw = (client.right - client.left).max(120);
    let ch = ((cw as f64 * vh as f64) / vw as f64).round() as i32;
    let style = if borderless {
        WS_POPUP | WS_VISIBLE | WS_MINIMIZEBOX
    } else {
        WS_OVERLAPPEDWINDOW | WS_VISIBLE
    };
    let ex = WS_EX_LAYERED | if topmost { WS_EX_TOPMOST } else { 0 };
    let mut adj = RECT { left: 0, top: 0, right: cw, bottom: ch.max(80) };
    AdjustWindowRectEx(&mut adj, style, 0, ex);
    SetWindowPos(
        hwnd, 0, 0, 0, adj.right - adj.left, adj.bottom - adj.top,
        SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
    );
}

unsafe fn update_title(hwnd: HWND) {
    let title = match get_state(hwnd) {
        Some(s) if s.selecting_region => "PanView - คลิกซ้ายลากคลุมพื้นที่ (Esc เพื่อยกเลิก)".to_string(),
        Some(s) if s.target_hwnd != 0 => {
            let mut buf = [0u16; 128];
            let len = GetWindowTextW(s.target_hwnd, buf.as_mut_ptr(), 128);
            let name = String::from_utf16_lossy(&buf[..len.max(0) as usize]);
            format!("PanView - {}", name)
        }
        _ => "PanView (คลิกขวาเพื่อเลือกหน้าต่าง)".to_string(),
    };
    SetWindowTextW(hwnd, wide(&title).as_ptr());
}

unsafe fn attach_target(hwnd: HWND, target: HWND) {
    if let Some(st) = get_state(hwnd) {
        if st.thumb_id != 0 {
            DwmUnregisterThumbnail(st.thumb_id);
            st.thumb_id = 0;
        }
        let mut thumb: HTHUMBNAIL = 0;
        if DwmRegisterThumbnail(hwnd, target, &mut thumb) == 0 {
            st.target_hwnd = target;
            st.thumb_id = thumb;
            st.selecting_region = false;
            let bounds = query_source_bounds(st);
            st.source_size = bounds;
            st.viewport = RegionRect { x: 0, y: 0, width: bounds.0, height: bounds.1 };
        }
    }
    update_title(hwnd);
    fit_window_aspect(hwnd);
    refresh_thumbnail(hwnd);
    InvalidateRect(hwnd, null(), 1);
}

struct EnumCtx {
    self_hwnd: HWND,
    overlay_hwnd: HWND,
    list: Vec<(HWND, String)>,
}

unsafe extern "system" fn enum_windows_cb(hwnd: HWND, lparam: isize) -> BOOL {
    let ctx = &mut *(lparam as *mut EnumCtx);
    if hwnd == ctx.self_hwnd || hwnd == ctx.overlay_hwnd || IsWindowVisible(hwnd) == 0 || IsIconic(hwnd) != 0 {
        return 1;
    }
    let mut cloaked: u32 = 0;
    if DwmGetWindowAttribute(hwnd, DWMWA_CLOAKED, &mut cloaked, 4) == 0 && cloaked != 0 {
        return 1;
    }
    let len = GetWindowTextLengthW(hwnd);
    if len <= 0 {
        return 1;
    }
    let mut buf = vec![0u16; (len + 1) as usize];
    let read = GetWindowTextW(hwnd, buf.as_mut_ptr(), len + 1);
    if read > 0 {
        let title = String::from_utf16_lossy(&buf[..read as usize]);
        if title != "Program Manager" {
            ctx.list.push((hwnd, title));
        }
    }
    1
}

struct DlgData {
    edit_hwnd: HWND,
    done: bool,
    result: Option<String>,
}

unsafe extern "system" fn dlg_wndproc(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize {
    match msg {
        WM_COMMAND => {
            let id = wp & 0xFFFF;
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut DlgData;
            if let Some(d) = ptr.as_mut() {
                if id == 1 {
                    let len = GetWindowTextLengthW(d.edit_hwnd);
                    let mut buf = vec![0u16; (len.max(0) + 1) as usize];
                    let r = GetWindowTextW(d.edit_hwnd, buf.as_mut_ptr(), len + 1);
                    let text = String::from_utf16_lossy(&buf[..r.max(0) as usize]).trim().to_string();
                    if !text.is_empty() {
                        d.result = Some(text);
                    }
                    d.done = true;
                } else if id == 2 {
                    d.done = true;
                }
            }
            0
        }
        WM_CLOSE => {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut DlgData;
            if let Some(d) = ptr.as_mut() {
                d.done = true;
            }
            0
        }
        _ => DefWindowProcW(hwnd, msg, wp, lp),
    }
}

unsafe fn prompt_region_name(owner: HWND, default_idx: usize) -> Option<String> {
    let inst = GetModuleHandleW(null());
    let cls = wide("PanViewSaveDlg");
    let wc = WNDCLASSEXW {
        cb_size: size_of::<WNDCLASSEXW>() as u32,
        style: 0,
        lpfn_wnd_proc: Some(dlg_wndproc),
        cb_cls_extra: 0,
        cb_wnd_extra: 0,
        h_instance: inst,
        h_icon: 0,
        h_cursor: LoadCursorW(0, 32512usize as *const u16),
        hbr_background: 6,
        lpsz_menu_name: null(),
        lpsz_class_name: cls.as_ptr(),
        h_icon_sm: 0,
    };
    RegisterClassExW(&wc);

    let mut pt = POINT { x: 200, y: 200 };
    ClientToScreen(owner, &mut pt);
    let dlg = CreateWindowExW(
        WS_EX_TOPMOST, cls.as_ptr(), wide("Save Current Region").as_ptr(),
        WS_POPUP | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
        pt.x + 20, pt.y + 20, 340, 145, owner, 0, inst, null(),
    );
    if dlg == 0 {
        return None;
    }
    CreateWindowExW(
        0, wide("STATIC").as_ptr(), wide("ตั้งชื่อพื้นที่ (Region Name):").as_ptr(),
        WS_CHILD | WS_VISIBLE, 14, 12, 300, 22, dlg, 0, inst, null(),
    );
    let def_name = format!("Region {}", default_idx);
    let edit = CreateWindowExW(
        0, wide("EDIT").as_ptr(), wide(&def_name).as_ptr(),
        WS_CHILD | WS_VISIBLE | WS_BORDER | 0x0080, 14, 36, 296, 26, dlg, 100, inst, null(),
    );
    CreateWindowExW(
        0, wide("BUTTON").as_ptr(), wide("บันทึก (Enter)").as_ptr(),
        WS_CHILD | WS_VISIBLE | 0x0001, 130, 72, 95, 28, dlg, 1, inst, null(),
    );
    CreateWindowExW(
        0, wide("BUTTON").as_ptr(), wide("ยกเลิก (Esc)").as_ptr(),
        WS_CHILD | WS_VISIBLE, 233, 72, 77, 28, dlg, 2, inst, null(),
    );

    let mut data = DlgData { edit_hwnd: edit, done: false, result: None };
    SetWindowLongPtrW(dlg, GWLP_USERDATA, &mut data as *mut DlgData as isize);
    EnableWindow(owner, 0);
    SetFocus(edit);
    SendMessageW(edit, 0x00B1, 0, -1);

    let mut msg: MSG = zeroed();
    while !data.done && GetMessageW(&mut msg, 0, 0, 0) > 0 {
        if msg.message == WM_KEYDOWN {
            if msg.w_param == VK_RETURN {
                SendMessageW(dlg, WM_COMMAND, 1, 0);
                continue;
            } else if msg.w_param == VK_ESCAPE {
                data.done = true;
                break;
            }
        }
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    EnableWindow(owner, 1);
    DestroyWindow(dlg);
    SetForegroundWindow(owner);
    data.result
}

unsafe fn map_and_forward_mouse(hwnd: HWND, msg: u32, wp: usize, client_pt: POINT, is_wheel: bool) {
    let Some(st) = get_state(hwnd) else { return };
    if st.target_hwnd == 0 || IsWindow(st.target_hwnd) == 0 {
        return;
    }
    let mut rc = RECT::default();
    GetClientRect(hwnd, &mut rc);
    let cw = (rc.right - rc.left).max(1) as f64;
    let ch = (rc.bottom - rc.top).max(1) as f64;
    let tx = st.viewport.x + ((client_pt.x as f64 / cw) * st.viewport.width as f64).round() as i32;
    let ty = st.viewport.y + ((client_pt.y as f64 / ch) * st.viewport.height as f64).round() as i32;
    let target_root = st.target_hwnd;

    let mut cur = target_root;
    let mut cur_pt = POINT { x: tx, y: ty };
    for _ in 0..8 {
        let child = ChildWindowFromPointEx(cur, cur_pt, 0x0001 | 0x0002);
        if child == 0 || child == cur {
            break;
        }
        MapWindowPoints(cur, child, &mut cur_pt, 1);
        cur = child;
    }
    if is_wheel {
        let mut scr = POINT { x: tx, y: ty };
        ClientToScreen(target_root, &mut scr);
        let lp = ((scr.y as i16 as u16 as isize) << 16) | (scr.x as i16 as u16 as isize);
        PostMessageW(cur, WM_MOUSEWHEEL, wp, lp);
    } else {
        let lp = ((cur_pt.y as i16 as u16 as isize) << 16) | (cur_pt.x as i16 as u16 as isize);
        PostMessageW(cur, msg, wp, lp);
    }
}

unsafe fn open_context_menu(hwnd: HWND) {
    let overlay = match get_state(hwnd) {
        Some(s) => s.sel_overlay_hwnd,
        None => return,
    };
    let mut enum_ctx = EnumCtx { self_hwnd: hwnd, overlay_hwnd: overlay, list: Vec::new() };
    EnumWindows(Some(enum_windows_cb), &mut enum_ctx as *mut EnumCtx as isize);

    let menu = CreatePopupMenu();
    let win_menu = CreatePopupMenu();
    let saved_menu = CreatePopupMenu();
    let del_menu = CreatePopupMenu();
    let opacity_menu = CreatePopupMenu();
    let mode_menu = CreatePopupMenu();

    let is_minimized = IsIconic(hwnd) != 0;
    let mut has_target = false;
    if let Some(st) = get_state(hwnd) {
        st.windows_list = enum_ctx.list;
        has_target = st.target_hwnd != 0 && IsWindow(st.target_hwnd) != 0;

        if st.windows_list.is_empty() {
            AppendMenuW(win_menu, MF_STRING | MF_GRAYED, 0, wide("(ไม่พบหน้าต่างที่เปิดอยู่)").as_ptr());
        } else {
            for (i, (w_hwnd, title)) in st.windows_list.iter().take(60).enumerate() {
                let flag = if *w_hwnd == st.target_hwnd { MF_CHECKED } else { 0 };
                let label: String = title.chars().take(55).collect();
                AppendMenuW(win_menu, MF_STRING | flag, 1000 + i, wide(&label).as_ptr());
            }
        }

        if st.config.saved_regions.is_empty() {
            AppendMenuW(saved_menu, MF_STRING | MF_GRAYED, 0, wide("(ยังไม่มีพื้นที่ที่บันทึกไว้)").as_ptr());
            AppendMenuW(del_menu, MF_STRING | MF_GRAYED, 0, wide("(ว่าง)").as_ptr());
        } else {
            for (i, (name, r)) in st.config.saved_regions.iter().enumerate() {
                let item = format!("{} ({}x{} @ {},{})", name, r.width, r.height, r.x, r.y);
                AppendMenuW(saved_menu, MF_STRING, 2000 + i, wide(&item).as_ptr());
                AppendMenuW(del_menu, MF_STRING, 3000 + i, wide(name).as_ptr());
            }
        }

        for (id, pct, val) in [(201, "100%", 255u8), (202, "90%", 230), (203, "75%", 191), (204, "50%", 128), (205, "25%", 64)] {
            let f = if st.opacity == val { MF_CHECKED } else { 0 };
            AppendMenuW(opacity_menu, MF_STRING | f, id, wide(pct).as_ptr());
        }

        let f_norm = if st.mouse_mode == MouseMode::Normal { MF_CHECKED } else { 0 };
        let f_fwd = if st.mouse_mode == MouseMode::ClickForwarding { MF_CHECKED } else { 0 };
        let f_thru = if st.mouse_mode == MouseMode::ClickThrough { MF_CHECKED } else { 0 };
        AppendMenuW(mode_menu, MF_STRING | f_norm, 210, wide("ปกติ (ลากย้ายหน้าต่าง + Ctrl ลากเพื่อ Pan)").as_ptr());
        AppendMenuW(mode_menu, MF_STRING | f_fwd, 211, wide("Click & Scroll Forwarding (ส่งคลิกและลูกกลิ้งไปต้นทาง)").as_ptr());
        AppendMenuW(mode_menu, MF_STRING | f_thru, 212, wide("Click-Through (เมาส์คลิกทะลุ - แก้คืนที่ไอคอน Tray)").as_ptr());

        let t_flag = if has_target { 0 } else { MF_GRAYED };
        AppendMenuW(menu, MF_POPUP, win_menu as usize, wide("เลือกหน้าต่าง (Select Window)").as_ptr());
        AppendMenuW(menu, MF_SEPARATOR, 0, null());
        AppendMenuW(menu, MF_STRING | t_flag, 101, wide("ลากกรอบเลือกพื้นที่ (Select Region...)").as_ptr());
        AppendMenuW(menu, MF_STRING | t_flag, 102, wide("แสดงเต็มหน้าต่าง (Reset Full View)").as_ptr());
        AppendMenuW(menu, MF_STRING | t_flag, 103, wide("บันทึกพื้นที่ปัจจุบัน... (Save Current Region)").as_ptr());
        AppendMenuW(menu, MF_POPUP | t_flag, saved_menu as usize, wide("เรียกใช้พื้นที่ที่บันทึกไว้ (Stored Regions)").as_ptr());
        AppendMenuW(menu, MF_POPUP, del_menu as usize, wide("ลบพื้นที่ที่บันทึกไว้ (Delete Region)").as_ptr());
        AppendMenuW(menu, MF_SEPARATOR, 0, null());
        AppendMenuW(menu, MF_POPUP, mode_menu as usize, wide("โหมดเมาส์ (Mouse Mode)").as_ptr());
        AppendMenuW(menu, MF_POPUP, opacity_menu as usize, wide("ความโปร่งใส (Opacity)").as_ptr());
        AppendMenuW(menu, MF_STRING | if st.always_on_top { MF_CHECKED } else { 0 }, 104, wide("ลอยบนสุดเสมอ (Always on Top)").as_ptr());
        AppendMenuW(menu, MF_STRING | if st.borderless { MF_CHECKED } else { 0 }, 105, wide("ซ่อนขอบหน้าต่าง (Borderless)").as_ptr());
        AppendMenuW(menu, MF_STRING | if st.client_area_only { MF_CHECKED } else { 0 }, 106, wide("ตัดแถบหัวหน้าต่างต้นทาง (Client Area Only)").as_ptr());
        AppendMenuW(menu, MF_SEPARATOR, 0, null());
        if is_minimized {
            AppendMenuW(menu, MF_STRING, 198, wide("เรียกคืนหน้าต่าง (Restore Window)").as_ptr());
        } else {
            AppendMenuW(menu, MF_STRING, 198, wide("พับเก็บหน้าต่าง (Minimize)").as_ptr());
        }
        AppendMenuW(menu, MF_STRING, 199, wide("ออกจากโปรแกรม (Exit)").as_ptr());
    }

    let mut pt = POINT::default();
    GetCursorPos(&mut pt);
    SetForegroundWindow(hwnd);
    let cmd = TrackPopupMenu(menu, TPM_RIGHTBUTTON | TPM_RETURNCMD, pt.x, pt.y, 0, hwnd, null());
    PostMessageW(hwnd, 0, 0, 0);
    DestroyMenu(menu);

    if cmd > 0 {
        handle_menu_cmd(hwnd, cmd as usize, has_target);
    }
}

unsafe fn handle_menu_cmd(hwnd: HWND, cmd: usize, has_target: bool) {
    match cmd {
        101 if has_target => {
            if IsIconic(hwnd) != 0 {
                ShowWindow(hwnd, SW_RESTORE);
            }
            if let Some(st) = get_state(hwnd) {
                st.prev_viewport = st.viewport;
                st.selecting_region = true;
            }
            update_title(hwnd);
            refresh_thumbnail(hwnd);
        }
        102 if has_target => {
            if let Some(st) = get_state(hwnd) {
                st.selecting_region = false;
                st.viewport = RegionRect { x: 0, y: 0, width: st.source_size.0, height: st.source_size.1 };
            }
            update_title(hwnd);
            fit_window_aspect(hwnd);
            refresh_thumbnail(hwnd);
        }
        103 if has_target => {
            let next_idx = get_state(hwnd).map(|s| s.config.saved_regions.len() + 1).unwrap_or(1);
            if let Some(name) = prompt_region_name(hwnd, next_idx) {
                if let Some(st) = get_state(hwnd) {
                    st.config.saved_regions.insert(name, st.viewport);
                    save_config(&st.config);
                }
            }
        }
        104 => {
            if let Some(st) = get_state(hwnd) {
                st.always_on_top = !st.always_on_top;
            }
            apply_window_styles(hwnd);
        }
        105 => {
            if let Some(st) = get_state(hwnd) {
                st.borderless = !st.borderless;
            }
            apply_window_styles(hwnd);
            refresh_thumbnail(hwnd);
        }
        106 => {
            if let Some(st) = get_state(hwnd) {
                st.client_area_only = !st.client_area_only;
            }
            refresh_thumbnail(hwnd);
        }
        198 => {
            if IsIconic(hwnd) != 0 {
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
            } else {
                ShowWindow(hwnd, SW_MINIMIZE);
            }
        }
        199 => {
            DestroyWindow(hwnd);
        }
        201..=205 => {
            let alpha = match cmd {
                201 => 255,
                202 => 230,
                203 => 191,
                204 => 128,
                _ => 64,
            };
            if let Some(st) = get_state(hwnd) {
                st.opacity = alpha;
            }
            apply_window_styles(hwnd);
        }
        210..=212 => {
            let mode = match cmd {
                211 => MouseMode::ClickForwarding,
                212 => MouseMode::ClickThrough,
                _ => MouseMode::Normal,
            };
            if let Some(st) = get_state(hwnd) {
                st.mouse_mode = mode;
            }
            apply_window_styles(hwnd);
        }
        1000..=1999 => {
            if IsIconic(hwnd) != 0 {
                ShowWindow(hwnd, SW_RESTORE);
            }
            let target = get_state(hwnd)
                .and_then(|s| s.windows_list.get(cmd - 1000).map(|x| x.0))
                .unwrap_or(0);
            if target != 0 {
                attach_target(hwnd, target);
            }
        }
        2000..=2999 if has_target => {
            if IsIconic(hwnd) != 0 {
                ShowWindow(hwnd, SW_RESTORE);
            }
            let chosen = get_state(hwnd).and_then(|s| {
                s.config.saved_regions.values().nth(cmd - 2000).copied()
            });
            if let Some(rect) = chosen {
                if let Some(st) = get_state(hwnd) {
                    st.selecting_region = false;
                    st.viewport = clamp_rect(rect, st.source_size);
                }
                update_title(hwnd);
                fit_window_aspect(hwnd);
                refresh_thumbnail(hwnd);
            }
        }
        3000..=3999 => {
            if let Some(st) = get_state(hwnd) {
                let key_opt = st.config.saved_regions.keys().nth(cmd - 3000).cloned();
                if let Some(key) = key_opt {
                    st.config.saved_regions.remove(&key);
                    save_config(&st.config);
                }
            }
        }
        _ => {}
    }
}

unsafe extern "system" fn host_wndproc(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize {
    match msg {
        WM_NCHITTEST => {
            let borderless = get_state(hwnd).map(|s| s.borderless).unwrap_or(false);
            if borderless {
                let mut pt = POINT {
                    x: (lp & 0xFFFF) as i16 as i32,
                    y: ((lp >> 16) & 0xFFFF) as i16 as i32,
                };
                ScreenToClient(hwnd, &mut pt);
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                let g = 6;
                let left = pt.x < g;
                let right = pt.x >= rc.right - g;
                let top = pt.y < g;
                let bottom = pt.y >= rc.bottom - g;
                if top && left { return HTTOPLEFT; }
                if top && right { return HTTOPRIGHT; }
                if bottom && left { return HTBOTTOMLEFT; }
                if bottom && right { return HTBOTTOMRIGHT; }
                if left { return HTLEFT; }
                if right { return HTRIGHT; }
                if top { return HTTOP; }
                if bottom { return HTBOTTOM; }
                return HTCLIENT;
            }
            DefWindowProcW(hwnd, msg, wp, lp)
        }
        WM_SETCURSOR => {
            if (lp & 0xFFFF) as isize == HTCLIENT {
                if let Some(st) = get_state(hwnd) {
                    if st.selecting_region {
                        SetCursor(LoadCursorW(0, 32515usize as *const u16));
                        return 1;
                    } else if st.is_panning || is_key_down(VK_CONTROL) {
                        SetCursor(LoadCursorW(0, 32646usize as *const u16));
                        return 1;
                    }
                }
            }
            DefWindowProcW(hwnd, msg, wp, lp)
        }
        WM_LBUTTONDOWN => {
            let client_pt = POINT {
                x: (lp & 0xFFFF) as i16 as i32,
                y: ((lp >> 16) & 0xFFFF) as i16 as i32,
            };
            let action = if let Some(st) = get_state(hwnd) {
                if st.selecting_region {
                    st.is_dragging_box = true;
                    st.drag_start = client_pt;
                    1
                } else if is_key_down(VK_CONTROL) && st.target_hwnd != 0 {
                    let mut cur = POINT::default();
                    GetCursorPos(&mut cur);
                    st.is_panning = true;
                    st.drag_start = cur;
                    st.pan_initial = st.viewport;
                    2
                } else if st.mouse_mode == MouseMode::ClickForwarding {
                    3
                } else {
                    4
                }
            } else {
                0
            };
            match action {
                1 | 2 => { SetCapture(hwnd); }
                3 => { map_and_forward_mouse(hwnd, WM_LBUTTONDOWN, wp, client_pt, false); }
                4 => {
                    ReleaseCapture();
                    SendMessageW(hwnd, WM_NCLBUTTONDOWN, HTCAPTION as usize, 0);
                }
                _ => {}
            }
            0
        }
        WM_MBUTTONDOWN => {
            if let Some(st) = get_state(hwnd) {
                if st.target_hwnd != 0 && !st.selecting_region {
                    let mut cur = POINT::default();
                    GetCursorPos(&mut cur);
                    st.is_panning = true;
                    st.drag_start = cur;
                    st.pan_initial = st.viewport;
                    SetCapture(hwnd);
                }
            }
            0
        }
        WM_MOUSEMOVE => {
            let mut do_refresh = false;
            let mut overlay_box: Option<(HWND, i32, i32, i32, i32)> = None;
            if let Some(st) = get_state(hwnd) {
                if st.is_panning {
                    let mut cur = POINT::default();
                    GetCursorPos(&mut cur);
                    let mut rc = RECT::default();
                    GetClientRect(hwnd, &mut rc);
                    let cw = (rc.right - rc.left).max(1) as f64;
                    let ch = (rc.bottom - rc.top).max(1) as f64;
                    let dx = ((cur.x - st.drag_start.x) as f64 * (st.pan_initial.width as f64 / cw)).round() as i32;
                    let dy = ((cur.y - st.drag_start.y) as f64 * (st.pan_initial.height as f64 / ch)).round() as i32;
                    st.viewport = clamp_rect(
                        RegionRect {
                            x: st.pan_initial.x - dx,
                            y: st.pan_initial.y - dy,
                            width: st.pan_initial.width,
                            height: st.pan_initial.height,
                        },
                        st.source_size,
                    );
                    do_refresh = true;
                } else if st.selecting_region && st.is_dragging_box {
                    let cx = (lp & 0xFFFF) as i16 as i32;
                    let cy = ((lp >> 16) & 0xFFFF) as i16 as i32;
                    let mut p1 = POINT { x: st.drag_start.x.min(cx), y: st.drag_start.y.min(cy) };
                    let w = (st.drag_start.x - cx).abs().max(2);
                    let h = (st.drag_start.y - cy).abs().max(2);
                    ClientToScreen(hwnd, &mut p1);
                    overlay_box = Some((st.sel_overlay_hwnd, p1.x, p1.y, w, h));
                }
            }
            if do_refresh {
                refresh_thumbnail(hwnd);
            }
            if let Some((ov, x, y, w, h)) = overlay_box {
                SetWindowPos(ov, HWND_TOPMOST, x, y, w, h, SWP_NOACTIVATE | SWP_SHOWWINDOW);
            }
            0
        }
        WM_LBUTTONUP | WM_MBUTTONUP => {
            let client_pt = POINT {
                x: (lp & 0xFFFF) as i16 as i32,
                y: ((lp >> 16) & 0xFFFF) as i16 as i32,
            };
            let mut finish_select = false;
            let mut hide_ov: HWND = 0;
            let mut fwd_up = false;
            if let Some(st) = get_state(hwnd) {
                if st.is_panning {
                    st.is_panning = false;
                    ReleaseCapture();
                } else if st.selecting_region && st.is_dragging_box && msg == WM_LBUTTONUP {
                    st.is_dragging_box = false;
                    st.selecting_region = false;
                    hide_ov = st.sel_overlay_hwnd;
                    ReleaseCapture();
                    let mut rc = RECT::default();
                    GetClientRect(hwnd, &mut rc);
                    let cw = (rc.right - rc.left).max(1);
                    let ch = (rc.bottom - rc.top).max(1);
                    let x1 = st.drag_start.x.min(client_pt.x).clamp(0, cw);
                    let y1 = st.drag_start.y.min(client_pt.y).clamp(0, ch);
                    let x2 = st.drag_start.x.max(client_pt.x).clamp(0, cw);
                    let y2 = st.drag_start.y.max(client_pt.y).clamp(0, ch);
                    if (x2 - x1) >= 16 && (y2 - y1) >= 16 {
                        let (sw, sh) = st.source_size;
                        let nx = ((x1 as f64 / cw as f64) * sw as f64).round() as i32;
                        let ny = ((y1 as f64 / ch as f64) * sh as f64).round() as i32;
                        let nw = (((x2 - x1) as f64 / cw as f64) * sw as f64).round() as i32;
                        let nh = (((y2 - y1) as f64 / ch as f64) * sh as f64).round() as i32;
                        st.viewport = clamp_rect(RegionRect { x: nx, y: ny, width: nw, height: nh }, (sw, sh));
                    } else {
                        st.viewport = st.prev_viewport;
                    }
                    finish_select = true;
                } else if msg == WM_LBUTTONUP && st.mouse_mode == MouseMode::ClickForwarding {
                    fwd_up = true;
                }
            }
            if hide_ov != 0 {
                SetWindowPos(hide_ov, 0, 0, 0, 0, 0, SWP_HIDEWINDOW | SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
            }
            if finish_select {
                update_title(hwnd);
                fit_window_aspect(hwnd);
                refresh_thumbnail(hwnd);
            }
            if fwd_up {
                map_and_forward_mouse(hwnd, WM_LBUTTONUP, wp, client_pt, false);
            }
            0
        }
        WM_MOUSEWHEEL => {
            let delta = ((wp >> 16) as u16 as i16) as i32;
            if is_key_down(VK_CONTROL) {
                let mut zoomed = false;
                if let Some(st) = get_state(hwnd) {
                    if st.target_hwnd != 0 && !st.selecting_region {
                        let factor = if delta > 0 { 0.90 } else { 1.11 };
                        let nw = ((st.viewport.width as f64) * factor).round() as i32;
                        let nh = ((st.viewport.height as f64) * factor).round() as i32;
                        let cx = st.viewport.x + st.viewport.width / 2;
                        let cy = st.viewport.y + st.viewport.height / 2;
                        st.viewport = clamp_rect(
                            RegionRect { x: cx - nw / 2, y: cy - nh / 2, width: nw, height: nh },
                            st.source_size,
                        );
                        zoomed = true;
                    }
                }
                if zoomed {
                    refresh_thumbnail(hwnd);
                }
            } else {
                let is_fwd = get_state(hwnd).map(|s| s.mouse_mode == MouseMode::ClickForwarding).unwrap_or(false);
                if is_fwd {
                    let mut pt = POINT {
                        x: (lp & 0xFFFF) as i16 as i32,
                        y: ((lp >> 16) & 0xFFFF) as i16 as i32,
                    };
                    ScreenToClient(hwnd, &mut pt);
                    map_and_forward_mouse(hwnd, WM_MOUSEWHEEL, wp, pt, true);
                }
            }
            0
        }
        WM_KEYDOWN => {
            if wp == VK_ESCAPE {
                let mut canceled = false;
                let mut hide_ov = 0;
                if let Some(st) = get_state(hwnd) {
                    if st.selecting_region {
                        st.selecting_region = false;
                        st.is_dragging_box = false;
                        st.viewport = st.prev_viewport;
                        hide_ov = st.sel_overlay_hwnd;
                        ReleaseCapture();
                        canceled = true;
                    }
                }
                if hide_ov != 0 {
                    SetWindowPos(hide_ov, 0, 0, 0, 0, 0, SWP_HIDEWINDOW | SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
                }
                if canceled {
                    update_title(hwnd);
                    refresh_thumbnail(hwnd);
                }
            } else if is_key_down(VK_CONTROL) {
                let step = if is_key_down(VK_SHIFT) { 2 } else { 15 };
                let (dx, dy) = match wp {
                    VK_LEFT => (-step, 0),
                    VK_RIGHT => (step, 0),
                    VK_UP => (0, -step),
                    VK_DOWN => (0, step),
                    _ => (0, 0),
                };
                if dx != 0 || dy != 0 {
                    if let Some(st) = get_state(hwnd) {
                        st.viewport.x += dx;
                        st.viewport.y += dy;
                        st.viewport = clamp_rect(st.viewport, st.source_size);
                    }
                    refresh_thumbnail(hwnd);
                }
            }
            0
        }
        WM_RBUTTONUP => {
            open_context_menu(hwnd);
            0
        }
        WM_TRAY => {
            let ev = (lp & 0xFFFF) as u32;
            if ev == WM_LBUTTONUP && IsIconic(hwnd) != 0 {
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
            } else if ev == WM_RBUTTONUP || ev == WM_LBUTTONUP {
                open_context_menu(hwnd);
            }
            0
        }
        WM_SIZE => {
            if wp != 1 {
                refresh_thumbnail(hwnd);
            }
            0
        }
        WM_SIZING => {
            if let Some(st) = get_state(hwnd) {
                if st.target_hwnd != 0 && st.viewport.width > 0 && st.viewport.height > 0 {
                    let rc = &mut *(lp as *mut RECT);
                    let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
                    let ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
                    let mut frame = RECT::default();
                    AdjustWindowRectEx(&mut frame, style, 0, ex);
                    let fw = frame.right - frame.left;
                    let fh = frame.bottom - frame.top;
                    let aspect = st.viewport.width as f64 / st.viewport.height as f64;
                    let cw = (rc.right - rc.left - fw).max(80);
                    let ch = (rc.bottom - rc.top - fh).max(60);
                    if wp == 3 || wp == 6 {
                        let new_cw = (ch as f64 * aspect).round() as i32;
                        rc.right = rc.left + new_cw + fw;
                    } else {
                        let new_ch = (cw as f64 / aspect).round() as i32;
                        if wp == 4 || wp == 5 {
                            rc.top = rc.bottom - (new_ch + fh);
                        } else {
                            rc.bottom = rc.top + new_ch + fh;
                        }
                    }
                    return 1;
                }
            }
            DefWindowProcW(hwnd, msg, wp, lp)
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            let brush = CreateSolidBrush(0x0024201E);
            FillRect(hdc, &ps.rc_paint, brush);
            DeleteObject(brush);

            let has_thumb = get_state(hwnd).map(|s| s.thumb_id != 0).unwrap_or(false);
            if !has_thumb {
                let mut rc = RECT::default();
                GetClientRect(hwnd, &mut rc);
                rc.top += (rc.bottom - rc.top) / 3;
                SetBkMode(hdc, 1);
                SetTextColor(hdc, 0x00EAEAEA);
                let hint = wide("คลิกขวาที่นี่เพื่อเลือกหน้าต่าง (Right-Click Menu)\n\nCtrl + ลากเมาส์ หรือ เมาส์กลาง = เลื่อนมุมมองสด (Live Pan)\nCtrl + ลูกกลิ้งเมาส์ = ซูมเข้า/ออก");
                DrawTextW(hdc, hint.as_ptr(), -1, &mut rc, 0x0001 | 0x0010);
            }
            EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            if let Some(st) = get_state(hwnd) {
                if st.thumb_id != 0 {
                    DwmUnregisterThumbnail(st.thumb_id);
                }
                if st.sel_overlay_hwnd != 0 {
                    DestroyWindow(st.sel_overlay_hwnd);
                }
            }
            let mut nid: NOTIFYICONDATAW = zeroed();
            nid.cb_size = size_of::<NOTIFYICONDATAW>() as u32;
            nid.hwnd = hwnd;
            nid.u_id = 1;
            Shell_NotifyIconW(2, &nid);

            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
            if !ptr.is_null() {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
                drop(Box::from_raw(ptr));
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wp, lp),
    }
}

fn main() {
    unsafe {
        SetProcessDpiAwarenessContext(-4isize);
        let inst = GetModuleHandleW(null());

        let ov_cls = wide("PanViewSelOverlay");
        let ov_brush = CreateSolidBrush(0x00FFCC00);
        let wc_ov = WNDCLASSEXW {
            cb_size: size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfn_wnd_proc: Some(DefWindowProcW),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance: inst,
            h_icon: 0,
            h_cursor: 0,
            hbr_background: ov_brush,
            lpsz_menu_name: null(),
            lpsz_class_name: ov_cls.as_ptr(),
            h_icon_sm: 0,
        };
        RegisterClassExW(&wc_ov);
        let sel_overlay = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW,
            ov_cls.as_ptr(), wide("").as_ptr(), WS_POPUP,
            0, 0, 10, 10, 0, 0, inst, null(),
        );
        SetLayeredWindowAttributes(sel_overlay, 0, 95, LWA_ALPHA);

        let cls = wide("PanViewMainHost");
        let icon = LoadIconW(0, 32512usize as *const u16);
        let wc = WNDCLASSEXW {
            cb_size: size_of::<WNDCLASSEXW>() as u32,
            style: 0x0003,
            lpfn_wnd_proc: Some(host_wndproc),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance: inst,
            h_icon: icon,
            h_cursor: LoadCursorW(0, 32512usize as *const u16),
            hbr_background: GetStockObject(4),
            lpsz_menu_name: null(),
            lpsz_class_name: cls.as_ptr(),
            h_icon_sm: icon,
        };
        RegisterClassExW(&wc);

        let state = Box::new(AppState {
            sel_overlay_hwnd: sel_overlay,
            target_hwnd: 0,
            thumb_id: 0,
            source_size: (800, 600),
            viewport: RegionRect { x: 0, y: 0, width: 800, height: 600 },
            prev_viewport: RegionRect::default(),
            opacity: 255,
            always_on_top: true,
            borderless: false,
            client_area_only: true,
            mouse_mode: MouseMode::Normal,
            is_panning: false,
            selecting_region: false,
            is_dragging_box: false,
            drag_start: POINT::default(),
            pan_initial: RegionRect::default(),
            windows_list: Vec::new(),
            config: load_config(),
        });

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_LAYERED,
            cls.as_ptr(),
            wide("PanView (คลิกขวาเพื่อเลือกหน้าต่าง)").as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            150, 150, 500, 320,
            0, 0, inst, null(),
        );
        if hwnd == 0 {
            return;
        }
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(state) as isize);
        SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA);
        ShowWindow(hwnd, SW_SHOW);

        let mut nid: NOTIFYICONDATAW = zeroed();
        nid.cb_size = size_of::<NOTIFYICONDATAW>() as u32;
        nid.hwnd = hwnd;
        nid.u_id = 1;
        nid.u_flags = 0x01 | 0x02 | 0x04;
        nid.u_callback_message = WM_TRAY;
        nid.h_icon = icon;
        let tip = wide("PanView - คลิกเพื่อเปิดเมนูหรือเรียกคืนหน้าต่าง");
        for (i, &c) in tip.iter().take(127).enumerate() {
            nid.sz_tip[i] = c;
        }
        Shell_NotifyIconW(0, &nid);

        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
