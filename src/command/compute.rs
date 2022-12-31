use nalgebra::{DMatrix, DVector, RowDVector};
use num_rational::BigRational;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};

use crate::app::log_json;

use super::invoke_args;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ComputeArgs<'a> {
    transport_rate: &'a DMatrix<f64>,
    cost_rate: &'a DMatrix<f64>,
    min_transport_per_line: &'a DVector<f64>,
    ships_count_per_type: &'a RowDVector<u16>,
}

pub async fn compute<'a>(
    transport_rate: &'a DMatrix<f64>,
    cost_rate: &'a DMatrix<f64>,
    min_transport_per_line: &'a DVector<f64>,
    ships_count_per_type: &'a RowDVector<u16>,
) -> Result<(DMatrix<BigRational>, BigRational), String> {
    let args = ComputeArgs {
        transport_rate,
        cost_rate,
        min_transport_per_line,
        ships_count_per_type,
    };
    let response = invoke_args("compute", to_value(&args).unwrap()).await;

    match response {
        Ok(matrix) => {
            log_json(&matrix);
            Ok(from_value(matrix).unwrap())
        }
        Err(err_msg) => {
            log_json(&err_msg);
            Err(from_value(err_msg).unwrap())
        }
    }
}
