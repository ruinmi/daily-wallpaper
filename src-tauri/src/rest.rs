use device_query::{DeviceQuery, DeviceState};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, mem, ptr, thread, time::{Duration, Instant}};
use tauri::Emitter;
use tauri::Manager;

// const WORK_DURATION: Duration = Duration::new(30 * 60, 0);
const WORK_DURATION: Duration = Duration::new(3, 0);
// const REST_DURATION: Duration = Duration::new(5 * 60, 0);
const REST_DURATION: Duration = Duration::new(10, 0);
static MP3_PATH: Mutex<String> = Mutex::new(String::new());

#[tauri::command]
pub fn start_timer(app: tauri::AppHandle) {
    let app_data_dir = app.path().app_data_dir().unwrap();
    let mp3_path = app_data_dir
        .join("audio")
        .join("wakeup.mp3")
        .to_string_lossy()
        .into_owned();
    *MP3_PATH.lock().unwrap() = mp3_path.clone();

    thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut last_position = device_state.get_mouse().coords;

        loop {
            let mouse_position = device_state.get_mouse().coords;

            if mouse_position != last_position {
                // 如果鼠标移动且计时器未开始，则开始计时
                println!("Mouse moved! Starting timer...");
                work_counting(&app);

                // 计时结束后播放音乐
                println!("30 minutes passed! Playing music...");
                play_mp3();

                println!("Waiting for mouse to stop");
                wait_for_mouse_stop(&app);

                println!("Rest Counting");
                rest_counting(&app);

                last_position = device_state.get_mouse().coords;
            }

            thread::sleep(Duration::from_millis(100)); // 每 100 毫秒检查一次
        }
    });
}

fn work_counting(app: &tauri::AppHandle) {
    let start_time = Instant::now();
    // 等待 30 分钟
    while start_time.elapsed() < WORK_DURATION {
        thread::sleep(Duration::from_millis(1000));
        app.emit("work_count", start_time.elapsed().as_secs())
            .unwrap();
    }

    app.emit("work_done", "true").unwrap();
}

fn play_mp3() {
    thread::spawn(|| {
        let file_path = MP3_PATH.lock().unwrap().clone();
        // 保持 OutputStream 不被 drop
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to initialize OutputStream: {:?}", e);
                return; // 错误时退出线程
            }
        };
        // 创建 Sink 管理播放
        let sink = match Sink::try_new(&stream_handle) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to create Sink: {:?}", e);
                return; // 错误时退出线程
            }
        };
        // 加载并解码 MP3
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open file: {:?}", e);
                return; // 错误时退出线程
            }
        };
        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to decode file: {:?}", e);
                return; // 错误时退出线程
            }
        };
        // 将音频添加到 Sink
        sink.append(source);
        // 阻塞当前线程直到播放完成
        sink.sleep_until_end();
    });
}

fn wait_for_mouse_stop(app: &tauri::AppHandle) {
    let color = 0x82AAFF;
    thread::spawn(move || {
        create_overlay(color, 0, 15);
    });
    let mut start_time = Instant::now();
    let device_state = DeviceState::new();
    let mut last_position = device_state.get_mouse().coords;
    while start_time.elapsed() < Duration::from_secs(10) {
        let mouse_position = device_state.get_mouse().coords;
        if mouse_position != last_position {
            last_position = mouse_position;
            start_time = Instant::now();
            change_overlay_color_size(color, 0, 15);
            app.emit("move_continue", "true").unwrap();
        }

        thread::sleep(Duration::from_millis(10));

        let width = start_time.elapsed().as_secs() as f64 * 2560.0 / 10.0;
        change_overlay_color_size(color, width as u64, 15);

        app.emit("wait_mouse_stop", start_time.elapsed().as_secs())
            .unwrap();
    }
    close_overlay();
}

fn rest_counting(app: &tauri::AppHandle) {
    let color = 0xC5E478;
    thread::spawn(move || {
        create_overlay(color, 0, 15);
    });

    let start_time = Instant::now();
    while start_time.elapsed() < REST_DURATION {
        thread::sleep(Duration::from_millis(1000));

        let width = start_time.elapsed().as_secs() as f64 * 2560.0 / REST_DURATION.as_secs() as f64;
        change_overlay_color_size(color, width as u64, 15);

        app.emit("rest_count", start_time.elapsed().as_secs())
            .unwrap();
    }

    app.emit("rest_done", "true").unwrap();
    close_overlay();
}

use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::shared::minwindef::{TRUE, HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::{LONG, NULL};
use winapi::shared::windef::{HGDIOBJ, HWND, HBITMAP,HDC};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{
    CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteObject, Rectangle,
    SelectObject, RGB, BitBlt, SRCCOPY, DeleteDC
};
use winapi::um::winuser::{BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, FillRect, FindWindowW, GetDC, GetMessageW, GetSystemMetrics, PostMessageW, PostQuitMessage, RegisterClassW, ReleaseDC, SetLayeredWindowAttributes, SetTimer, ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, LWA_COLORKEY, MSG, PAINTSTRUCT, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, WM_CLOSE, WM_CREATE, WM_DESTROY, WM_PAINT, WNDCLASSW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP, WM_TIMER, InvalidateRect, WNDCLASSEXW, LoadCursorW, IDC_ARROW, RegisterClassExW};
use lazy_static::lazy_static;


static COLOR: Mutex<u32> = Mutex::new(0x00FF00);
static RECT_SIZE: Mutex<(u64, u64)> = Mutex::new((100, 200));
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
lazy_static! {
    static ref RECT_POS: Mutex<(i32, i32, i32, i32)> =
        Mutex::new((50, 50, 2, 2)); // (x, y, dx, dy)
}
unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let color = *COLOR.lock().unwrap();
    static mut HDC_MEM: HDC = null_mut();
    static mut HBM_MEM: HBITMAP = null_mut();
    static mut H_OLD: HGDIOBJ = null_mut();
    match msg {
        WM_CREATE => {
            // 创建内存 DC
            let hdc = GetDC(hwnd);
            HDC_MEM = CreateCompatibleDC(hdc);
            HBM_MEM = CreateCompatibleBitmap(hdc, WINDOW_WIDTH, WINDOW_HEIGHT);
            H_OLD = SelectObject(HDC_MEM, HBM_MEM as _);
            ReleaseDC(hwnd, hdc);
            0
        }
        WM_TIMER => {
            // 更新矩形位置
            let mut rect_pos = RECT_POS.lock().unwrap();
            let (x, y, dx, dy) = *rect_pos;
            let new_x = x + dx;
            let new_y = y + dy;
            let new_dx = if new_x <= 0 || new_x >= WINDOW_WIDTH - 100 { -dx } else { dx };
            let new_dy = if new_y <= 0 || new_y >= WINDOW_HEIGHT - 100 { -dy } else { dy };
            *rect_pos = (new_x, new_y, new_dx, new_dy);
            InvalidateRect(hwnd, null_mut(), 1);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);

            // 填充背景为透明色（黑色）
            let h_brush = CreateSolidBrush(color);
            FillRect(HDC_MEM, &ps.rcPaint, h_brush);
            DeleteObject(h_brush as _);

            // 绘制矩形
            let rect_pos = RECT_POS.lock().unwrap();
            let h_brush = CreateSolidBrush(RGB(255, 0, 0)); // 红色
            SelectObject(HDC_MEM, h_brush as _);
            Rectangle(HDC_MEM, rect_pos.0, rect_pos.1, rect_pos.0 + 100, rect_pos.1 + 100);
            DeleteObject(h_brush as _);

            // 复制到窗口
            BitBlt(hdc, 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, HDC_MEM, 0, 0, SRCCOPY);

            EndPaint(hwnd, &ps);
            0
        }
        WM_DESTROY => {
            // 清理资源
            SelectObject(HDC_MEM, H_OLD);
            DeleteObject(HBM_MEM as _);
            DeleteDC(HDC_MEM);
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn create_overlay(color: u32, width: u64, height: u64) {
    unsafe {
        let class_name = to_wstring("OverlayWindowClass");
        let window_name = to_wstring("Click-Through Overlay");
        let h_instance = GetModuleHandleW(ptr::null());
        *COLOR.lock().unwrap() = color;
        *RECT_SIZE.lock().unwrap() = (width, height);
        // Register the window class
        let wc = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as _,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: ptr::null_mut(),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: ptr::null_mut(),
        };

        RegisterClassExW(&wc);

        // let hwnd = CreateWindowExW(
        //     WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW,
        //     class_name.as_ptr(),
        //     window_name.as_ptr(),
        //     WS_POPUP,
        //     0,
        //     0,
        //     GetSystemMetrics(SM_CXSCREEN),
        //     GetSystemMetrics(SM_CYSCREEN),
        //     null_mut(),
        //     null_mut(),
        //     null_mut(),
        //     null_mut(),
        // );
        // 创建窗口
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_POPUP,
            0,
            0,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut(),
        );
        // 设置透明
        SetLayeredWindowAttributes(hwnd, RGB(0, 0, 0), 0, LWA_COLORKEY);

        // 显示窗口
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        // 设置定时器（每50ms）
        SetTimer(hwnd, 1, 50, *ptr::null_mut());

        // 消息循环
        let mut msg: winapi::um::winuser::MSG = mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn to_wstring(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn close_overlay() {
    let wide_title = to_wstring("Click-Through Overlay");
    let hwnd = unsafe { FindWindowW(null_mut(), wide_title.as_ptr()) }; // 根据类名查找
    if hwnd.is_null() {
        println!("No window found to close.");
    } else {
        unsafe {
            if PostMessageW(hwnd, WM_CLOSE, 0, 0) == 0 {
                println!(
                    "Failed to send close message! Error code: {}",
                    GetLastError()
                );
            } else {
                println!("Overlay closed.");
            }
        }
    }
}

fn change_overlay_color_size(color: u32, width: u64, height: u64) {
    *COLOR.lock().unwrap() = color;
    *RECT_SIZE.lock().unwrap() = (width, height);
    // let wide_title = to_wstring("Click-Through Overlay");
    // let hwnd = unsafe { FindWindowW(null_mut(), wide_title.as_ptr()) };
    // if hwnd.is_null() {
    //     println!("No window found to close.");
    // } else {
    //     unsafe {
    //         InvalidateRect(hwnd, null_mut(), TRUE);
    //     }
    // }
}
