use std::fs;
use std::path::Path;

use log::info;

use crate::driver::{self, Subsystem};
use crate::error::{Error, Result};

// ── Read ─────────────────────────────────────────────────────────────

/// Read the current value from a sysfs node and return a human-readable label.
pub fn get_current(subsystem: Subsystem) -> Result<String> {
    let path = driver::resolve_path(subsystem, false)?;
    let raw = read_raw(&path)?;
    let label = decode(subsystem, &raw, &path)?;
    Ok(label.to_string())
}

/// Read the raw trimmed value from a sysfs node.
fn read_raw(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|e| Error::ReadFailed {
            path: path.to_path_buf(),
            source: e,
        })
}

/// Translate a raw sysfs value into a human-readable label.
fn decode<'a>(subsystem: Subsystem, raw: &str, path: &Path) -> Result<&'a str> {
    match subsystem {
        Subsystem::Led => match raw {
            "0" => Ok("off"),
            "1" => Ok("min"),
            "2" => Ok("med"),
            "3" => Ok("max"),
            _ => Err(Error::UnexpectedValue {
                path: path.to_path_buf(),
                value: raw.to_string(),
            }),
        },
        Subsystem::Fan => match raw {
            "0" => Ok("balanced"),
            "1" => Ok("turbo"),
            "2" => Ok("silent"),
            _ => Err(Error::UnexpectedValue {
                path: path.to_path_buf(),
                value: raw.to_string(),
            }),
        },
        Subsystem::Thermal => match raw {
            "0" => Ok("default"),
            "1" => Ok("overboost"),
            "2" => Ok("silent"),
            _ => Err(Error::UnexpectedValue {
                path: path.to_path_buf(),
                value: raw.to_string(),
            }),
        },
        Subsystem::Battery => {
            // Battery returns a numeric percentage — just echo it.
            Ok(Box::leak(format!("{}%", raw).into_boxed_str()))
        }
    }
}

// ── Write ────────────────────────────────────────────────────────────

/// Write a value to a sysfs node.
pub fn set_value(subsystem: Subsystem, value: &str, label: &str) -> Result<()> {
    let path = driver::resolve_path(subsystem, true)?;
    write_raw(&path, value)?;
    info!("set {} to {}", subsystem, label);
    println!("perfmode: set {} to {}", subsystem, label);
    Ok(())
}

fn write_raw(path: &Path, value: &str) -> Result<()> {
    fs::write(path, value).map_err(|e| Error::WriteFailed {
        path: path.to_path_buf(),
        source: e,
    })
}

// ── Fan helpers ──────────────────────────────────────────────────────

pub fn fan_silent() -> Result<()> {
    set_value(Subsystem::Fan, "2", "silent")
}

pub fn fan_balanced() -> Result<()> {
    set_value(Subsystem::Fan, "0", "balanced")
}

pub fn fan_turbo() -> Result<()> {
    set_value(Subsystem::Fan, "1", "turbo")
}

pub fn fan_get() -> Result<()> {
    let val = get_current(Subsystem::Fan)?;
    println!("{}", val);
    Ok(())
}

// ── Thermal helpers ──────────────────────────────────────────────────

pub fn thermal_silent() -> Result<()> {
    set_value(Subsystem::Thermal, "2", "silent")
}

pub fn thermal_default() -> Result<()> {
    set_value(Subsystem::Thermal, "0", "default")
}

pub fn thermal_overboost() -> Result<()> {
    set_value(Subsystem::Thermal, "1", "overboost")
}

pub fn thermal_get() -> Result<()> {
    let val = get_current(Subsystem::Thermal)?;
    println!("{}", val);
    Ok(())
}

// ── LED helpers ──────────────────────────────────────────────────────

pub fn led_off() -> Result<()> {
    set_value(Subsystem::Led, "0", "off")
}

pub fn led_min() -> Result<()> {
    set_value(Subsystem::Led, "1", "min")
}

pub fn led_med() -> Result<()> {
    set_value(Subsystem::Led, "2", "med")
}

pub fn led_max() -> Result<()> {
    set_value(Subsystem::Led, "3", "max")
}

pub fn led_get() -> Result<()> {
    let val = get_current(Subsystem::Led)?;
    println!("{}", val);
    Ok(())
}

// ── Battery helpers ──────────────────────────────────────────────────

pub fn battery_set(limit: u8) -> Result<()> {
    set_value(
        Subsystem::Battery,
        &limit.to_string(),
        &format!("{}%", limit),
    )
}

pub fn battery_get() -> Result<()> {
    let val = get_current(Subsystem::Battery)?;
    println!("{}", val);
    Ok(())
}

// ── Status ───────────────────────────────────────────────────────────

pub fn print_status() {
    println!("perfmode: current status");
    println!("{:-<35}", "");

    for (subsystem, result) in driver::detect_all() {
        let label = match result {
            Ok(_path) => match get_current(subsystem) {
                Ok(val) => val,
                Err(e) => format!("error: {}", e),
            },
            Err(e) => format!("n/a ({})", e),
        };
        println!("  {:<10} {}", format!("{}:", subsystem), label);
    }
}

// ── Profiles ─────────────────────────────────────────────────────────

pub fn apply_profile_silent() -> Result<()> {
    println!("perfmode: applying 'silent' profile");
    let mut errors = Vec::new();

    if let Err(e) = fan_silent() {
        errors.push(format!("  fan: {}", e));
    }
    if let Err(e) = thermal_silent() {
        errors.push(format!("  thermal: {}", e));
    }
    if let Err(e) = led_off() {
        errors.push(format!("  led: {}", e));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        eprintln!("perfmode: some subsystems failed:");
        for e in &errors {
            eprintln!("{}", e);
        }
        // Return the first error for the exit code
        Err(Error::NoDriverDetected)
    }
}

pub fn apply_profile_balanced() -> Result<()> {
    println!("perfmode: applying 'balanced' profile");
    let mut errors = Vec::new();

    if let Err(e) = fan_balanced() {
        errors.push(format!("  fan: {}", e));
    }
    if let Err(e) = thermal_default() {
        errors.push(format!("  thermal: {}", e));
    }
    if let Err(e) = led_min() {
        errors.push(format!("  led: {}", e));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        eprintln!("perfmode: some subsystems failed:");
        for e in &errors {
            eprintln!("{}", e);
        }
        Err(Error::NoDriverDetected)
    }
}

pub fn apply_profile_gaming() -> Result<()> {
    println!("perfmode: applying 'gaming' profile");
    let mut errors = Vec::new();

    if let Err(e) = fan_turbo() {
        errors.push(format!("  fan: {}", e));
    }
    if let Err(e) = thermal_overboost() {
        errors.push(format!("  thermal: {}", e));
    }
    if let Err(e) = led_max() {
        errors.push(format!("  led: {}", e));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        eprintln!("perfmode: some subsystems failed:");
        for e in &errors {
            eprintln!("{}", e);
        }
        Err(Error::NoDriverDetected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_fan_values() {
        let p = Path::new("/tmp/fake");
        assert_eq!(decode(Subsystem::Fan, "0", p).unwrap(), "balanced");
        assert_eq!(decode(Subsystem::Fan, "1", p).unwrap(), "turbo");
        assert_eq!(decode(Subsystem::Fan, "2", p).unwrap(), "silent");
        assert!(decode(Subsystem::Fan, "9", p).is_err());
    }

    #[test]
    fn decode_thermal_values() {
        let p = Path::new("/tmp/fake");
        assert_eq!(decode(Subsystem::Thermal, "0", p).unwrap(), "default");
        assert_eq!(decode(Subsystem::Thermal, "1", p).unwrap(), "overboost");
        assert_eq!(decode(Subsystem::Thermal, "2", p).unwrap(), "silent");
        assert!(decode(Subsystem::Thermal, "5", p).is_err());
    }

    #[test]
    fn decode_led_values() {
        let p = Path::new("/tmp/fake");
        assert_eq!(decode(Subsystem::Led, "0", p).unwrap(), "off");
        assert_eq!(decode(Subsystem::Led, "1", p).unwrap(), "min");
        assert_eq!(decode(Subsystem::Led, "2", p).unwrap(), "med");
        assert_eq!(decode(Subsystem::Led, "3", p).unwrap(), "max");
        assert!(decode(Subsystem::Led, "7", p).is_err());
    }
}
