use lazy_static::lazy_static;
use nalgebra::{DMatrix, DVector, DVectorSlice, RowDVector};
use num_traits::Zero;
use ratio_extension::BigRationalExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::dbg_display;

use super::{big_number::BigNumber, ObjectiveFunction, Problem, Solution};
use super::{SolutionError, SolutionResult};

#[derive(Debug, Clone, PartialEq)]
pub struct SimplexTable {
    n_significant_variables: usize,
    /// Indices of basis vectors
    basis: DVector<usize>,
    /// i_max x j_max table of coefficients from constraints
    tableau: DMatrix<BigRationalExt>,
    rhs: DVector<BigRationalExt>,
    coefficients: RowDVector<BigNumber<BigRationalExt>>,
    minimization: bool,
}

lazy_static! {
    static ref ZERO: BigRationalExt = BigRationalExt::zero();
}

impl SimplexTable {
    pub fn new(problem: Problem) -> Self {
        let Problem {
            objective_function:
                ObjectiveFunction {
                    n_significant_variables,
                    coefficients,
                    minimization,
                },
            constraints,
            rhs,
        } = problem;
        Self {
            n_significant_variables,
            basis: DVector::from_vec(
                coefficients
                    .column_iter()
                    .enumerate()
                    .filter_map(|(i, el)| {
                        (el.x == BigNumber::<BigRationalExt>::one_big()).then_some(i)
                    })
                    .collect::<Vec<_>>(),
            ),
            tableau: constraints,
            coefficients: coefficients.map(BigNumber::<BigRationalExt>::from),
            rhs,
            minimization,
        }
    }

    pub fn basis(&self) -> DVectorSlice<usize> {
        (&self.basis).into()
    }

    pub fn basis_coefficients(&self) -> DVector</* f64 */ BigNumber<BigRationalExt>> {
        self.basis
            .iter()
            .map(|i| unsafe { self.coefficients.get_unchecked(*i) }.to_owned())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn function_estimation(&self) -> BigNumber<BigRationalExt> {
        std::mem::take(
            &mut (dbg_display!(self.basis_coefficients().transpose())
                * dbg_display!(self.rhs.map(BigNumber::from)))[0],
        )
    }

    pub fn column_estimation(&self, index: usize) -> Option<BigNumber<BigRationalExt>> {
        if index > self.tableau.ncols() {
            return None;
        }
        // Safety: now we know, that the index is in bounds
        Some(unsafe { self.column_estimation_unchecked(index) })
    }

    /// # Safety
    /// Panics if index is out of bounds
    unsafe fn column_estimation_unchecked(&self, index: usize) -> BigNumber<BigRationalExt> {
        std::mem::take(
            &mut (self.basis_coefficients().transpose()
                * self
                    .tableau
                    .column(index)
                    .map(BigNumber::<BigRationalExt>::from))[0],
        ) - {
            let column_coef = self.coefficients[index].to_owned();
            log::debug!("Coefficient of column {index}: {column_coef}");
            column_coef
        }
    }

    pub fn step(
        &mut self,
        prev_pivot_column: Option<usize>,
    ) -> (Option<SolutionResult>, Option<usize>) {
        log::debug!("Tableau:{}", self.tableau);
        if self.minimization {
            log::info!("Minimization step");
            self.step_min(prev_pivot_column)
        } else {
            log::info!("Maximization step");
            self.step_max(prev_pivot_column)
        }
    }

    fn step_min(
        &mut self,
        prev_pivot_column: Option<usize>,
    ) -> (Option<SolutionResult>, Option<usize>) {
        let pivot_col: Option<usize> = (0..self.tableau.ncols())
            .filter_map(|i| {
                let estimation = unsafe { self.column_estimation_unchecked(i) };
                log::debug!("{estimation}");
                (estimation > Zero::zero()).then_some((i, estimation))
            })
            .max_by(|(_, es1), (_, es2)| es1.total_cmp(es2))
            .map(|(i, _)| i);

        log::info!("Pivot column: {pivot_col:?}");
        if pivot_col.is_some() && prev_pivot_column == pivot_col {
            return (Some(Err(SolutionError::Absent)), pivot_col);
        }

        match pivot_col {
            Some(pivot_col) => {
                log::info!("Optimal solution was not found");

                let pivot_row = self
                    .tableau
                    .column(pivot_col)
                    .row_iter()
                    .zip(&self.rhs)
                    .enumerate()
                    .filter(|(_, (pivot_col_el, _))| &pivot_col_el.x > &ZERO)
                    .map(|(i, (pivot_col_el, rhs_el))| (i, rhs_el / &pivot_col_el.x))
                    .min_by(|(_, ratio1), (_, ratio2)| ratio1.partial_cmp(ratio2).unwrap())
                    .map(|(i, _)| i)
                    .unwrap();
                log::info!("Pivot row: {pivot_row}");

                let pivot_el = self.tableau[(pivot_row, pivot_col)].to_owned();
                log::info!("Pivot element: {pivot_el}");

                // divide all elements in a row by pivot element
                self.rhs[pivot_row] /= &pivot_el;
                self.tableau.row_mut(pivot_row).apply(|el| *el /= &pivot_el);

                // subtract pivot row from other rows till all of elements in pivot coll except of pivot element are zero
                for i in (0..self.tableau.nrows()).filter(|i| i != &pivot_row) {
                    let multiplier = self.tableau[(i, pivot_col)].to_owned();
                    self.rhs[i] = &self.rhs[i] - &(&self.rhs[pivot_row] * &multiplier);

                    let pivot_row = self.tableau.row(pivot_row).into_owned(); // maybe optimize
                    self.tableau.row_mut(i).zip_apply(
                        &pivot_row,
                        |tableau_row_el, pivot_row_el| {
                            *tableau_row_el -= &pivot_row_el * &multiplier
                        },
                    );
                }

                self.basis[pivot_row] = pivot_col;

                (None, Some(pivot_col))
            }
            None => (
                Some(
                    /* if self
                        .basis
                        .iter()
                        .any(|i| &self.coefficients[*i] == &self.big_coefficient)
                    {
                        log::info!("There is no solution");
                        Solution::Absent
                    } else */
                    'b: {
                        log::info!("Optimal solution was found");
                        Ok(Solution {
                            vars: (0..self.n_significant_variables)
                                .into_par_iter()
                                .map(|i| {
                                    if let Some(k) = self
                                        .basis
                                        .iter()
                                        .enumerate()
                                        .find_map(|(k, j)| (j == &i).then_some(k))
                                    {
                                        self.rhs[k].to_owned()
                                    } else {
                                        Zero::zero()
                                    }
                                })
                                .collect(),
                            fn_val: match self.function_estimation().try_into() {
                                Ok(val) => val,
                                Err(err_msg) => {
                                    log::error!("{err_msg}");
                                    break 'b Err(SolutionError::Infinite);
                                }
                            },
                        })
                    },
                ),
                pivot_col,
            ),
        }
    }

    fn step_max(
        &mut self,
        _prev_pivot_column: Option<usize>,
    ) -> (Option<SolutionResult>, Option<usize>) {
        unimplemented!()
    }
}
