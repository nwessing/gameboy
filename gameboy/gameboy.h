#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define VERTICAL_RES 144

#define HORIZONTAL_RES 160

#define PIXELS_PER_BYTE 4

#define BUFFER_SIZE (((uintptr_t)VERTICAL_RES * (uintptr_t)HORIZONTAL_RES) / (uintptr_t)PIXELS_PER_BYTE)
