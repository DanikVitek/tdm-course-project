use pretty_assertions::assert_str_eq;

use super::*;

fn prepare_problem() -> Problem {
    Problem::normalize(
        ObjectiveFunction::new(
            RowDVector::from_iterator(
                12,
                [15., 70., 40., 20., 23., 70., 25., 15., 40., 40., 45., 65.]
                    .into_iter()
                    .map(BigRationalExt::from_float),
            ),
            true,
        ),
        vec![
            Constraint::new(
                RowDVector::from_iterator(
                    12,
                    [15., 30., 25., 0., 0., 0., 0., 0., 0., 0., 0., 0.]
                        .into_iter()
                        .map(BigRationalExt::from_float),
                ),
                Sign::Greater,
                BigRationalExt::from_float(300.),
            ),
            Constraint::new(
                RowDVector::from_iterator(
                    12,
                    [0., 0., 0., 10., 25., 50., 0., 0., 0., 0., 0., 0.]
                        .into_iter()
                        .map(BigRationalExt::from_float),
                ),
                Sign::Greater,
                BigRationalExt::from_float(200.),
            ),
            Constraint::new(
                RowDVector::from_iterator(
                    12,
                    [0., 0., 0., 0., 0., 0., 20., 10., 30., 0., 0., 0.]
                        .into_iter()
                        .map(BigRationalExt::from_float),
                ),
                Sign::Greater,
                BigRationalExt::from_float(1000.),
            ),
            Constraint::new(
                RowDVector::from_iterator(
                    12,
                    [0., 0., 0., 0., 0., 0., 0., 0., 0., 50., 17., 45.]
                        .into_iter()
                        .map(BigRationalExt::from_float),
                ),
                Sign::Greater,
                BigRationalExt::from_float(500.),
            ),
            Constraint::new(
                RowDVector::from_iterator(
                    12,
                    [1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0.]
                        .into_iter()
                        .map(BigRationalExt::from_float),
                ),
                Sign::Equals,
                BigRationalExt::from_float(50.),
            ),
            Constraint::new(
                RowDVector::from_iterator(
                    12,
                    [0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0.]
                        .into_iter()
                        .map(BigRationalExt::from_float),
                ),
                Sign::Equals,
                BigRationalExt::from_float(20.),
            ),
            Constraint::new(
                RowDVector::from_iterator(
                    12,
                    [0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1.]
                        .into_iter()
                        .map(BigRationalExt::from_float),
                ),
                Sign::Equals,
                BigRationalExt::from_float(30.),
            ),
        ],
    )
}

#[test]
fn problem_add_constraint_on_var_works_for_3_less_0() {
    let mut problem = prepare_problem();

    problem.add_constraint_on_var(3, Sign::Less, Zero::zero());

    assert_str_eq!(
        problem.constraints.to_string(),
        DMatrix::from_row_slice(
            8,
            24,
            &[
                15., 30., 25., 0., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0., 0.,
                0., 0., 0., 0., //
                0., 0., 0., 10., 25., 50., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0.,
                0., 0., 0., 0., //
                0., 0., 0., 0., 0., 0., 20., 10., 30., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0.,
                0., 0., 0., 0., //
                0., 0., 0., 0., 0., 0., 0., 0., 0., 50., 17., 45., 0., 0., 0., -1., 0., 0., 0., 1.,
                0., 0., 0., 0., //
                1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1.,
                0., 0., 0., //
                0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                1., 0., 0., //
                0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 1., 0., //
                0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 1. //
            ]
        )
        .to_string()
    );
    assert_str_eq!(
        problem.rhs.to_string(),
        DVector::from_column_slice(&[300., 200., 1000., 500., 50., 20., 30., 0.]).to_string()
    );
    assert_str_eq!(
        problem.objective_function.coefficients.to_string(),
        RowDVector::from_row_slice(&[
            BigNumber::from(BigRationalExt::from_float(15.)),
            BigNumber::from(BigRationalExt::from_float(70.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(20.)),
            BigNumber::from(BigRationalExt::from_float(23.)),
            BigNumber::from(BigRationalExt::from_float(70.)),
            BigNumber::from(BigRationalExt::from_float(25.)),
            BigNumber::from(BigRationalExt::from_float(15.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(45.)),
            BigNumber::from(BigRationalExt::from_float(65.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big()
        ])
        .to_string()
    );
}

#[test]
fn problem_add_constraint_on_var_works_for_3_less_1() {
    let mut problem = prepare_problem();

    problem.add_constraint_on_var(3, Sign::Less, One::one());

    assert_str_eq!(
        problem.constraints.to_string(),
        DMatrix::from_row_slice(
            8,
            25,
            &[
                15., 30., 25., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0.,
                0., 0., 0., 0., 0., //
                0., 0., 0., 10., 25., 50., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0.,
                0., 0., 0., 0., 0., //
                0., 0., 0., 0., 0., 0., 20., 10., 30., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1.,
                0., 0., 0., 0., 0., //
                0., 0., 0., 0., 0., 0., 0., 0., 0., 50., 17., 45., 0., 0., 0., 0., -1., 0., 0., 0.,
                1., 0., 0., 0., 0., //
                1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                1., 0., 0., 0., //
                0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 1., 0., 0., //
                0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 1., 0., //
                0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 1. //
            ]
        )
        .to_string()
    );
    assert_str_eq!(
        problem.rhs.to_string(),
        DVector::from_column_slice(&[300., 200., 1000., 500., 50., 20., 30., 1.]).to_string()
    );
    assert_str_eq!(
        problem.objective_function.coefficients.to_string(),
        RowDVector::from_row_slice(&[
            BigNumber::from(BigRationalExt::from_float(15.)),
            BigNumber::from(BigRationalExt::from_float(70.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(20.)),
            BigNumber::from(BigRationalExt::from_float(23.)),
            BigNumber::from(BigRationalExt::from_float(70.)),
            BigNumber::from(BigRationalExt::from_float(25.)),
            BigNumber::from(BigRationalExt::from_float(15.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(45.)),
            BigNumber::from(BigRationalExt::from_float(65.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big()
        ])
        .to_string()
    );
}

#[test]
fn problem_add_constraint_on_var_works_for_3_greater_2() {
    let mut problem = prepare_problem();

    problem.add_constraint_on_var(3, Sign::Greater, BigRationalExt::from_float(2.));

    assert_str_eq!(
        problem.constraints.to_string(),
        DMatrix::from_row_slice(
            8,
            25,
            &[
                15., 30., 25., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0.,
                0., 0., 0., 0., 0., //
                0., 0., 0., 10., 25., 50., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0.,
                0., 0., 0., 0., 0., //
                0., 0., 0., 0., 0., 0., 20., 10., 30., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1.,
                0., 0., 0., 0., 0., //
                0., 0., 0., 0., 0., 0., 0., 0., 0., 50., 17., 45., 0., 0., 0., 0., -1., 0., 0., 0.,
                1., 0., 0., 0., 0., //
                1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                1., 0., 0., 0., //
                0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 1., 0., 0., //
                0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 1., 0., //
                0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0., 1. //
            ]
        )
        .to_string()
    );
    assert_str_eq!(
        problem.rhs.to_string(),
        DVector::from_column_slice(&[300., 200., 1000., 500., 50., 20., 30., 2.]).to_string()
    );
    assert_str_eq!(
        problem.objective_function.coefficients.to_string(),
        RowDVector::from_row_slice(&[
            BigNumber::from(BigRationalExt::from_float(15.)),
            BigNumber::from(BigRationalExt::from_float(70.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(20.)),
            BigNumber::from(BigRationalExt::from_float(23.)),
            BigNumber::from(BigRationalExt::from_float(70.)),
            BigNumber::from(BigRationalExt::from_float(25.)),
            BigNumber::from(BigRationalExt::from_float(15.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(45.)),
            BigNumber::from(BigRationalExt::from_float(65.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big()
        ])
        .to_string()
    );
}

#[test]
fn problem_normalize_works_with_my_variant() {
    let problem = prepare_problem();
    // let big_coefficient = problem.big_coefficient.clone();
    // assert_eq!(big_coefficient, 7_000_000.);
    assert_str_eq!(
        problem.objective_function.coefficients.to_string(),
        RowDVector::from_row_slice(&[
            BigNumber::from(BigRationalExt::from_float(15.)),
            BigNumber::from(BigRationalExt::from_float(70.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(20.)),
            BigNumber::from(BigRationalExt::from_float(23.)),
            BigNumber::from(BigRationalExt::from_float(70.)),
            BigNumber::from(BigRationalExt::from_float(25.)),
            BigNumber::from(BigRationalExt::from_float(15.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(40.)),
            BigNumber::from(BigRationalExt::from_float(45.)),
            BigNumber::from(BigRationalExt::from_float(65.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::from(BigRationalExt::from_float(0.)),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big(),
            BigNumber::one_big()
        ])
        .to_string()
    );
    assert_str_eq!(
        problem.constraints.to_string(),
        DMatrix::from_row_slice(
            7,
            23,
            &[
                15., 30., 25., 0., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0., 0.,
                0., 0., 0., //
                0., 0., 0., 10., 25., 50., 0., 0., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0., 0.,
                0., 0., 0., //
                0., 0., 0., 0., 0., 0., 20., 10., 30., 0., 0., 0., 0., 0., -1., 0., 0., 0., 1., 0.,
                0., 0., 0., //
                0., 0., 0., 0., 0., 0., 0., 0., 0., 50., 17., 45., 0., 0., 0., -1., 0., 0., 0., 1.,
                0., 0., 0., //
                1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1.,
                0., 0., //
                0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                1., 0., //
                0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 1., //
            ]
        )
        .to_string()
    );
    assert_str_eq!(
        problem.rhs.to_string(),
        DVector::from_column_slice(&[300., 200., 1000., 500., 50., 20., 30.]).to_string()
    )
}
