# CONTRIBUTION

## General overview

InputPlumber manages the mapping between virtual and physical devices:
  - Sources are physical devices
  - Targets are virtual devices
  - Input devices: take human actions and put them into the system (joystick, button, etc)
  - Output devices: take digital actions and make physical responses (light, rumble, sound, etc )

So a SourceOutputDevice is physical device that produces a physical response (like LEDs).

When a new device appears InputPlumber tries to load it in a composite device, this way it can map one virtual device (such as a dualsense)
to one or more physical devices: all buttons and features (like leds and IMUs) of a controller that linux sees as multiple devices are gathered into one virtual controller making all those hardware features available for games!
