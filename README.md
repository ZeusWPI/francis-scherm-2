# Francis Scherm 2 (electric fronsaloo)

A simple webserver with endpoints to sent individual pixels on KelderScherm

Currently supports the following interactions:
 - POST requests to `http://{ip}/{x}/{y}/{r}/{g}/{b}`
 - Websocket Text messages to `ws://{ip}/set_pixel` of the form `{x} {y} {r} {g} {b}`
