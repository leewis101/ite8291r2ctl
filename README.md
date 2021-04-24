# What is it?

A utility controlling ITE Device(8291) Rev 0.02 RGB keyboard backlight in Linux.

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

# Dependencies

This utility depends on hidapi. Install it first:

```
sudo apt install libhidapi-dev
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
USAGE:
    ite8291r2_ctl [FLAGS] [OPTIONS] --effect <effect>

FLAGS:
    -h, --help       Prints help information
    -S, --save       Save the settings
    -V, --version    Prints version information

OPTIONS:
    -b, --brightness <brightness>    Brightness of the effect: 0-4 [default: 2]
    -c, --color <color>              Color of the effect
    -d, --direction <direction>      Direction of the effect: left, right
    -e, --effect <effect>            Keyboard backlight effect: monocolor, breathing, wave, rainbow, flash, mix, disable
    -s, --speed <speed>              Speed of the effect: 0-4

```

e.g. `ite8291r2_ctl --effect rainbow`

# See also

- https://github.com/kirainmoe/tongfang-hackintosh-utility/blob/master/starbeat-client/build/electron/hidutils.js 
- https://github.com/ederfmartins/rgb_keyboard
- https://github.com/pobrn/ite8291r3-ctl