Steam controller handling
=========================
Rust library to work with the Steam controller.

Request
=======
All requests are done with a 64 bytes HID feature send, the 1st byte
contains the ID, the 2nd byte the size of the packet, then the payload
and the rest is padded with `0x00`.

Some requests have responses, some do not, responses have the same layout as
requests. In the listed responses only the payload is shown.

Check connection
----------------
When using the dongle the controller status can be checked.

- `0xb4` id
- `0x00` size

### Response

- `0x01` for disconnected, `0x02` for connected

Build details
-------------
Information about the device can be fetched, it's a list of key/value pairs
where the key is 1 byte, and the value is 4 bytes.

- `0x83` id
- `0x00` size

### Keys

- `0x0a` is the bootloader build date.
- `0x04` is the controller firmware build date.
- `0x05` is the radio firmware build date.

Serial number
-------------
Serial numbers can be requested.

- `0xae` id
- `0x15` size
- `0x00` for main board, `0x01` for controller

### Response

- `0x00`
- `[u8; 10]` the serial number

Receiver information
--------------------
If the controller is in wireless mode information about the receiver can be
fetched.

- `0xa1` id
- `0x00` size

### Response

- `i32` firmware build date.
- 10 bytes of ヽ(´ｰ｀ )ﾉ
- `[u8; 10]` receiver serial number

Control
=======
All controls are done with a 64 bytes HID feature send, the 1st byte
contains the ID, the 2nd byte the size of the packet, then the payload
and the rest is padded with `0x00`.

Disable lizard mode
-------------------
The so called lizard mode can be disabled.

- `0x81` id
- `0x00` size

Enable lizard mode
------------------
The so called lizard mode can be enabled.

- `0x85` id
- `0x00` size

Feedback
--------
Feedbacks can be sent to either pad.

- `0x8f` id
- `0x08` size
- `0x00` for right, `0x01` for left
- `u16` for amplitude
- `u16` for period
- `u16` for count

Sensors and timeout
-------------------
The gyroscope and accellerometer can be enabled or disabled and the idle
timeout can be changed.

- `0x87` id
- `0x15` size
- `0x32`
- `u16` timeout
- `0x18 0x00 0x00 0x31 0x02 0x00 0x08 0x07 0x00 0x07 0x07 0x00 0x30`
- `0x00` to disable, `0x14` to disable
- `0x00 0x2e`

Led intensity
-------------
The led intensity can be controlled.

- `0x87` id
- `0x03` size
- `0x2d`
- `u8` between `0 .. 100`

Notification sound test
-----------------------
Each notification sound can be tested.

- `0xb6` id
- `0x04` size
- `u8` the sound ID

Notification sound change
-------------------------
The notification sound for turn on and turn off can be changed.

- `0xc1` id
- `0x10` size
- `u8` the turn on sound ID
- `u8` the turn off sound ID
- `0xff 0xff 0x03 0x09`
- `0x05 0xff 0xff 0xff`
- `0xff 0xff 0xff 0xff`
- `0xff 0xff`

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
|            0x0100             |      Type     |      Size     |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Type

It's a 1 byte flag.

- `0x01` means the device is sending input.
- `0x03` means the device is sending power events.
- `0x04` means the device is idle.

### Size

Tells the size of the packet.

Power
-----
```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Event     |                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Event

It's an unsigned char.

- `0x01` means power off.
- `0x02` means power on.

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
|           Something?          |      0x64     |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Sequence

It's a little endian unsigned int, it's increased by one with every read.

### Something?

Seems to get reset every time it goes from input to idle and increments in some
kind of exponential curve.

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
|       Orientation Roll        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
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
- `0b000000000000000001000000` the analog stick has been pressed.
- `0b000000000000000010000000` the analog stick is being touched.
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
- `0b000000001000000000000000` the left grip has been pressed.
- `0b000000000000000000000001` the right grip has been pressed.
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

### Something?

There's something, I don't know what it is tho.
