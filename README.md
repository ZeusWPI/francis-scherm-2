# Francis Scherm 2 (electric fronsaloo)

A simple webserver with endpoints to sent individual pixels on KelderScherm

Currently supports the following interactions:
 - POST requests to `http://{ip}:8000/{x}/{y}/{r}/{g}/{b}/{a}`
 - Websocket Text messages to `ws://{ip}:8000/set_pixel` of the form `{x} {y} {r} {g} {b} [{a}]`
 - Websocket Binary messages to `ws://{ip}:8000/set_pixel` of the form `XX XX XX XX YY YY YY YY RR GG BB AA`
 - UDP datagram to `{ip}:8001` of the form `XX XX XX XX YY YY YY YY RR GG BB AA`
