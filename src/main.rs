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
        AppendMenuW(mode_menu, MF_STRING | f_fwd, 
