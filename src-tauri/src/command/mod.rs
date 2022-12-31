mod compute;

pub use compute::*;

use nalgebra::DMatrix;
use num_rational::Rational64;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet() -> DMatrix<Rational64> {
    DMatrix::from_element(5, 7, Rational64::new(26, 4))
}