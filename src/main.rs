mod cli;
mod driver;
mod error;
mod operations;

use clap::Parser;
use cli::{BatteryAction, Cli, Command, FanAction, LedAction, ProfileName, ThermalAction};
use std::process::exit;

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    let result = match cli.command {
        // ── Fan ──────────────────────────────────────────────────
        Command::Fan { action } => match action {
            FanAction::Silent => operations::fan_silent(),
            FanAction::Balanced => operations::fan_balanced(),
            FanAction::Turbo => operations::fan_turbo(),
            FanAction::Get => operations::fan_get(),
        },

        // ── Thermal ──────────────────────────────────────────────
        Command::Thermal { action } => match action {
            ThermalAction::Silent => operations::thermal_silent(),
            ThermalAction::Default => operations::thermal_default(),
            ThermalAction::Overboost => operations::thermal_overboost(),
            ThermalAction::Get => operations::thermal_get(),
        },

        // ── LED ──────────────────────────────────────────────────
        Command::Led { action } => match action {
            LedAction::Off => operations::led_off(),
            LedAction::Min => operations::led_min(),
            LedAction::Med => operations::led_med(),
            LedAction::Max => operations::led_max(),
            LedAction::Get => operations::led_get(),
        },

        // ── Battery ──────────────────────────────────────────────
        Command::Battery { action } => match action {
            BatteryAction::Set { limit } => operations::battery_set(limit),
            BatteryAction::Get => operations::battery_get(),
        },

        // ── Status ───────────────────────────────────────────────
        Command::Status => {
            operations::print_status();
            Ok(())
        }

        // ── Profiles ─────────────────────────────────────────────
        Command::Profile { name } => match name {
            ProfileName::Silent => operations::apply_profile_silent(),
            ProfileName::Balanced => operations::apply_profile_balanced(),
            ProfileName::Gaming => operations::apply_profile_gaming(),
        },
    };

    if let Err(e) = result {
        eprintln!("perfmode: {}", e);
        exit(1);
    }
}
