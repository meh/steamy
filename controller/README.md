Steam controller handling
=========================
Rust library to work with the Steam controller.

Protocol
========
There's a script in `support/` to help out with reversing the protocol, the
following is what I know so far.

Header
------
```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                               |            Status             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Status

It's a 16 bit flag, the values below are in big endian.

- `0x0301` means the device is sending power events.
- `0x040b` means the device is idle.
- `0x013c` means the device is sending input.

Power
-----
```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Event     |    Garbage?   |                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Event

It's an unsigned char.

- `0x01` means power off.
- `0x02` means power on.

### Garbage?

Seems to be a monotonically increasing value after the controller has been
turned on.

Can't find a pattern in it so far.

Idle
----
```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                          Sequence                             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                          Garbage?                             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Sequence

It's a little endian unsigned int, it's increased by one with every read.

Input
-----

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           Sequence                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                    Buttons                    |  Left Trigger |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Right Trigger |                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|          Left Pad X           |           Left Pad Y          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|         Right Pad X           |          Right Pad Y          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                               |                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                               |                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                               |       Acceleration Pitch      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       Acceleration Yaw        |       Acceleration Roll       |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       Orientation Pitch       |        Orientation Yaw        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       Orientation Roll        |                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Sequence

It's a little endian unsigned int, it's increased by one with every read.

### Buttons

It's a 24 bit set, the values below are in big endian.

- `0b100000000000000000000000` the A button has been pressed.
- `0b001000000000000000000000` the B button has been pressed.
- `0b010000000000000000000000` the X button has been pressed.
- `0b000100000000000000000000` the Y button has been pressed.
- `0b000000000000000000000010` the pad is being pressed.
- `0b000000000000000000001000` the pad is being touched.
- `0b000000000000000001000000` the analog has been pressed.
- `0b000000000000100000000000` the pad down side has been pressed.
- `0b000000000000010000000000` the pad left side has been pressed.
- `0b000000000000001000000000` the pad right side has been pressed.
- `0b000000000000000100000000` the pad up side has been pressed.
- `0b000000000000000000000100` the trackpad has been pressed.
- `0b000000000000000000010000` the trackpad has been touched.
- `0b000000000001000000000000` the back button has been pressed.
- `0b000000000010000000000000` the home button has been pressed.
- `0b000000000100000000000000` the forward button has been pressed.
- `0b000010000000000000000000` the left bumper has been pressed.
- `0b000001000000000000000000` the right bumper has been pressed.
- `0b00000001000000000000000` the left grip has been pressed.
- `0b00000000000000000000001` the right grip has been pressed.
- `0b000000100000000000000000` the left trigger has been fully pressed.
- `0b000000010000000000000000` the right trigger has been fully pressed.

### Left trigger

It's an unsigned byte for the pressure applied to the trigger.

### Right trigger

It's an unsigned byte for the pressure applied to the trigger.

### Left Pad X

It's a little endian signed short.

### Left Pad Y

It's a little endian signed short.

### Right Pad X

It's a little endian signed short.

### Right Pad Y

It's a little endian signed short.

### Orientation Roll

It's a little endian signed short.

### Orientation Yawl

It's a little endian signed short.

### Orientation Pitch

It's a little endian signed short.

### Acceleration Roll

It's a little endian signed short.

### Acceleration Yaw

It's a little endian signed short.

### Acceleration Pitch

It's a little endian signed short.
