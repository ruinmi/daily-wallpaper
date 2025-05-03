use device_query::{DeviceQuery, DeviceState};
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::BufReader,
    mem, ptr, thread,
    time::{Duration, Instant},
};
use tauri::Emitter;
use tauri::Manager;

const WORK_DURATION: Duration = Duration::new(30 * 60, 0);
// const WORK_DURATION: Duration = Duration::new(3, 0);
const REST_DURATION: Duration = Duration::new(5 * 60, 0);
// const REST_DURATION: Duration = Duration::new(10, 0);
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
        change_overlay_color_size(color, width as i32, 15);

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
        change_overlay_color_size(color, width as i32, 15);

        app.emit("rest_count", start_time.elapsed().as_secs())
            .unwrap();
    }

    app.emit("rest_done", "true").unwrap();
    close_overlay();
}

use std::ptr::null_mut;
use std::sync::Mutex;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HGDIOBJ, HWND, RECT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC, DeleteObject
    , SelectObject, RGB, SRCCOPY,
};
use winapi::um::winuser::{
    BeginPaint, CreateWindowExW, DefWindowProcW, DispatchMessageW, EndPaint, FillRect, FindWindowW
    , GetMessageW, GetSystemMetrics, InvalidateRect, KillTimer, PostMessageW,
    PostQuitMessage, RegisterClassW, SetLayeredWindowAttributes,
    SetTimer, ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW,
    LWA_COLORKEY, MSG, PAINTSTRUCT, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, WM_CLOSE, WM_CREATE,
    WM_DESTROY, WM_PAINT, WM_TIMER, WNDCLASSW, WS_EX_LAYERED, WS_EX_TOOLWINDOW,
    WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};

static COLOR: Mutex<u32> = Mutex::new(0x00FF00);
static RECT_SIZE: Mutex<(i32, i32)> = Mutex::new((100, 200));

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let color = *COLOR.lock().unwrap();
    let (width, height) = *RECT_SIZE.lock().unwrap();
    match msg {
        WM_CREATE => unsafe {
            SetTimer(hwnd, 1, 16, None);
            0
        },
        WM_TIMER => unsafe {
            InvalidateRect(hwnd, null_mut(), 1);
            0
        },
        WM_PAINT => unsafe {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);

            // 创建兼容DC和位图
            let mem_dc = CreateCompatibleDC(hdc);
            let mem_bitmap = CreateCompatibleBitmap(
                hdc,
                GetSystemMetrics(SM_CXSCREEN),
                GetSystemMetrics(SM_CYSCREEN),
            );
            SelectObject(mem_dc, mem_bitmap as HGDIOBJ);
            // 填充透明背景
            let h_brush = CreateSolidBrush(RGB(0, 0, 0));
            FillRect(mem_dc, &ps.rcPaint, h_brush);
            DeleteObject(h_brush as HGDIOBJ);
            // 绘制红色矩形
            let real_brush = CreateSolidBrush(color);
            let rect = RECT { left: 0, top: 0, right: width, bottom: height };
            FillRect(mem_dc, &rect, real_brush);

            DeleteObject(real_brush as HGDIOBJ);
            // 将内存DC内容复制到屏幕DC
            BitBlt(hdc,0,0,GetSystemMetrics(SM_CXSCREEN),GetSystemMetrics(SM_CYSCREEN),mem_dc,0,0,SRCCOPY,);
            // 清理
            DeleteObject(mem_bitmap as HGDIOBJ);
            DeleteDC(mem_dc);
            EndPaint(hwnd, &ps);
            0
        },
        WM_DESTROY => unsafe {
            KillTimer(hwnd, 1);
            PostQuitMessage(0);
            0
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn create_overlay(color: u32, width: i32, height: i32) {
    unsafe {
        let class_name = to_wstring("OverlayWindowClass");
        let window_name = to_wstring("Click-Through Overlay");
        let h_instance = GetModuleHandleW(ptr::null());
        *COLOR.lock().unwrap() = color;
        *RECT_SIZE.lock().unwrap() = (width, height);
        // Register the window class
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: GetModuleHandleW(null_mut()),
            lpszClassName: class_name.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        RegisterClassW(&wc);
        // 获取屏幕尺寸
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        // 创建窗口
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_POPUP,
            0,
            0,
            screen_width,
            screen_height,
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

        // 消息循环
        let mut msg: MSG = mem::zeroed();
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

fn change_overlay_color_size(color: u32, width: i32, height: i32) {
    *COLOR.lock().unwrap() = color;
    *RECT_SIZE.lock().unwrap() = (width, height);
}
