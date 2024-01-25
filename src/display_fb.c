#include <stdio.h>
#include <stdlib.h>

/* Screen buffer Stuff */
#include <linux/fb.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>

#include "display.h"

typedef struct display_fb {
	volatile uint8_t* fb;
	int fb_fd;
	uint8_t bytes_per_pixel;
} display_fb_t;

display_t* display_init()
{
	printf("Initializing display_fb\n");

	display_t* display = malloc(sizeof(display_t));

	if (display == NULL) {
		exit(EXIT_FAILURE);
	}

	display_fb_t* display_fb = malloc(sizeof(display_fb_t));

	if (display_fb == NULL) {
		exit(EXIT_FAILURE);
	}

	display_fb->fb_fd = open("/dev/fb0", O_RDWR);

	struct fb_var_screeninfo var_info;
	struct fb_fix_screeninfo fix_info;

	ioctl(display_fb->fb_fd, FBIOGET_VSCREENINFO, &var_info);
	ioctl(display_fb->fb_fd, FBIOGET_FSCREENINFO, &fix_info);

	display->extra = (void*) display_fb;
	display->width = var_info.xres_virtual;
	display->height = var_info.yres_virtual;
	display->bytes_per_pixel = var_info.bits_per_pixel / 8;

	display_fb->fb = mmap(0, fix_info.line_length * var_info.yres_virtual, PROT_READ | PROT_WRITE, MAP_SHARED, display_fb->fb_fd, 0);

	return (display_t*) display;
}

void display_render_frame(display_t* display, buffer_t* buffer)
{
	display_fb_t* display_fb = (display_fb_t*) display->extra;

	for (int i = 0; i < buffer->size; i++) {
		*(display_fb->fb + i) = buffer->data[i];
	}
}
