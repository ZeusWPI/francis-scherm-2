import struct
import socket
from typing import Tuple, List


# Class to send pixels to a screen
class Pixels:
	def __init__(self):
		self.client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		self.client_socket.connect(("10.0.0.8", 8000))
		self._x, self._y = struct.unpack('>HH', self.client_socket.recv(4))

	def get_size(self) -> Tuple[int, int]:
		"""
		Returns the size of the screen.\n
		Be aware that even though the screen can be 200 pixels wide, the last pixel has index 199.
		The numbering starts from 0.
		"""
		return self._x, self._y

	def _validate_coordinate(self, coordinate: Tuple[int, int]):
		if not isinstance(coordinate, tuple) or len(coordinate) != 2 or not all(isinstance(coord, int) for coord in coordinate):
			raise ValueError("The coordinate has to be a tuple. \nAn example is (5, 10)")
		x, y = coordinate
		if not (0 <= x < self._x) or not (0 <= y < self._y):
			raise ValueError(f"Invalid coordinate ({x}, {y})! It has to be between (0, 0) and ({self._x - 1}, {self._y - 1}).")

	def _validate_colour(self, colour: Tuple[int, int, int]):
		if not isinstance(colour, tuple) or len(colour) != 3 or not all(isinstance(comp, int) for comp in colour):
			raise ValueError("The colour has to represent a RGB value. \nFor example yellow is (255, 255, 0)")
		if not all(0 <= comp <= 255 for comp in colour):
			raise ValueError(f"Invalid colour ({colour})! \nEvery component has to be between 0 and 255.")

	def send_pixel(self, coordinate: Tuple[int, int], colour: Tuple[int, int, int]):
		"""
		Send a pixel to the screen\n
		:param coordinate: A tuple containing the coordinate. E.g. (5, 10)
		:param colour: A RGB value. For example yellow is (255, 255, 0)
		"""

		# Check arguments
		self._validate_coordinate(coordinate)
		self._validate_colour(colour)

		# Send pixel
		packet = struct.pack('>hhBBB', coordinate[0], coordinate[1], colour[0], colour[1], colour[2])
		self.client_socket.send(packet)

	def send_many_pixels(self, coordinates: List[Tuple[int, int]], colours: List[Tuple[int, int, int]]):
		"""
		Send multiple pixels at once.
		For every colour there has to be 1 coordinate and visa versa.\n
		:param coordinates: A list of coordinates.
		:param colours: A list of RGB colours.
		"""

		assert len(coordinates) == len(colours), "The size of the coordinates has to be the same as the amount of colours!"

		packets = []
		for i, (coordinate, colour) in enumerate(zip(coordinates, colours), start=1):
			try:
				self._validate_coordinate(coordinate)
				self._validate_colour(colour)
			except ValueError as e:
				raise ValueError(f"Error in pixel {i}: {e}")

			packet = struct.pack('>hhBBB', coordinate[0], coordinate[1], colour[0], colour[1], colour[2])
			packets.append(packet)

		# Concatenate all packets into a single binary string
		joined = b''.join(packets)

		# Send the concatenated binary string
		self.client_socket.send(joined)

	def __del__(self):
		self.client_socket.close()
