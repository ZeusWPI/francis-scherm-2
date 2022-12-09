import struct
import socket
import random

clientSocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
clientSocket.connect(("10.0.0.69", 8000))

x, y = struct.unpack('>HH', clientSocket.recv(4))
print(x, y)

while True:
	packet = []
	for i in range(1000):
		packet.append(struct.pack('>hhBBB', random.randint(0, x), random.randint(0, y), 0, 0, 200))
	joined = b''.join(packet)
	clientSocket.send(joined)
