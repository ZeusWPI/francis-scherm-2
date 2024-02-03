#ifndef FS2_DISPLAY
#define FS2_DISPLAY

#include <stdint.h>

#include "buffer.h"

typedef struct display {
	uint16_t width;
	uint16_t height;
	uint8_t bytes_per_pixel;
	void * extra;
} display_t;

display_t* display_init();

void display_render(display_t*, buffer_t*);

#endif /* FS2_DISPLAY */
