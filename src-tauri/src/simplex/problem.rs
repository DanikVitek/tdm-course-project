#[cfg(test)]
mod tests;

use std::{
    fmt, hint,
    mem::{self, MaybeUninit},
    ops::{Add, Mul, MulAssign},
    sync::{Arc, Mutex},
    thread,
};

use derive_more::{Display, IsVariant};
use derive_new::new;
use nalgebra::{Const, DMatrix, DVector, Dynamic, RowDVector, Scalar, UninitMatrix};
use num_traits::{One, Zero};
use ratio_extension::BigRationalExt;
use rayon::prelude::*;

use crate::{helpers::arc_mut, simplex::SolutionError};

use super::{big_number::BigNumber, SimplexTable, Solution, SolutionResult};

#[derive(Debug, Clone, PartialEq, Display, new)]
#[display(
    fmt = "ObjectiveFunction {{\n    coefficients:\n{}\n    {}\n}}",
    r#"coefficients.to_string().trim().lines().map(|l| l.trim().to_owned()).collect::<Vec<_>>().join("\n")"#,
    r#"if *minimization { "Minimization" } else { "Maximization" }"#
)]
pub struct ObjectiveFunction<T>
where
    T: Scalar + fmt::Display + Zero + Add,
{
    #[new(value = "coefficients.iter().filter(|c| *c != &T::zero()).count()")]
    pub(crate) n_significant_variables: usize,
    pub(crate) coefficients: RowDVector<T>,
    pub(crate) minimization: bool,
}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Constraint {
    coefficients: RowDVector<BigRationalExt>,
    sign: Sign,
    rhs: BigRationalExt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IsVariant)]
pub enum Sign {
    Less = -1,
    Equals = 0,
    Greater = 1,
}

#[derive(Debug, Clone, PartialEq, Display)]
#[display(
    fmt = "Problem: {{\n    objective_function: {},\n    constraints:\n{}\n    rhs:\n{}\n}}",
    objective_function,
    r#"constraints.to_string().trim().lines().map(|l| format!("{}\n", l.trim())).collect::<String>()"#,
    r#"rhs.to_string().trim().lines().map(|l| format!("{}\n", l.trim())).collect::<String>()"#,
)]
pub struct Problem {
    pub(crate) objective_function: ObjectiveFunction<BigNumber<BigRationalExt>>,
    pub(crate) constraints: DMatrix<BigRationalExt>,
    pub(crate) rhs: DVector<BigRationalExt>,
}

impl Problem {
    pub fn new(
        objective_function: ObjectiveFunction<BigRationalExt>,
        constraints: Vec<Constraint>,
    ) -> Self {
        Self::normalize(objective_function, constraints)
    }

    pub fn solve(self) -> SolutionResult {
        let mut table = SimplexTable::new(self);

        log::info!("Iteration: 1");
        log::info!("Function estimation: {}", table.function_estimation());
        let (mut solution, mut prev_pivot_col) = table.step(None);
        let mut iteration = 2u32;

        while solution.is_none() {
            log::info!("Iteration: {iteration}");
            log::info!("Function estimation: {}", table.function_estimation());
            (solution, prev_pivot_col) = table.step(prev_pivot_col);
            iteration += 1;
        }

        solution.unwrap()
    }

    pub fn solve_with_whole(self) -> SolutionResult {
        let solution = self.clone().solve()?;

        let progress = "root";
        log::info!("{progress}");

        self.improve(solution, progress)
    }

    fn improve(self, solution: Solution, progress: &str) -> SolutionResult {
        let solution = Arc::new(solution);
        log::info!("Solution:\n{solution}");

        let Solution { vars, .. } = &*solution;

        let Some((i, var)) = vars.par_iter().enumerate().find_map_any(|(i, var)| {
            (!var.is_integer()).then_some((i, var))
        }) else {
            log::info!("Solution has all integer variables. Returning.");
            return Ok(Arc::try_unwrap(solution).unwrap());
        };
        log::info!("Solution has non-integer variables");

        let whole_part: BigRationalExt = var.trunc().into();
        let minimization = self.objective_function.minimization;

        let best_sol = arc_mut::<Option<Solution>>(None);

        // Parallel branches computation
        thread::scope(|s| -> Result<(), SolutionError> {
            let best_sol = best_sol.clone();
            let problem = Arc::new(&self);

            // Left branch
            let left_join_handle = s.spawn({
                let problem = problem.clone();
                let best_sol = best_sol.clone();
                let whole_part = whole_part.clone();
                move || -> Result<(), SolutionError> {
                    Self::add_branch(
                        &format!("{progress}.left"),
                        &problem,
                        i,
                        Sign::Less,
                        whole_part,
                        best_sol,
                        minimization,
                    )
                }
            });

            // Right branch
            let right_join_handle = s.spawn(move || -> Result<(), SolutionError> {
                Self::add_branch(
                    &format!("{progress}.right"),
                    &problem,
                    i,
                    Sign::Greater,
                    whole_part + BigRationalExt::one(),
                    best_sol,
                    minimization,
                )
            });

            left_join_handle.join().unwrap()?;
            right_join_handle.join().unwrap()?;

            Ok(())
        })?;

        log::info!("Computed both branches");

        Arc::try_unwrap(best_sol)
            .unwrap()
            .into_inner()
            .unwrap()
            .ok_or(SolutionError::Absent)
    }

    fn add_branch(
        progress: &str,
        problem: &Problem,
        i: usize,
        constraint_sign: Sign,
        rhs: BigRationalExt,
        best_sol: Arc<Mutex<Option<Solution>>>,
        minimization: bool,
    ) -> Result<(), SolutionError> {
        log::info!("{progress}");
        let mut problem = (*problem).clone();
        problem.add_constraint_on_var(i, constraint_sign, rhs);
        let branch_sol = problem.clone().solve()?;
        let mut best_sol = best_sol.lock().unwrap();
        match (&*best_sol, &branch_sol) {
            (None, Solution { vars, .. }) => {
                if vars.par_iter().all(|var| var.is_integer()) {
                    log::info!("{progress}. Branch all integers. Saving.");
                    *best_sol = Some(branch_sol);
                    return Ok(());
                }
                log::info!("{progress}. Branch could be improved. Branching.");

                let Ok(improved_sol) = problem.improve(branch_sol, progress) else {
                    return Ok(());
                };
                *best_sol = Some(improved_sol);
            }
            (
                Some(Solution {
                    fn_val: best_fn_val,
                    ..
                }),
                Solution {
                    fn_val: branch_fn_val,
                    vars: branch_vars,
                },
            ) => {
                if minimization && best_fn_val <= branch_fn_val {
                    log::info!("{progress}. Branch worse than the best_sol. Returning.");
                    return Ok(());
                } else if !minimization && best_fn_val >= branch_fn_val {
                    log::info!("{progress}. Branch worse than the best_sol. Returning.");
                    return Ok(());
                }
                if branch_vars.par_iter().all(|var| var.is_integer()) {
                    log::info!("{progress}. Branch all integers. Saving.");
                    *best_sol = Some(branch_sol);
                    return Ok(());
                }
                log::info!("{progress}. Branch could be improved. Branching.");
                let Ok(maybe_improved_sol) = problem.improve(branch_sol, progress) else {
                    return Ok(());
                };
                if matches!(
                    &maybe_improved_sol,
                    Solution { fn_val, vars }
                    if ((minimization && fn_val < best_fn_val) || (!minimization && fn_val > best_fn_val))
                    && vars.par_iter().all(|var| var.is_integer())
                ) {
                    *best_sol = Some(maybe_improved_sol);
                }
            }
        }
        Ok(())
    }

    fn add_constraint_on_var(&mut self, i: usize, mut sign: Sign, rhs: BigRationalExt) {
        if sign == Sign::Less && rhs == Zero::zero() {
            sign = Sign::Equals;
        }
        let sign = sign;

        let n_coefs = self.objective_function.coefficients.ncols();
        let n_constr = self.constraints.nrows();
        let n_significant = self.objective_function.n_significant_variables;
        let minimization = self.objective_function.minimization;

        self.constraints = mem::replace(&mut self.constraints, DMatrix::zeros(0, 0))
            .insert_column(n_coefs, Zero::zero());

        match sign {
            Sign::Equals => {
                let mut coefficients: Vec<MaybeUninit<BigRationalExt>> =
                    Vec::with_capacity(n_coefs + 1);
                coefficients.resize_with(n_coefs + 1, MaybeUninit::uninit);

                coefficients[i].write(One::one());
                for j in (0..n_coefs + 1).filter(|j| j != &i && j != &n_coefs) {
                    coefficients[j].write(Zero::zero());
                }

                // Artificial var
                coefficients[n_coefs].write(BigRationalExt::one());

                let coefficients = unsafe { vec_assume_init(coefficients) };
                insert_row(&mut self.constraints, n_constr, coefficients);

                // Objective function
                // Artificial var
                self.objective_function
                    .coefficients
                    .extend([if minimization {
                        BigNumber::one_big()
                    } else {
                        -BigNumber::one_big()
                    }]);
            }
            sign @ Sign::Less | sign @ Sign::Greater => {
                self.constraints = mem::replace(&mut self.constraints, DMatrix::zeros(0, 0))
                    .insert_column(n_significant, Zero::zero());

                let mut coefficients: Vec<MaybeUninit<BigRationalExt>> =
                    Vec::with_capacity(n_coefs + 2);
                coefficients.resize_with(n_coefs + 2, MaybeUninit::uninit);

                coefficients[i].write(One::one());
                for j in (0..n_coefs + 2)
                    .filter(|j| j != &i && j != &n_significant && j != &(n_coefs + 1))
                {
                    coefficients[j].write(Zero::zero());
                }

                // Helper var
                coefficients[n_significant].write(dbg!(match sign {
                    Sign::Less => BigRationalExt::one(),
                    Sign::Greater => -BigRationalExt::one(),
                    _ => unsafe { hint::unreachable_unchecked() },
                }));

                // Artificial var
                coefficients[n_coefs + 1].write(BigRationalExt::one());

                let coefficients = unsafe { vec_assume_init(coefficients) };
                insert_row(&mut self.constraints, n_constr, coefficients);

                // Objective function
                // Helper var
                self.objective_function.coefficients = mem::replace(
                    &mut self.objective_function.coefficients,
                    RowDVector::zeros(0),
                )
                .insert_column(n_significant, Zero::zero());
                // Artificial var
                self.objective_function
                    .coefficients
                    .extend([if minimization {
                        BigNumber::one_big()
                    } else {
                        -BigNumber::one_big()
                    }]);
            }
        }
        self.rhs.extend([rhs]);
    }

    #[inline]
    fn normalize(
        mut objective_function: ObjectiveFunction<BigRationalExt>,
        mut constraints: Vec<Constraint>,
    ) -> Self {
        let max_coefficients_count = constraints
            .par_iter()
            .map(|constraint| constraint.coefficients.len())
            .chain([objective_function.coefficients.len()])
            .max()
            .unwrap();

        assert_ne!(max_coefficients_count, 0);

        constraints
            .par_iter_mut()
            // Reverse sign on constraints with negative rhs
            .map(|constraint| {
                if constraint.rhs < Zero::zero() {
                    *constraint *= -BigRationalExt::one();
                }
                &mut constraint.coefficients
            })
            // Add zero coefficients to the constraints and objective function
            .chain([&mut objective_function.coefficients])
            .for_each(|coefficients| {
                let current_len = coefficients.len();
                for _ in 0..max_coefficients_count - current_len {
                    let new_len = coefficients.len();
                    *coefficients = coefficients.clone().insert_column(new_len, Zero::zero());
                }
            });

        // Inserting compensating variables
        let non_equals = constraints
            .par_iter_mut()
            .enumerate()
            .filter_map(|(i, constraint)| (!constraint.sign.is_equals()).then_some(i))
            .collect::<Vec<_>>();
        for i in non_equals {
            let constraint = &mut constraints[i];
            constraint
                .coefficients
                .extend([if constraint.sign.is_less() {
                    One::one()
                } else {
                    -BigRationalExt::one()
                }]);
            objective_function
                .coefficients
                .extend([BigRationalExt::zero()]);
            let constraints_count = constraints.len();
            for i in (0..constraints_count).filter(|j| j != &i) {
                constraints[i].coefficients.extend([BigRationalExt::zero()]);
            }
        }

        // Inserting artificial variables
        let is_minimization = objective_function.minimization;
        let mut objective_function: ObjectiveFunction<BigNumber<BigRationalExt>> =
            ObjectiveFunction {
                n_significant_variables: objective_function.n_significant_variables,
                coefficients: objective_function.coefficients.map(BigNumber::from),
                minimization: is_minimization,
            };
        for i in 0..constraints.len() {
            constraints
                .par_iter_mut()
                .enumerate()
                .for_each(|(j, constraint)| {
                    constraint.coefficients.extend([if i != j {
                        BigRationalExt::zero()
                    } else {
                        BigRationalExt::one()
                    }]);
                });
            objective_function.coefficients.extend([if is_minimization {
                // big_coefficient
                BigNumber::one_big()
            } else {
                // -big_coefficient
                -BigNumber::one_big()
            }])
        }

        // Reformat
        let (constraints, rhs) = {
            let nrows = constraints.len();
            let ncols = constraints[0].coefficients.len();
            let (constrains, rhs): (
                DMatrix<MaybeUninit<BigRationalExt>>,
                DVector<MaybeUninit<BigRationalExt>>,
            ) = constraints.into_iter().enumerate().fold(
                (
                    UninitMatrix::uninit(Dynamic::new(nrows), Dynamic::new(ncols)),
                    UninitMatrix::uninit(Dynamic::new(nrows), Const::<1>),
                ),
                |(mut acc_mat, mut acc_vec), (i, mut constraint)| {
                    acc_mat
                        .row_mut(i)
                        .iter_mut()
                        .zip(constraint.coefficients.iter_mut())
                        .for_each(|(acc_mar_row, coefficient)| {
                            acc_mar_row.write(mem::take(coefficient));
                        });
                    acc_vec
                        .row_mut(i)
                        .iter_mut()
                        .zip([constraint.rhs])
                        .for_each(|(acc_vec_row, rhs)| {
                            acc_vec_row.write(rhs);
                        });
                    (acc_mat, acc_vec)
                },
            );

            unsafe { (constrains.assume_init(), rhs.assume_init()) }
        };

        Self {
            objective_function,
            constraints,
            rhs,
        }
    }
}

fn insert_row<T>(matrix: &mut DMatrix<T>, i: usize, values: Vec<T>)
where
    T: Clone + Scalar + Default + Zero,
{
    *matrix = mem::replace(matrix, DMatrix::zeros(0, 0)).insert_row(i, Default::default());
    matrix
        .row_mut(i)
        .zip_apply(&RowDVector::from_vec(values), |z, c| *z = c);
}

unsafe fn vec_assume_init<T>(vec: Vec<MaybeUninit<T>>) -> Vec<T>
where
    T: Send,
    MaybeUninit<T>: Sized,
{
    vec.into_par_iter()
        .map(|el| unsafe { el.assume_init() })
        .collect()
}

impl Mul<BigRationalExt> for Sign {
    type Output = Self;

    fn mul(self, rhs: BigRationalExt) -> Self::Output {
        if rhs >= Zero::zero() {
            return self;
        }
        match self {
            Sign::Less => Sign::Greater,
            Sign::Equals => self,
            Sign::Greater => Sign::Less,
        }
    }
}

impl MulAssign<BigRationalExt> for Sign {
    fn mul_assign(&mut self, rhs: BigRationalExt) {
        *self = *self * rhs;
    }
}

impl Mul<BigRationalExt> for Constraint {
    type Output = Self;

    fn mul(self, rhs: BigRationalExt) -> Self::Output {
        Self {
            coefficients: self.coefficients * rhs.clone(),
            rhs: &self.rhs * &rhs,
            sign: self.sign * rhs,
        }
    }
}

impl MulAssign<BigRationalExt> for Constraint {
    fn mul_assign(&mut self, rhs: BigRationalExt) {
        self.coefficients *= rhs.clone();
        self.rhs *= &rhs;
        self.sign *= rhs;
    }
}
