use std::os::raw::c_void;

/* Environment callback. Gives implementations a way of performing
 * uncommon tasks. Extensible. */
pub type GetEnvironmentInfo = unsafe extern "C" fn(cmd: u32, data: *mut c_void) -> bool;

/* Render a frame. Pixel format is 15-bit 0RGB1555 native endian
 * unless changed (see RETRO_ENVIRONMENT_SET_PIXEL_FORMAT).
 *
 * Width and height specify dimensions of buffer.
 * Pitch specifices length in bytes between two lines in buffer.
 *
 * For performance reasons, it is highly recommended to have a frame
 * that is packed in memory, i.e. pitch == width * byte_per_pixel.
 * Certain graphic APIs, such as OpenGL ES, do not like textures
 * that are not packed in memory.
 */
pub type RefreshVideo =
    unsafe extern "C" fn(data: *const c_void, width: u32, height: u32, pitch: usize);

/* Renders a single audio frame. Should only be used if implementation
 * generates a single sample at a time.
 * Format is signed 16-bit native endian.
 */
pub type RenderAudioFrame = unsafe extern "C" fn(left: u16, right: u16);

/* Renders multiple audio frames in one go.
 *
 * One frame is defined as a sample of left and right channels, interleaved.
 * I.e. int16_t buf[4] = { l, r, l, r }; would be 2 frames.
 * Only one of the audio callbacks must ever be used.
 */
pub type RenderAudioBatch = unsafe extern "C" fn(data: *const u16, frames: usize);

/* Polls input. */
pub type PollInput = unsafe extern "C" fn();

/* Queries for input for player 'port'. device will be masked with
 * RETRO_DEVICE_MASK.
 *
 * Specialization of devices such as RETRO_DEVICE_JOYPAD_MULTITAP that
 * have been set with retro_set_controller_port_device()
 * will still use the higher level RETRO_DEVICE_JOYPAD to request input.
 */
pub type QueryInputState = unsafe extern "C" fn(port: u32, device: u32, index: u32, id: u32) -> i16;

#[repr(C)]
pub struct RetroSystemInfo {
    /* All pointers are owned by libretro implementation, and pointers must
     * remain valid until it is unloaded. */
    pub library_name: *const char, /* Descriptive name of library. Should not
                                    * contain any version numbers, etc. */
    pub library_version: *const char, /* Descriptive version of core. */

    pub valid_extensions: *const char, /* A string listing probably content
                                        * extensions the core will be able to
                                        * load, separated with pipe.
                                        * I.e. "bin|rom|iso".
                                        * Typically used for a GUI to filter
                                        * out extensions. */

    /* Libretro cores that need to have direct access to their content
     * files, including cores which use the path of the content files to
     * determine the paths of other files, should set need_fullpath to true.
     *
     * Cores should strive for setting need_fullpath to false,
     * as it allows the frontend to perform patching, etc.
     *
     * If need_fullpath is true and retro_load_game() is called:
     *    - retro_game_info::path is guaranteed to have a valid path
     *    - retro_game_info::data and retro_game_info::size are invalid
     *
     * If need_fullpath is false and retro_load_game() is called:
     *    - retro_game_info::path may be NULL
     *    - retro_game_info::data and retro_game_info::size are guaranteed
     *      to be valid
     *
     * See also:
     *    - RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY
     *    - RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY
     */
    pub need_fullpath: bool,

    /* If true, the frontend is not allowed to extract any archives before
     * loading the real content.
     * Necessary for certain libretro implementations that load games
     * from zipped archives. */
    pub block_extract: bool,
}

#[repr(C)]
pub struct RetroGameInfo {
    pub path: *const char, /* Path to game, UTF-8 encoded.
                            * Sometimes used as a reference for building other paths.
                            * May be NULL if game was loaded from stdin or similar,
                            * but in this case some cores will be unable to load `data`.
                            * So, it is preferable to fabricate something here instead
                            * of passing NULL, which will help more cores to succeed.
                            * retro_system_info::need_fullpath requires
                            * that this path is valid. */
    pub data: *const c_void, /* Memory buffer of loaded game. Will be NULL
                              * if need_fullpath was set. */
    pub size: usize,       /* Size of memory buffer. */
    pub meta: *const char, /* String of implementation specific meta-data. */
}

#[repr(C)]
pub struct RetroGameGeometry {
    pub base_width: u32,  /* Nominal video width of game. */
    pub base_height: u32, /* Nominal video height of game. */
    pub max_width: u32,   /* Maximum possible width of game. */
    pub max_height: u32,  /* Maximum possible height of game. */

    pub aspect_ratio: f32, /* Nominal aspect ratio of game. If
                            * aspect_ratio is <= 0.0, an aspect ratio
                            * of base_width / base_height is assumed.
                            * A frontend could override this setting,
                            * if desired. */
}

#[repr(C)]
pub struct RetroSystemTiming {
    pub fps: f64,         /* FPS of video content. */
    pub sample_rate: f64, /* Sampling rate of audio. */
}

#[repr(C)]
pub struct RetroSystemAvInfo {
    pub geometry: RetroGameGeometry,
    pub timing: RetroSystemTiming,
}

pub const RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: u32 = 10;

/* struct retro_log_callback * --
 * Gets an interface for logging. This is useful for
 * logging in a cross-platform way
 * as certain platforms cannot use stderr for logging.
 * It also allows the frontend to
 * show logging information in a more suitable way.
 * If this interface is not used, libretro cores should
 * log to stderr as desired.
 */
pub const RETRO_ENVIRONMENT_GET_LOG_INTERFACE: u32 = 27;

/* const struct retro_controller_info * --
 * This environment call lets a libretro core tell the frontend
 * which controller subclasses are recognized in calls to
 * retro_set_controller_port_device().
 *
 * Some emulators such as Super Nintendo support multiple lightgun
 * types which must be specifically selected from. It is therefore
 * sometimes necessary for a frontend to be able to tell the core
 * about a special kind of input device which is not specifcally
 * provided by the Libretro API.
 *
 * In order for a frontend to understand the workings of those devices,
 * they must be defined as a specialized subclass of the generic device
 * types already defined in the libretro API.
 *
 * The core must pass an array of const struct retro_controller_info which
 * is terminated with a blanked out struct. Each element of the
 * retro_controller_info struct corresponds to the ascending port index
 * that is passed to retro_set_controller_port_device() when that function
 * is called to indicate to the core that the frontend has changed the
 * active device subclass. SEE ALSO: retro_set_controller_port_device()
 *
 * The ascending input port indexes provided by the core in the struct
 * are generally presented by frontends as ascending User # or Player #,
 * such as Player 1, Player 2, Player 3, etc. Which device subclasses are
 * supported can vary per input port.
 *
 * The first inner element of each entry in the retro_controller_info array
 * is a retro_controller_description struct that specifies the names and
 * codes of all device subclasses that are available for the corresponding
 * User or Player, beginning with the generic Libretro device that the
 * subclasses are derived from. The second inner element of each entry is the
 * total number of subclasses that are listed in the retro_controller_description.
 *
 * NOTE: Even if special device types are set in the libretro core,
 * libretro should only poll input based on the base input device types.
 */
pub const RETRO_ENVIRONMENT_SET_CONTROLLER_INFO: u32 = 35;

#[repr(i32)]
pub enum RetroPixelFormat {
    /* 0RGB1555, native endian.
     * 0 bit must be set to 0.
     * This pixel format is default for compatibility concerns only.
     * If a 15/16-bit pixel format is desired, consider using RGB565. */
    RetroPixelFormat0RGB1555 = 0,

    /* XRGB8888, native endian.
     * X bits are ignored. */
    RetroPixelFormatXRGB8888 = 1,

    /* RGB565, native endian.
     * This pixel format is the recommended format to use if a 15/16-bit
     * format is desired as it is the pixel format that is typically
     * available on a wide range of low-power devices.
     *
     * It is also natively supported in APIs like OpenGL ES. */
    RetroPixelFormatRGB565 = 2,

    /* Ensure sizeof() == sizeof(int). */
    RetroPixelFormatUnknown = i32::max_value(),
}

#[repr(u32)]
pub enum RetroLogLevel {
    Debug = 0,
    Info,
    Warn,
    Error,
    Dummy = u32::max_value(),
}

/* Logging function. Takes log level argument as well. */
pub type RetroLogPrintf = unsafe extern "C" fn(level: RetroLogLevel, fmt: *const char, ...);

#[repr(C)]
pub struct RetroLogCallback {
    pub log: RetroLogPrintf,
}

#[repr(u32)]
pub enum RetroRegion {
    Ntsc = 0,
    Pal = 1,
}

pub const RETRO_DEVICE_TYPE_SHIFT: u32 = 8;
pub const RETRO_DEVICE_MASK: u32 = (1 << RETRO_DEVICE_TYPE_SHIFT) - 1;
pub fn retro_device_subclass(base: RetroDevice, id: u32) -> u32 {
    ((id + 1) << RETRO_DEVICE_TYPE_SHIFT) | base as u32
}

#[repr(u32)]
pub enum RetroDevice {
    /* Input disabled. */
    None = 0,

    /* The JOYPAD is called RetroPad. It is essentially a Super Nintendo
     * controller, but with additional L2/R2/L3/R3 buttons, similar to a
     * PS1 DualShock. */
    Joypad = 1,
}

#[repr(u32)]
pub enum JoypadInput {
    /* Buttons for the RetroPad (JOYPAD).
     * The placement of these is equivalent to placements on the
     * Super Nintendo controller.
     * L2/R2/L3/R3 buttons correspond to the PS1 DualShock.
     * Also used as id values for RETRO_DEVICE_INDEX_ANALOG_BUTTON */
    B = 0,
    Y = 1,
    Select = 2,
    Start = 3,
    Up = 4,
    Down = 5,
    Left = 6,
    Right = 7,
    A = 8,
    X = 9,
    L = 10,
    R = 11,
    L2 = 12,
    R2 = 13,
    L3 = 14,
    R3 = 15,
}

#[repr(C)]
pub struct RetroControllerDescription {
    /* Human-readable description of the controller. Even if using a generic
     * input device type, this can be set to the particular device type the
     * core uses. */
    pub desc: *const char,

    /* Device type passed to retro_set_controller_port_device(). If the device
     * type is a sub-class of a generic input device type, use the
     * RETRO_DEVICE_SUBCLASS macro to create an ID.
     *
     * E.g. RETRO_DEVICE_SUBCLASS(RETRO_DEVICE_JOYPAD, 1). */
    pub id: u32,
}

#[repr(C)]
pub struct RetroControllerInfo {
    pub types: *const RetroControllerDescription,
    pub num_types: u32,
}
