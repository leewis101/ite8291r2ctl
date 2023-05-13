# What is it?

A utility controlling ITE Device(8291) Rev 0.02 RGB keyboard backlight in Linux. The keyboard control code is ported from [tongfang-utility](https://github.com/kirainmoe/tongfang-utility).

# Compatibility

This utility only works for rev 0.02. Please confirm your device.

`sudo hwinfo --keyboard`

```
...
Hardware Class: keyboard
Model: "Integrated Technology Express ITE Device(8291)"
Hotplug: USB
Vendor: usb 0x048d "Integrated Technology Express, Inc."
Device: usb 0xce00 "ITE Device(8291)"
Revision: "0.02" <-----------------------
...
```

# Running it without root privileges

By default you need *root* privileges if you want to use this utility. you can create a `udev` rule to allow everyone on your system to access this particular USB device, and thus you won't need to run the program as `root`.

If you want to do that, create a file `/etc/udev/rules.d/99-ite8291.rules`:
```
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", ATTRS{idVendor}=="048d", ATTRS{idProduct}=="ce00", TAG+="uaccess"

```
after creating the file, reboot.

# Usage

```
Usage: ite8291r2ctl [OPTIONS] --effect <EFFECT>

Options:
  -e, --effect <EFFECT>          keyboard backlight effect [possible values: mono, breath, wave, rainbow, flash, mix, disable]
  -S, --save                     save settings
  -b, --brightness <BRIGHTNESS>  keyboard backlight brightness [default: 3] [possible values: 0, 1, 2, 3, 4]
  -c, --color <COLOR>            keybarod backlight color hex code, e.g. #ff0000
  -d, --direction <DIRECTION>    keybarod backlight moving direction [possible values: left, right]
  -s, --speed <SPEED>            keybarod backlight moving speed [possible values: 1, 2, 3, 4, 5]
  -h, --help                     Print help
  -V, --version                  Print version

```

e.g. `ite8291r2ctl --effect rainbow`

# See also

- https://github.com/kirainmoe/tongfang-hackintosh-utility/blob/master/starbeat-client/build/electron/hidutils.js 
- https://github.com/ederfmartins/rgb_keyboard
- https://github.com/pobrn/ite8291r3-ctl