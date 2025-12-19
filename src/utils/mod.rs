// Utility functions
use bevy::prelude::*;

pub fn log_error(In(result): In<Result<(), String>>) {
    if let Err(e) = result {
        error!("System error: {e}");
    }
}
