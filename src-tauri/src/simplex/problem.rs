use std::{
    fmt,
    mem::MaybeUninit,
    ops::{Add, Mul, MulAssign},
};

use derive_more::{Display, IsVariant};
use derive_new::new;
use nalgebra::{Const, DMatrix, DVector, Dynamic, RowDVector, Scalar, UninitMatrix};
use num_rational::BigRational;
use num_traits::Zero;

use crate::simplex::big_number::BigNumber;

use super::{SimplexTable, Solution};

#[derive(Debug, Clone, PartialEq, Display, new)]
#[display(
    fmt = "ObjectiveFunction {{\n    coefficients:\n{}\n    {}\n}}",
    r#"coefficients.to_string().trim().lines().map(|l| format!("{}\n", l.trim())).collect::<String>()"#,
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
    coefficients: RowDVector<f64>,
    sign: Sign,
    rhs: f64,
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
    // big_coefficient
)]
pub struct Problem {
    pub(crate) objective_function: ObjectiveFunction<BigNumber<BigRational>>,
    pub(crate) constraints: DMatrix<BigRational>,
    pub(crate) rhs: DVector<BigRational>,
}

impl Problem {
    pub fn new(objective_function: ObjectiveFunction<f64>, constraints: Vec<Constraint>) -> Self {
        Self::normalize(objective_function, constraints)
    }

    pub fn solve(self) -> Solution {
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

    #[inline]
    fn normalize(
        mut objective_function: ObjectiveFunction<f64>,
        mut constraints: Vec<Constraint>,
    ) -> Self {
        let max_coefficients_count = constraints
            .iter()
            .map(|constraint| constraint.coefficients.len())
            .chain([objective_function.coefficients.len()])
            .max()
            .unwrap();

        assert_ne!(max_coefficients_count, 0);

        constraints
            .iter_mut()
            // Reverse sign on constraints with negative rhs
            .map(|constraint| {
                if constraint.rhs < 0. {
                    *constraint *= -1.;
                }
                &mut constraint.coefficients
            })
            // Add zero coefficients to the constraints and objective function
            .chain([&mut objective_function.coefficients])
            .for_each(|coefficients| {
                let current_len = coefficients.len();
                for _ in 0..max_coefficients_count - current_len {
                    let new_len = coefficients.len();
                    *coefficients = coefficients.clone().insert_column(new_len, 0.);
                }
            });

        // Inserting compensating variables
        let non_equals = constraints
            .iter_mut()
            .enumerate()
            .filter_map(|(i, constraint)| (!constraint.sign.is_equals()).then_some(i))
            .collect::<Vec<_>>();
        for i in non_equals {
            let constraint = &mut constraints[i];
            constraint
                .coefficients
                .extend([if constraint.sign.is_less() { 1. } else { -1. }]);
            objective_function.coefficients.extend([0.]);
            let constraints_count = constraints.len();
            for i in (0..constraints_count).filter(|j| j != &i) {
                constraints[i].coefficients.extend([0.]);
            }
        }

        // Inserting artificial variables
        let is_minimization = objective_function.minimization;
        let mut objective_function: ObjectiveFunction<BigNumber<BigRational>> = ObjectiveFunction {
            n_significant_variables: objective_function.n_significant_variables,
            coefficients: objective_function
                .coefficients
                .map(|f| BigNumber::from(BigRational::from_float(f).unwrap())),
            minimization: is_minimization,
        };
        for i in 0..constraints.len() {
            constraints
                .iter_mut()
                .enumerate()
                .for_each(|(j, constraint)| {
                    constraint
                        .coefficients
                        .extend([if i != j { 0. } else { 1. }]);
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
                DMatrix<MaybeUninit<BigRational>>,
                DVector<MaybeUninit<BigRational>>,
            ) = constraints.into_iter().enumerate().fold(
                (
                    UninitMatrix::uninit(Dynamic::new(nrows), Dynamic::new(ncols)),
                    UninitMatrix::uninit(Dynamic::new(nrows), Const::<1>),
                ),
                |(mut acc_mat, mut acc_vec), (i, constraint)| {
                    acc_mat
                        .row_mut(i)
                        .iter_mut()
                        .zip(constraint.coefficients.into_iter())
                        .for_each(|(acc_mar_row, coefficient)| {
                            acc_mar_row.write(BigRational::from_float(*coefficient).unwrap());
                        });
                    acc_vec
                        .row_mut(i)
                        .iter_mut()
                        .zip([constraint.rhs])
                        .for_each(|(acc_vec_row, rhs)| {
                            acc_vec_row.write(BigRational::from_float(rhs).unwrap());
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
            // big_coefficient,
        }
    }
}

impl Mul<f64> for Sign {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        if rhs >= 0. {
            return self;
        }
        match self {
            Sign::Less => Sign::Greater,
            Sign::Equals => self,
            Sign::Greater => Sign::Less,
        }
    }
}

impl MulAssign<f64> for Sign {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul<f64> for Constraint {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            coefficients: self.coefficients * rhs.clone(),
            rhs: self.rhs * &rhs,
            sign: self.sign * rhs,
        }
    }
}

impl MulAssign<f64> for Constraint {
    fn mul_assign(&mut self, rhs: f64) {
        self.coefficients *= rhs.clone();
        self.rhs *= &rhs;
        self.sign *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;

    use super::*;

    #[test]
    fn problem_normalize_works_with_my_variant() {
        let problem = Problem::normalize(
            ObjectiveFunction::new(
                RowDVector::from_row_slice(&[
                    15., 70., 40., 20., 23., 70., 25., 15., 40., 40., 45., 65.,
                ]),
                true,
            ),
            vec![
                Constraint::new(
                    RowDVector::from_row_slice(&[
                        15., 30., 25., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                    ]),
                    Sign::Greater,
                    300.,
                ),
                Constraint::new(
                    RowDVector::from_row_slice(&[
                        0., 0., 0., 10., 25., 50., 0., 0., 0., 0., 0., 0.,
                    ]),
                    Sign::Greater,
                    200.,
                ),
                Constraint::new(
                    RowDVector::from_row_slice(&[
                        0., 0., 0., 0., 0., 0., 20., 10., 30., 0., 0., 0.,
                    ]),
                    Sign::Greater,
                    1000.,
                ),
                Constraint::new(
                    RowDVector::from_row_slice(&[
                        0., 0., 0., 0., 0., 0., 0., 0., 0., 50., 17., 45.,
                    ]),
                    Sign::Greater,
                    500.,
                ),
                Constraint::new(
                    RowDVector::from_row_slice(&[1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0.]),
                    Sign::Equals,
                    50.,
                ),
                Constraint::new(
                    RowDVector::from_row_slice(&[0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0.]),
                    Sign::Equals,
                    20.,
                ),
                Constraint::new(
                    RowDVector::from_row_slice(&[0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1.]),
                    Sign::Equals,
                    30.,
                ),
            ],
        );
        // let big_coefficient = problem.big_coefficient.clone();
        // assert_eq!(big_coefficient, 7_000_000.);
        assert_str_eq!(
            problem.objective_function.coefficients.to_string(),
            RowDVector::from_row_slice(&[
                BigNumber::<BigRational>::from(BigRational::from_float(15.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(70.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(40.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(20.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(23.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(70.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(25.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(15.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(40.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(40.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(45.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(65.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(0.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(0.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(0.).unwrap()),
                BigNumber::<BigRational>::from(BigRational::from_float(0.).unwrap()),
                BigNumber::<BigRational>::one_big(),
                BigNumber::<BigRational>::one_big(),
                BigNumber::<BigRational>::one_big(),
                BigNumber::<BigRational>::one_big(),
                BigNumber::<BigRational>::one_big(),
                BigNumber::<BigRational>::one_big(),
                BigNumber::<BigRational>::one_big()
            ])
            .to_string()
        );
        assert_str_eq!(
            problem.constraints.to_string(),
            DMatrix::from_row_slice(
                7,
                23,
                &[
                    15., 30., 25., 0., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0.,
                    0., 0., 0., 0., //
                    0., 0., 0., 10., 25., 50., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0.,
                    0., 0., 0., 0., //
                    0., 0., 0., 0., 0., 0., 20., 10., 30., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1.,
                    0., 0., 0., 0., //
                    0., 0., 0., 0., 0., 0., 0., 0., 0., 50., 17., 45., 0., 0., 0., -1., 0., 0., 0.,
                    1., 0., 0., 0., //
                    1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                    1., 0., 0., //
                    0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                    0., 1., 0., //
                    0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0.,
                    0., 0., 1., //
                ]
            )
            .to_string()
        );
        assert_str_eq!(
            problem.rhs.to_string(),
            DVector::from_column_slice(&[300., 200., 1000., 500., 50., 20., 30.]).to_string()
        )
    }
}
