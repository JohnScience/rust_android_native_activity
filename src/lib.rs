// Based on:
use core::mem::MaybeUninit;

use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use log::info;

const FPS: f64 = 60.0;
const FRAME_TIME: u64 = (1.0 / FPS * 1000.0) as u64;

struct State {
    t: u8,
}

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );

    let mut quit = false;
    let mut redraw_pending = true;
    let mut native_window: Option<ndk::native_window::NativeWindow> = None;

    let mut state = State { t: 0 };

    while !quit {
        app.poll_events(
            Some(std::time::Duration::from_millis(FRAME_TIME)), /* timeout */
            |event| {
                match event {
                    PollEvent::Wake => {
                        info!("Early wake up");
                    }
                    PollEvent::Timeout => {
                        info!("Timed out");
                        // Real app would probably rely on vblank sync via graphics API...
                        redraw_pending = true;
                    }
                    PollEvent::Main(main_event) => {
                        info!("Main event: {:?}", main_event);
                        match main_event {
                            MainEvent::SaveState { saver, .. } => {
                                saver.store("foo://bar".as_bytes());
                            }
                            MainEvent::Pause => {}
                            MainEvent::Resume { loader, .. } => {
                                if let Some(state) = loader.load() {
                                    if let Ok(uri) = String::from_utf8(state) {
                                        info!("Resumed with saved state = {uri:#?}");
                                    }
                                }
                            }
                            MainEvent::InitWindow { .. } => {
                                native_window = app.native_window();
                                redraw_pending = true;
                            }
                            MainEvent::TerminateWindow { .. } => {
                                native_window = None;
                            }
                            MainEvent::WindowResized { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::RedrawNeeded { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::InputAvailable { .. } => {
                                redraw_pending = true;
                            }
                            MainEvent::ConfigChanged { .. } => {
                                info!("Config Changed: {:#?}", app.config());
                            }
                            MainEvent::LowMemory => {}

                            MainEvent::Destroy => quit = true,
                            _ => { /* ... */ }
                        }
                    }
                    _ => {}
                }

                if redraw_pending {
                    if let Some(native_window) = &native_window {
                        redraw_pending = false;

                        // Handle input
                        app.input_events(|event| {
                            info!("Input Event: {event:?}");
                            InputStatus::Unhandled
                        });

                        info!("Render...");
                        quit = match render(native_window, &state) {
                            Some(()) => false,
                            None => true,
                        };
                    }
                }
            },
        );
        state.t = state.t.wrapping_add(1);
    }
}

fn render(native_window: &ndk::native_window::NativeWindow, state: &State) -> Option<()> {
    let nw_ptr: *mut ndk_sys::ANativeWindow = native_window.ptr().as_ptr() as _;

    let mut buf: MaybeUninit<ndk_sys::ANativeWindow_Buffer> = MaybeUninit::uninit();
    let mut rect: MaybeUninit<ndk_sys::ARect> = MaybeUninit::uninit();

    let buf: ndk_sys::ANativeWindow_Buffer = unsafe {
        // this is safe because all pointers are valid by construction
        ndk_sys::ANativeWindow_lock(nw_ptr, buf.as_mut_ptr(), rect.as_mut_ptr());

        // this is safe because ANativeWindow_lock initializes the buffer and rect
        let buf: ndk_sys::ANativeWindow_Buffer = buf.assume_init();
        // at the moment of writing, we don't use the rect
        let _rect: ndk_sys::ARect = rect.assume_init();
        buf
    };

    // =================== HERE'S WHERE YOU DRAW ===================================

    // Assumes R8G8B8A8_UNORM format
    for y in 0..buf.height {
        for x in 0..buf.width {
            const SIZE_OF_PIXEL: i32 = 4;
            let offset = ((y * buf.stride + x) * SIZE_OF_PIXEL) as isize;
            // This is safe because the size of the buffer is buf.stride * buf.height * SIZE_OF_PIXEL
            let pixel = unsafe { buf.bits.offset(offset) } as *mut [u8; 4];
            // This is safe because
            // 1. the dst pointer is valid for writes because it points to a pixel
            // in a locked buffer
            // 2. the dst pointer is properly aligned because it points to a pixel
            unsafe {
                core::ptr::write(pixel, [state.t, state.t, state.t, 255]);
            }
        }
    }
    // ==============================================================================

    unsafe {
        ndk_sys::ANativeWindow_unlockAndPost(nw_ptr);
    }
    Some(())
}
