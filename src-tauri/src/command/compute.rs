use std::borrow::Cow;

use nalgebra::{Const, DMatrix, DVector, Dynamic, RowDVector};
use num_rational::BigRational;

use crate::{ensure_eq, simplex};

#[tauri::command]
pub fn compute(
    transport_rate: DMatrix<f64>,
    cost_rate: DMatrix<f64>,
    min_transport_per_line: DVector<f64>,
    ships_count_per_type: RowDVector<u16>,
) -> Result<(DMatrix<BigRational>, BigRational), Cow<'static, str>> {
    log::info!(
        "Received input:\n\
        transport_rate:\n{transport_rate}\n\
        cost_rate:\n{cost_rate}\n\
        min_transport_per_line:\n{min_transport_per_line}\n\
        ships_count_per_type:\n{ships_count_per_type}"
    );

    ensure_eq!(transport_rate.shape(), cost_rate.shape());
    ensure_eq!(ships_count_per_type.ncols(), transport_rate.ncols());
    ensure_eq!(min_transport_per_line.nrows(), transport_rate.nrows());
    log::info!("Shape assertions passed");

    let n_lines = min_transport_per_line.nrows();
    let n_ships = ships_count_per_type.ncols();
    log::info!("n_lines: {n_lines}\nn_ships: {n_ships}");

    let problem = simplex::Problem::new(
        simplex::ObjectiveFunction::new(
            cost_rate
                .transpose()
                .reshape_generic(Const::<1>, Dynamic::new(n_lines * n_ships)),
            true,
        ),
        construct_constraints(
            transport_rate,
            min_transport_per_line,
            ships_count_per_type,
            n_ships,
            n_lines,
        ),
    );
    log::info!("Problem formed: {problem}");

    let solution = problem.solve();
    log::info!("Solution: {}", solution.as_str());

    match solution {
        simplex::Solution::Finite {
            variables,
            function_value,
        } => Ok((
            DMatrix::from_row_iterator(n_lines, n_ships, variables.into_iter().map(|f| f.to_owned())),
            function_value,
        )),
        non_compliant => Err(non_compliant.as_str()),
    }
}

fn construct_constraints(
    transport_rate: DMatrix<f64>,
    min_transport_per_line: DVector<f64>,
    ships_count_per_type: RowDVector<u16>,
    n_ships: usize,
    n_lines: usize,
) -> Vec<simplex::Constraint> {
    transport_rate
        .row_iter()
        .enumerate()
        .map(|(i, row)| {
            let lines_constraint = row.insert_columns(0, dbg!(n_ships * i), 0.).insert_columns(
                dbg!(n_ships * (i + 1)),
                dbg!(n_ships * (n_lines - i - 1)),
                0.,
            );
            log::info!("{lines_constraint}");
            lines_constraint
        })
        .zip(min_transport_per_line.row_iter())
        .map(|(coefficients, rhs)| {
            simplex::Constraint::new(coefficients, simplex::Sign::Greater, rhs.x)
        })
        .chain({
            let mut coefficients = Vec::with_capacity(n_ships);
            coefficients.push(1.);
            coefficients.resize(n_ships, 0.);
            let block = coefficients.clone();
            for _ in 0..n_lines - 1 {
                coefficients.extend_from_slice(&block);
            }
            drop(block);
            ships_count_per_type.column_iter().map(move |count| {
                let constraint = simplex::Constraint::new(
                    RowDVector::from_row_slice(&coefficients),
                    simplex::Sign::Equals,
                    count.x as f64,
                );
                coefficients.rotate_right(1);
                constraint
            })
        })
        .collect()
}
