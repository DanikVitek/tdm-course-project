use nalgebra::{DMatrix, DVector, RowDVector};
use num_rational::BigRational;
use num_traits::Zero;
use ratio_extension::BigRationalExt;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};

use crate::app::{log, log_json};

use super::invoke_args;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ComputeArgs<'a> {
    transport_rate: &'a DMatrix<BigRationalExt>,
    cost_rate: &'a DMatrix<BigRationalExt>,
    min_transport_per_line: &'a DVector<BigRationalExt>,
    ships_count_per_type: &'a RowDVector<u16>,
}

pub async fn compute<'a>(
    available_ship_line: &'a DMatrix<bool>,
    transport_rate: &'a DMatrix<f64>,
    cost_rate: &'a DMatrix<f64>,
    min_transport_per_line: &'a DVector<f64>,
    ships_count_per_type: &'a RowDVector<u16>,
) -> Result<(DMatrix<BigRational>, BigRational), String> {
    let transport_rate = transport_rate.zip_map(available_ship_line, |a_ij, available| {
        if !available {
            BigRationalExt::zero()
        } else {
            BigRationalExt::from_float(a_ij)
        }
    });
    let cost_rate = cost_rate.zip_map(available_ship_line, |c_ij, available| {
        if !available {
            BigRationalExt::Inf
        } else {
            BigRationalExt::from_float(c_ij)
        }
    });
    let min_transport_per_line = min_transport_per_line.map(BigRationalExt::from_float);
    log_json(&to_value(&transport_rate).unwrap());
    log_json(&to_value(&cost_rate).unwrap());
    let args = ComputeArgs {
        transport_rate: &transport_rate,
        cost_rate: &cost_rate,
        min_transport_per_line: &min_transport_per_line,
        ships_count_per_type,
    };
    let response = invoke_args("compute", to_value(&args).unwrap()).await;

    match response {
        Ok(solution) => {
            log("Received response");
            log_json(&solution);
            Ok(from_value(solution).unwrap())
        }
        Err(err_msg) => {
            log("Error occurred");
            log_json(&err_msg);
            Err(from_value(err_msg).unwrap())
        }
    }
}
