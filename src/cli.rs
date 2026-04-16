use clap::{Parser, Subcommand, ValueEnum};

/// Perfmode — Manage performance mode of your ASUS laptop.
///
/// Controls fan speed, thermal policy, keyboard backlight, and battery
/// charge limit via sysfs on ASUS TUF / ROG / Zenbook laptops.
#[derive(Debug, Parser)]
#[command(name = "perfmode", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Control the fan boost mode
    Fan {
        #[command(subcommand)]
        action: FanAction,
    },

    /// Control the thermal throttle policy
    Thermal {
        #[command(subcommand)]
        action: ThermalAction,
    },

    /// Control the keyboard backlight brightness
    Led {
        #[command(subcommand)]
        action: LedAction,
    },

    /// Control battery charge limit (0-100)
    Battery {
        #[command(subcommand)]
        action: BatteryAction,
    },

    /// Show current status of all subsystems
    Status,

    /// Apply a predefined profile
    Profile {
        /// The profile to apply
        #[arg(value_enum)]
        name: ProfileName,
    },
}

// ── Fan ──────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum FanAction {
    /// Set fan to silent mode
    Silent,
    /// Set fan to balanced mode
    Balanced,
    /// Set fan to turbo mode
    Turbo,
    /// Get the current fan mode
    Get,
}

// ── Thermal ──────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum ThermalAction {
    /// Set thermal policy to silent
    Silent,
    /// Set thermal policy to default (balanced)
    Default,
    /// Set thermal policy to overboost (performance)
    Overboost,
    /// Get the current thermal policy
    Get,
}

// ── LED ──────────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum LedAction {
    /// Turn off keyboard backlight
    Off,
    /// Set keyboard backlight to minimum
    Min,
    /// Set keyboard backlight to medium
    Med,
    /// Set keyboard backlight to maximum
    Max,
    /// Get the current keyboard backlight level
    Get,
}

// ── Battery ──────────────────────────────────────────────────────────

#[derive(Debug, Subcommand)]
pub enum BatteryAction {
    /// Set the battery charge limit (e.g. 60, 80, 100)
    Set {
        /// Charge limit percentage (1-100)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=100))]
        limit: u8,
    },
    /// Get the current battery charge limit
    Get,
}

// ── Profiles ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, ValueEnum)]
pub enum ProfileName {
    /// Silent fan + silent thermal + LED off
    Silent,
    /// Balanced fan + default thermal + LED min
    Balanced,
    /// Turbo fan + overboost thermal + LED max
    Gaming,
}
