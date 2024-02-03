#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "display.h"

typedef struct display_sdl {
	SDL_Window* window;
	SDL_Renderer* renderer;
	SDL_Texture* texture;
	TTF_Font* font;
} display_sdl_t;

display_t* display_init()
{
	printf("Initializing display_fb\n");

	display_t* display = malloc(sizeof(display_t));

	if (display == NULL) {
		exit(EXIT_FAILURE);
	}

	display_sdl_t* display_sdl = malloc(sizeof(display_sdl_t));

	if (display_sdl == NULL) {
		exit(EXIT_FAILURE);
	}

	SDL_Init(SDL_INIT_VIDEO);
	TTF_Init();

	display_sdl->window = SDL_CreateWindow("francis-scherm-1", 0, 0, 640, 480, 0);
	display_sdl->renderer = SDL_CreateRenderer(display_sdl->window, -1, 0);
	display_sdl->texture = SDL_CreateTexture(display_sdl->renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STATIC, 640, 480);
	display_sdl->font = TTF_OpenFont("ariblk.ttf", 12);

	if (! display_sdl->font) {
		exit(EXIT_FAILURE);
	}

	display->extra = (void*) display_sdl;
	display->height = 480;
	display->width = 640;
	display->bytes_per_pixel = 4;

	return (display_t*) display;
}

void display_render(display_t* display, buffer_t* buffer)
{
	display_sdl_t* display_sdl = (display_sdl_t*) display->extra;

	SDL_Colour fg = {0, 0, 0};
	SDL_Colour bg = {255, 255, 255};

	char buff[256] = { 0 };
	sprintf(buff, " %dx%d ", display->width, display->height);

	SDL_Surface* text = TTF_RenderText_Shaded(display_sdl->font, buff, fg, bg);
	SDL_Texture* text_texture = SDL_CreateTextureFromSurface(display_sdl->renderer, text);

	int render_text = 0;

	while (1) {

		for (int i = 0; i < buffer->size; i++) {
			SDL_SetRenderDrawColor(
				display_sdl->renderer,
				buffer->data[i],
				buffer->data[i+1],
				buffer->data[i+2],
				buffer->data[i+3]
			);
			SDL_RenderDrawPoint(display_sdl->renderer, i / 640, i % 480);
		}

		SDL_UpdateTexture(display_sdl->texture, NULL, (uint32_t *) buffer->data, 640 * sizeof(Uint32));

		SDL_RenderClear(display_sdl->renderer);
		SDL_RenderCopy(display_sdl->renderer, display_sdl->texture, NULL, NULL);

		if (render_text) {
			SDL_Rect dest = {
				.x = 10,
				.y = 10,
				.w = text->w,
				.h = text->h
			};

			SDL_RenderCopy(display_sdl->renderer, text_texture, NULL, &dest);
		}
		SDL_RenderPresent(display_sdl->renderer);

		SDL_Event e;
		while(SDL_PollEvent(&e) > 0)
		{
			switch(e.type)
			{
			case SDL_QUIT:
				exit(EXIT_FAILURE);
				break;

			case SDL_KEYDOWN:
				if (e.key.keysym.scancode == SDL_SCANCODE_S) {
					render_text = ! render_text;
				};
				break;
			}
		}

		SDL_Delay(10);
	}

	SDL_DestroyTexture(text_texture);
	SDL_FreeSurface(text);
}
