#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define VERTICAL_RES 144

#define HORIZONTAL_RES 160

#define PIXELS_PER_BYTE 4

#define BUFFER_SIZE (((uintptr_t)VERTICAL_RES * (uintptr_t)HORIZONTAL_RES) / (uintptr_t)PIXELS_PER_BYTE)

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

struct SystemHandle *gameboy_create_system(struct SystemInitializationOptions options);

void gameboy_add_event(struct SystemHandle *handle, struct Event event);

void gameboy_run_single_frame(struct SystemHandle *handle,
                              uint8_t *output,
                              uint32_t *width,
                              uint32_t *height);

bool gameboy_is_exit_requested(const struct SystemHandle *handle);

void gameboy_request_exit(struct SystemHandle *handle);

void gameboy_destroy_system(struct SystemHandle *handle);
