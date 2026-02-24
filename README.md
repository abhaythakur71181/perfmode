# Perfmode

Perfmode is a performance control utility for ASUS TUF Gaming series of laptops.
It allows both fan and thermal policy control along with backlight control of the laptop.

#### Fan Control Modes

- `turbo` | `t`
- `balanced` | `b`
- `silent` | `s`

#### Thermal Policy
- `overboost` | `ob`
- `default` | `df`
- `silent` | `s`

Keyboard backlight :

- `off`
- `min`
- `med`
- `max`

> You can use either of Fan or Thermal policy options and it will take desired effect.
Read program help for more information.

## Usage

### Command Line mode

Generic usage:

```bash
$ sudo perfmode -option arg
```

- Viewing Help

```bash
$ perfmode --help
```

> Please do note that this program relies on the files present in 
 `/sys/devices/platform/` and assumes if they exist, then the kernel driver
 is also loaded. Earlier the program relied on lsmod output but that was not
 feasible where the module(s) were built into the kernel.

## Dependencies

- cargo
- git (optional)

## Installation

### Regular Linux Distributions

```bash
git clone https://github.com/falcon71181/perfmode.git && cd perfmode
cargo build --release # optional - installs to /usr/bin
```

Or

```bash
cargo install perfmode
```

Clone. Make. Install. Simple as that!

## Acknowledgments

This project is a Rust implementation of the original [perfmode](https://github.com/icebarf/perfmode) utility created by [icebarf](https://github.com/icebarf). Special thanks to icebarf for the original work and logic.

