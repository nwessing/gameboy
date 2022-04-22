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
