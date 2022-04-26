pub mod libretro_types;

use crate::libretro_types::*;
use gameboy::{Button, ButtonState, InitializationOptions, InputEvent, System};
use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_void;

#[no_mangle]
pub unsafe extern "C" fn retro_api_version() -> u32 {
    return 1;
}

struct LibRetroCallbacks {
    get_environment_info: Option<GetEnvironmentInfo>,
    refresh_video: Option<RefreshVideo>,
    render_audio_frame: Option<RenderAudioFrame>,
    render_audio_batch: Option<RenderAudioBatch>,
    poll_input: Option<PollInput>,
    query_input_state: Option<QueryInputState>,
}

impl LibRetroCallbacks {
    fn new() -> Self {
        Self {
            get_environment_info: None,
            refresh_video: None,
            render_audio_frame: None,
            render_audio_batch: None,
            poll_input: None,
            query_input_state: None,
        }
    }
}

struct SystemInfo {
    library_name: CString,
    library_version: CString,
    valid_extensions: CString,
}

thread_local! {
    static CALLBACKS: RefCell<LibRetroCallbacks> = RefCell::new(LibRetroCallbacks::new());
    static SYSTEM_INFO: RefCell<Option<SystemInfo>> = RefCell::new(None);
    static SYSTEM: RefCell<Option<System>> = RefCell::new(None);
    static OUTPUT_FRAMEBUFFER: RefCell<[u8; 144 * 160 * 4]> = RefCell::new([0; 144 * 160 * 4]);
    static OUTPUT_SOUND_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(48000));
    static LOG: RefCell<Option<RetroLogPrintf>> = RefCell::new(None);
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_environment(get_environment_info: GetEnvironmentInfo) {
    CALLBACKS.with(|state| {
        state.borrow_mut().get_environment_info = Some(get_environment_info);

        let mut log_callback = std::mem::MaybeUninit::<RetroLogCallback>::zeroed();
        if get_environment_info(
            RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
            log_callback.as_mut_ptr() as *mut c_void,
        ) {
            let log_callback = log_callback.assume_init();
            LOG.with(|log| {
                *log.borrow_mut() = Some(log_callback.log);
                let message = CString::new("Log function received!!!").unwrap();
                (log_callback.log)(RetroLogLevel::Info, message.as_ptr() as *const char);
            });
        }

        let description = CString::new("Gameboy Pad").unwrap();
        let controller_desc = RetroControllerDescription {
            desc: description.as_ptr() as *const char,
            id: retro_device_subclass(RetroDevice::Joypad, 0),
        };
        let mut controller_info = [
            RetroControllerInfo {
                num_types: 1,
                types: &controller_desc as *const RetroControllerDescription,
            },
            RetroControllerInfo {
                num_types: 0,
                types: std::ptr::null(),
            },
        ];
        get_environment_info(
            RETRO_ENVIRONMENT_SET_CONTROLLER_INFO,
            &mut controller_info as *mut RetroControllerInfo as *mut c_void,
        );
    });
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_video_refresh(refresh_video: RefreshVideo) {
    CALLBACKS.with(|state| {
        state.borrow_mut().refresh_video = Some(refresh_video);
    });
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample(render_audio_frame: RenderAudioFrame) {
    CALLBACKS.with(|state| {
        state.borrow_mut().render_audio_frame = Some(render_audio_frame);
    });
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_audio_sample_batch(render_audio_batch: RenderAudioBatch) {
    CALLBACKS.with(|state| {
        state.borrow_mut().render_audio_batch = Some(render_audio_batch);
    });
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_poll(poll_input: PollInput) {
    CALLBACKS.with(|state| {
        state.borrow_mut().poll_input = Some(poll_input);
    });
}

#[no_mangle]
pub unsafe extern "C" fn retro_set_input_state(query_input_state: QueryInputState) {
    CALLBACKS.with(|state| {
        state.borrow_mut().query_input_state = Some(query_input_state);
    });
}

#[no_mangle]
pub unsafe extern "C" fn retro_init() {}

#[no_mangle]
pub unsafe extern "C" fn retro_deinit() {}

#[no_mangle]
pub unsafe extern "C" fn retro_reset() {}

// Sets device to be used for player 'port'.
// By default, RETRO_DEVICE_JOYPAD is assumed to be plugged into all
// available ports.
// Setting a particular device type is not a guarantee that libretro cores
// will only poll input based on that particular device type. It is only a
// hint to the libretro core when a core cannot automatically detect the
// appropriate input device type on its own. It is also relevant when a
// core can change its behavior depending on device type.

// As part of the core's implementation of retro_set_controller_port_device,
// the core should call RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS to notify the
// frontend if the descriptions for any controls have changed as a
// result of changing the device type.
#[no_mangle]
pub unsafe extern "C" fn retro_set_controller_port_device(_port: u32, _device: u32) {}

// Gets statically known system info. Pointers provided in *info
// must be statically allocated.
// Can be called at any time, even before retro_init().
#[no_mangle]
pub unsafe extern "C" fn retro_get_system_info(info: *mut RetroSystemInfo) {
    SYSTEM_INFO.with(|data| {
        let mut data = data.borrow_mut();
        if data.is_none() {
            *data = Some(SystemInfo {
                library_name: CString::new("rustgameboycore").unwrap(),
                library_version: CString::new("0.0.2").unwrap(),
                valid_extensions: CString::new("gb|gbc").unwrap(),
            });
        }

        if let Some(data) = data.as_ref() {
            (*info).library_name = data.library_name.as_ptr() as *const char;
            (*info).library_version = data.library_version.as_ptr() as *const char;
            (*info).valid_extensions = data.valid_extensions.as_ptr() as *const char;
        }
    });

    (*info).need_fullpath = false;
    (*info).block_extract = false;
}

// Loads a game.
// Return true to indicate successful loading and false to indicate load failure.
#[no_mangle]
pub unsafe extern "C" fn retro_load_game(game: *const RetroGameInfo) -> bool {
    let mut supports_format = false;
    CALLBACKS.with(|callback| {
        let callback = callback.borrow();
        let mut format = RetroPixelFormat::RetroPixelFormatXRGB8888 as i32;

        supports_format = callback.get_environment_info.unwrap()(
            RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
            &mut format as *mut i32 as *mut c_void,
        );
    });

    if !supports_format {
        return false;
    }

    let game_rom: &[u8] = std::slice::from_raw_parts((*game).data.cast(), (*game).size);
    SYSTEM.with(|system| {
        let mut system = system.borrow_mut();
        *system = Some(System::new(crate::InitializationOptions {
            boot_rom: None,
            game_rom,
            debug_mode: false,
            external_ram: None,
            sound_frequency: 48000,
        }));
    });

    true
}

// Gets information about system audio/video timings and geometry.
// Can be called only after retro_load_game() has successfully completed.
// NOTE: The implementation of this function might not initialize every
// variable if needed.
// E.g. geom.aspect_ratio might not be initialized if core doesn't
// desire a particular aspect ratio.
#[no_mangle]
pub unsafe extern "C" fn retro_get_system_av_info(info: *mut RetroSystemAvInfo) {
    (*info).geometry.base_width = 160;
    (*info).geometry.base_height = 144;
    (*info).geometry.max_width = 160;
    (*info).geometry.max_height = 144;
    (*info).geometry.aspect_ratio = 0.0;
    (*info).timing.fps = 60.0;
    (*info).timing.sample_rate = 0.0;
}

// Runs the game for one video frame.
// During retro_run(), input_poll callback must be called at least once.

// If a frame is not rendered for reasons where a game "dropped" a frame,
// this still counts as a frame, and retro_run() should explicitly dupe
// a frame if GET_CAN_DUPE returns true.
// In this case, the video callback can take a NULL argument for data.
#[no_mangle]
pub unsafe extern "C" fn retro_run() {
    SYSTEM.with(|system| {
        let mut system = system.borrow_mut();
        let system = system.as_mut().unwrap();
        CALLBACKS.with(|callbacks| {
            let callbacks = callbacks.borrow();

            callbacks.poll_input.unwrap()();

            let query_input_state = callbacks.query_input_state.unwrap();
            let mut input_events = [InputEvent {
                button: Button::A,
                state: ButtonState::Pressed,
            }; 8];

            update_button(
                &mut input_events[0],
                Button::A,
                JoypadInput::A,
                query_input_state,
            );
            update_button(
                &mut input_events[1],
                Button::B,
                JoypadInput::B,
                query_input_state,
            );
            update_button(
                &mut input_events[2],
                Button::Start,
                JoypadInput::Start,
                query_input_state,
            );
            update_button(
                &mut input_events[3],
                Button::Select,
                JoypadInput::Select,
                query_input_state,
            );
            update_button(
                &mut input_events[4],
                Button::Up,
                JoypadInput::Up,
                query_input_state,
            );
            update_button(
                &mut input_events[5],
                Button::Left,
                JoypadInput::Left,
                query_input_state,
            );
            update_button(
                &mut input_events[6],
                Button::Down,
                JoypadInput::Down,
                query_input_state,
            );
            update_button(
                &mut input_events[7],
                Button::Right,
                JoypadInput::Right,
                query_input_state,
            );

            OUTPUT_FRAMEBUFFER.with(|video_output| {
                OUTPUT_SOUND_BUFFER.with(|sound_output| {
                    let mut video_buffer = video_output.borrow_mut();
                    let mut sound_buffer = sound_output.borrow_mut();
                    system.run_single_frame(
                        &input_events,
                        video_buffer.as_mut_slice(),
                        sound_buffer.as_mut(),
                    );

                    callbacks.refresh_video.unwrap()(
                        video_buffer.as_ptr() as *const c_void,
                        System::screen_width(),
                        System::screen_height(),
                        System::screen_width() as usize * 4,
                    );
                });
            });
            // }
        });
    });
}

unsafe fn update_button(
    input_event: &mut InputEvent,
    button: Button,
    input: JoypadInput,
    query_input_state: QueryInputState,
) {
    input_event.button = button;
    input_event.state = if query_input_state(0, RetroDevice::Joypad as u32, 0, input as u32) > 0 {
        ButtonState::Pressed
    } else {
        ButtonState::Released
    };
}

// Returns the amount of data the implementation requires to serialize
// internal state (save states).
// Between calls to retro_load_game() and retro_unload_game(), the
// returned size is never allowed to be larger than a previous returned
// value, to ensure that the frontend can allocate a save state buffer once.
#[no_mangle]
pub unsafe extern "C" fn retro_serialize_size() -> usize {
    0
}

// Serializes internal state. If failed, or size is lower than
// retro_serialize_size(), it should return false, true otherwise.
#[no_mangle]
pub unsafe extern "C" fn retro_serialize(_data: *mut c_void, _size: usize) -> bool {
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn retro_unserialize(_data: *mut c_void, _size: usize) -> bool {
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn retro_cheat_reset() {}

#[no_mangle]
pub unsafe extern "C" fn retro_cheat_set(_index: u32, _enabled: bool, _code: *const char) {}

// Loads a "special" kind of game. Should not be used,
// except in extreme cases.
#[no_mangle]
pub unsafe extern "C" fn retro_load_game_special(
    _game_type: u32,
    _info: *const RetroGameInfo,
    _num_info: usize,
) -> bool {
    return false;
}

//  Unloads the currently loaded game. Called before retro_deinit(void).
#[no_mangle]
pub unsafe extern "C" fn retro_unload_game() {}

//  Gets region of game.
#[no_mangle]
pub unsafe extern "C" fn retro_get_region() -> u32 {
    RetroRegion::Ntsc as u32
}

//  Gets region of memory.
#[no_mangle]
pub unsafe extern "C" fn retro_get_memory_data(_id: u32) -> *mut c_void {
    return std::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn retro_get_memory_size(_id: u32) -> usize {
    return 0;
}
