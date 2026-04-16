use std::path::{Path, PathBuf};

use log::debug;

use crate::error::{Error, Result};

// ── Sysfs paths ──────────────────────────────────────────────────────

// ASUS WMI driver (most common)
const ASUS_THERMAL: &str = "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy";
const ASUS_FAN: &str = "/sys/devices/platform/asus-nb-wmi/fan_boost_mode";

// Faustus driver (alternative for unsupported models)
const FSTS_THERMAL: &str = "/sys/devices/platform/faustus/throttle_thermal_policy";
const FSTS_FAN: &str = "/sys/devices/platform/faustus/fan_boost_mode";

// Keyboard backlight
const LED_BRIGHTNESS: &str = "/sys/class/leds/asus::kbd_backlight/brightness";

// Battery charge limit — common locations
const BATTERY_CHARGE_LIMIT_PATHS: &[&str] = &[
    "/sys/class/power_supply/BAT0/charge_control_end_threshold",
    "/sys/class/power_supply/BAT1/charge_control_end_threshold",
    "/sys/class/power_supply/BATT/charge_control_end_threshold",
];

// ── Subsystem enum ───────────────────────────────────────────────────

/// Represents a hardware subsystem that perfmode can control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subsystem {
    Fan,
    Thermal,
    Led,
    Battery,
}

impl std::fmt::Display for Subsystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subsystem::Fan => write!(f, "fan"),
            Subsystem::Thermal => write!(f, "thermal"),
            Subsystem::Led => write!(f, "led"),
            Subsystem::Battery => write!(f, "battery"),
        }
    }
}

// ── Driver detection ─────────────────────────────────────────────────

/// Resolve the sysfs path for a given subsystem.
///
/// For fan and thermal, we probe the ASUS WMI driver first, then fall
/// back to the Faustus driver.  For LED we check the single known path.
/// For battery we probe multiple known locations.
pub fn resolve_path(subsystem: Subsystem, need_write: bool) -> Result<PathBuf> {
    match subsystem {
        Subsystem::Fan => probe_first(&[ASUS_FAN, FSTS_FAN], need_write),
        Subsystem::Thermal => probe_first(&[ASUS_THERMAL, FSTS_THERMAL], need_write),
        Subsystem::Led => probe_single(LED_BRIGHTNESS, need_write),
        Subsystem::Battery => {
            for path_str in BATTERY_CHARGE_LIMIT_PATHS {
                let p = Path::new(path_str);
                if p.exists() {
                    debug!("battery node found at {}", path_str);
                    if need_write && is_readonly(p) {
                        return Err(Error::PermissionDenied {
                            path: p.to_path_buf(),
                        });
                    }
                    return Ok(p.to_path_buf());
                }
            }
            Err(Error::BatteryNotSupported)
        }
    }
}

/// Check whether **all** subsystems are detectable (used by `status`).
/// Returns `Ok(path)` or the detection error for each subsystem.
pub fn detect_all() -> Vec<(Subsystem, std::result::Result<PathBuf, Error>)> {
    [
        Subsystem::Fan,
        Subsystem::Thermal,
        Subsystem::Led,
        Subsystem::Battery,
    ]
    .into_iter()
    .map(|s| {
        let result = resolve_path(s, false); // read-only probe
        (s, result)
    })
    .collect()
}

// ── helpers ──────────────────────────────────────────────────────────

fn probe_first(candidates: &[&str], need_write: bool) -> Result<PathBuf> {
    for path_str in candidates {
        let p = Path::new(path_str);
        if p.exists() {
            debug!("found sysfs node: {}", path_str);
            if need_write && is_readonly(p) {
                return Err(Error::PermissionDenied {
                    path: p.to_path_buf(),
                });
            }
            return Ok(p.to_path_buf());
        }
    }
    Err(Error::NoDriverDetected)
}

fn probe_single(path_str: &str, need_write: bool) -> Result<PathBuf> {
    let p = Path::new(path_str);
    if !p.exists() {
        return Err(Error::NodeNotFound {
            path: p.to_path_buf(),
        });
    }
    if need_write && is_readonly(p) {
        return Err(Error::PermissionDenied {
            path: p.to_path_buf(),
        });
    }
    Ok(p.to_path_buf())
}

fn is_readonly(path: &Path) -> bool {
    path.metadata()
        .map(|m| m.permissions().readonly())
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn resolve_missing_path_returns_error() {
        // None of the sysfs paths exist on a dev machine, so we expect an error.
        let result = resolve_path(Subsystem::Fan, false);
        assert!(result.is_err());
    }

    #[test]
    fn resolve_battery_missing_returns_battery_not_supported() {
        let result = resolve_path(Subsystem::Battery, false);
        assert!(matches!(
            result,
            Err(Error::BatteryNotSupported) | Err(Error::PermissionDenied { .. })
        ));
    }

    #[test]
    fn detect_all_returns_entries_for_every_subsystem() {
        let results = detect_all();
        assert_eq!(results.len(), 4);
        let subsystems: Vec<_> = results.iter().map(|(s, _)| *s).collect();
        assert!(subsystems.contains(&Subsystem::Fan));
        assert!(subsystems.contains(&Subsystem::Thermal));
        assert!(subsystems.contains(&Subsystem::Led));
        assert!(subsystems.contains(&Subsystem::Battery));
    }

    #[test]
    fn is_readonly_on_tempfile() {
        let dir = std::env::temp_dir().join("perfmode_test");
        let _ = fs::create_dir_all(&dir);
        let file_path = dir.join("test_readonly");
        {
            let mut f = fs::File::create(&file_path).unwrap();
            f.write_all(b"0").unwrap();
        }
        // File should be writable by default
        assert!(!is_readonly(&file_path));

        // Make it read-only
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&file_path, perms).unwrap();
        assert!(is_readonly(&file_path));

        // Cleanup
        let mut perms = fs::metadata(&file_path).unwrap().permissions();
        #[allow(clippy::permissions_set_readonly_false)]
        perms.set_readonly(false);
        fs::set_permissions(&file_path, perms).unwrap();
        let _ = fs::remove_file(&file_path);
        let _ = fs::remove_dir(&dir);
    }
}
