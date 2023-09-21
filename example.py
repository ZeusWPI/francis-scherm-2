from client import Pixels
from PIL import Image
from time import sleep

# Get pixel class
pixel_class = Pixels()
screen_width, screen_height = pixel_class.get_size()

# Get image data
image = Image.open("zeus_logo.png")
width, height = image.size
aspect_ratio = width / height

# Resize if needed
resized = False
if width > screen_width:
	resized = True
	width = screen_width
	height = int(width / aspect_ratio)

if height > screen_height:
	resized = True
	height = screen_height
	width = int(height * aspect_ratio)
if resized:
	image = image.resize((width, height), Image.ANTIALIAS)

# Image data
image = image.convert('RGB')
pixels = list(image.getdata())

image.close()


def clear():
	"""
	Clear the screen
	"""
	for i in range(0, screen_width):
		for j in range(0, screen_height):
			pixel_class.send_pixel((i, j), (0, 0, 0))


def send_pixel_per_pixel():
	""""
	Send pixel per pixel
	"""
	for i in range(0, screen_width):
		for j in range(0, screen_height):
			if i < width and j < height:
				index = j * width + i
				pixel_class.send_pixel((i, j), pixels[index])
			else:
				pixel_class.send_pixel((i, j), (0, 0, 0))


def send_all_pixels_at_once():
	""""
	Send all pixels at once
	"""
	coordinates = []
	colours = []
	for i in range(0, screen_width):
		for j in range(0, screen_height):
			coordinates.append((i, j))
			if i < width and j < height:
				index = j * width + i
				colours.append(pixels[index])
			else:
				colours.append((0, 0, 0))
	pixel_class.send_many_pixels(coordinates, colours)


if __name__ == "__main__":
	clear()
	sleep(2)

	send_pixel_per_pixel()
	sleep(2)

	clear()
	sleep(2)

	send_all_pixels_at_once()
	sleep(2)

	clear()
