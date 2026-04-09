# Perfmode

Perfmode is a performance control utility for ASUS TUF Gaming series of laptops.
It allows fan and thermal policy control, keyboard backlight control, battery
charge limit management, and predefined profiles — all from the command line.

> Please note that this program relies on sysfs nodes present in
> `/sys/devices/platform/` and assumes if they exist, then the kernel driver
> is also loaded. Both the `asus-nb-wmi` and `faustus` drivers are supported.

## Features

- **Fan Control** — Switch between silent, balanced, and turbo modes.
- **Thermal Policy** — Switch between silent, default, and overboost policies.
- **Keyboard Backlight** — Set brightness to off, min, med, or max.
- **Battery Charge Limit** — Cap battery charging at a specific percentage (e.g. 60%, 80%).
- **Status Overview** — View the current state of all subsystems at once.
- **Profiles** — Apply predefined profiles (`silent`, `balanced`, `gaming`) with a single command.
- **Logging** — Enable debug output with `RUST_LOG=debug` for diagnostics.

## Usage

Most write operations require root privileges (`sudo`).

```bash
perfmode <COMMAND> [SUBCOMMAND]
```

### Fan Control

```bash
sudo perfmode fan silent      # Silent mode
sudo perfmode fan balanced    # Balanced mode
sudo perfmode fan turbo       # Turbo mode
perfmode fan get              # Show current fan mode
```

### Thermal Policy

```bash
sudo perfmode thermal silent     # Silent policy
sudo perfmode thermal default    # Default (balanced) policy
sudo perfmode thermal overboost  # Overboost (performance) policy
perfmode thermal get             # Show current thermal policy
```

### Keyboard Backlight

```bash
sudo perfmode led off    # Turn off backlight
sudo perfmode led min    # Minimum brightness
sudo perfmode led med    # Medium brightness
sudo perfmode led max    # Maximum brightness
perfmode led get         # Show current backlight level
```

### Battery Charge Limit

```bash
sudo perfmode battery set 80    # Limit charging to 80%
sudo perfmode battery set 60    # Limit charging to 60%
sudo perfmode battery set 100   # Remove limit (charge to full)
perfmode battery get            # Show current charge limit
```

### Status

Show the current state of all detected subsystems:

```bash
perfmode status
```

Example output:

```
perfmode: current status
-----------------------------------
  fan:       balanced
  thermal:   default
  led:       min
  battery:   80%
```

### Profiles

Apply a predefined profile that sets fan, thermal, and LED simultaneously:

```bash
sudo perfmode profile silent     # Silent fan + silent thermal + LED off
sudo perfmode profile balanced   # Balanced fan + default thermal + LED min
sudo perfmode profile gaming     # Turbo fan + overboost thermal + LED max
```

### Help

```bash
perfmode --help           # General help
perfmode fan --help       # Help for fan subcommand
perfmode battery --help   # Help for battery subcommand
```

## Dependencies

- cargo
- git (optional)

## Installation

### Regular Linux Distributions

```bash
git clone https://github.com/falcon71181/perfmode.git && cd perfmode
cargo build --release
sudo cp target/release/perfmode /usr/local/bin/
```

Or install directly from crates.io:

```bash
cargo install perfmode
```

### NixOS

A `flake.nix` is provided for Nix users:

```bash
nix build
```

## Debugging

Enable verbose logging by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug sudo perfmode fan turbo
```

This will print which sysfs nodes are being probed and selected.

## Acknowledgments

This project is a Rust implementation of the original [perfmode](https://github.com/icebarf/perfmode) utility created by [icebarf](https://github.com/icebarf). Special thanks to icebarf for the original work and logic.
