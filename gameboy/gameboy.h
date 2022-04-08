#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define VERTICAL_RES 144

#define HORIZONTAL_RES 160

#define PIXELS_PER_BYTE 4

#define BUFFER_SIZE (((uintptr_t)VERTICAL_RES * (uintptr_t)HORIZONTAL_RES) / (uintptr_t)PIXELS_PER_BYTE)

#define RETRO_ENVIRONMENT_SET_PIXEL_FORMAT 10

#define RETRO_ENVIRONMENT_GET_LOG_INTERFACE 27

#define RETRO_ENVIRONMENT_SET_CONTROLLER_INFO 35

#define RETRO_DEVICE_TYPE_SHIFT 8

#define RETRO_DEVICE_MASK ((1 << RETRO_DEVICE_TYPE_SHIFT) - 1)

enum Key {
  Up,
  Down,
  Left,
  Right,
  A,
  B,
  Start,
  Select,
};
typedef uint32_t Key;

typedef struct System System;

typedef struct Vec_InputEvent Vec_InputEvent;

typedef struct SystemHandle {
  struct System system;
  struct Vec_InputEvent events_buffer;
} SystemHandle;

typedef struct SystemInitializationOptions {
  uint32_t boot_rom_length;
  const uint8_t *boot_rom;
  uint32_t game_rom_length;
  const uint8_t *game_rom;
  uint32_t external_ram_length;
  const uint8_t *external_ram;
  bool debug_mode;
} SystemInitializationOptions;

typedef struct Event {
  Key key;
  bool is_pressed;
} Event;

typedef bool (*GetEnvironmentInfo)(uint32_t cmd, void *data);

typedef void (*RefreshVideo)(const void *data, uint32_t width, uint32_t height, uintptr_t pitch);

typedef void (*RenderAudioFrame)(uint16_t left, uint16_t right);

typedef void (*RenderAudioBatch)(const uint16_t *data, uintptr_t frames);

typedef void (*PollInput)(void);

typedef int16_t (*QueryInputState)(uint32_t port, uint32_t device, uint32_t index, uint32_t id);

typedef struct RetroSystemInfo {
  const uint32_t *library_name;
  const uint32_t *library_version;
  const uint32_t *valid_extensions;
  bool need_fullpath;
  bool block_extract;
} RetroSystemInfo;

typedef struct RetroGameInfo {
  const uint32_t *path;
  const void *data;
  uintptr_t size;
  const uint32_t *meta;
} RetroGameInfo;

typedef struct RetroGameGeometry {
  uint32_t base_width;
  uint32_t base_height;
  uint32_t max_width;
  uint32_t max_height;
  float aspect_ratio;
} RetroGameGeometry;

typedef struct RetroSystemTiming {
  double fps;
  double sample_rate;
} RetroSystemTiming;

typedef struct RetroSystemAvInfo {
  struct RetroGameGeometry geometry;
  struct RetroSystemTiming timing;
} RetroSystemAvInfo;

struct SystemHandle *gameboy_create_system(struct SystemInitializationOptions options);

void gameboy_add_event(struct SystemHandle *handle, struct Event event);

uint32_t gameboy_framebuffer_size(void);

bool gameboy_run_single_frame(struct SystemHandle *handle, uint8_t *output);

bool gameboy_is_exit_requested(const struct SystemHandle *handle);

void gameboy_request_exit(struct SystemHandle *handle);

void gameboy_destroy_system(struct SystemHandle *handle);

uint32_t retro_api_version(void);

void retro_set_environment(GetEnvironmentInfo get_environment_info);

void retro_set_video_refresh(RefreshVideo refresh_video);

void retro_set_audio_sample(RenderAudioFrame render_audio_frame);

void retro_set_audio_sample_batch(RenderAudioBatch render_audio_batch);

void retro_set_input_poll(PollInput poll_input);

void retro_set_input_state(QueryInputState query_input_state);

void retro_init(void);

void retro_deinit(void);

void retro_reset(void);

void retro_set_controller_port_device(uint32_t _port, uint32_t _device);

void retro_get_system_info(struct RetroSystemInfo *info);

bool retro_load_game(const struct RetroGameInfo *game);

void retro_get_system_av_info(struct RetroSystemAvInfo *info);

void retro_run(void);

uintptr_t retro_serialize_size(void);

bool retro_serialize(void *_data, uintptr_t _size);

bool retro_unserialize(void *_data, uintptr_t _size);

void retro_cheat_reset(void);

void retro_cheat_set(uint32_t _index, bool _enabled, const uint32_t *_code);

bool retro_load_game_special(uint32_t _game_type,
                             const struct RetroGameInfo *_info,
                             uintptr_t _num_info);

void retro_unload_game(void);

uint32_t retro_get_region(void);

void *retro_get_memory_data(uint32_t _id);

uintptr_t retro_get_memory_size(uint32_t _id);
