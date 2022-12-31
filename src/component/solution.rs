use std::{cmp::PartialEq, fmt::Display};

use lazy_static::lazy_static;
use nalgebra::{DMatrix, Scalar};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};
use yew::{function_component, html, AttrValue, Html, Properties, UseStateHandle};

use crate::component::Math;

#[derive(Properties, PartialEq)]
pub struct Props<T: Scalar + Display> {
    pub solution: UseStateHandle<Result<(DMatrix<T>, T), Option<AttrValue>>>,
}

lazy_static! {
    static ref ZERO: BigInt = Zero::zero();
}

#[function_component]
pub fn Solution(Props { solution }: &Props<BigRational>) -> Html {
    let ratio_to_latex = |ratio: &BigRational| -> String {
        if ratio.is_integer() {
            ratio.to_integer().to_string()
        } else if !ratio.trunc().is_zero() {
            let whole = ratio.trunc().to_integer();
            let frac = ratio.fract();
            format!(
                r"{whole}\frac{{{numer}}}{{{denom}}}",
                numer = frac.numer(),
                denom = frac.denom()
            )
        } else {
            format!(
                r"{sign}\frac{{{numer}}}{{{denom}}}",
                sign = if ratio.numer() < &ZERO { "-" } else { "" },
                numer = ratio.numer().abs(),
                denom = ratio.denom()
            )
        }
    };

    match &**solution {
        Ok((matrix, function_value)) => {
            html! {<>
                <Math
                    centered=true
                    expression={format!(
                        r"\begin{{pmatrix}}{}\end{{pmatrix}}",
                        matrix
                            .row_iter()
                            .map(|row| row
                                .column_iter()
                                .map(|el| {
                                    let ratio = &el.x;
                                    ratio_to_latex(ratio)
                                })
                                .collect::<Vec<_>>()
                                .join("&")
                            )
                            .collect::<Vec<_>>()
                            .join(r"\\")
                    )}
                />
                <div style="padding-top: 1em;" />
                <Math
                    expression={format!("F={}", ratio_to_latex(function_value))}
                    centered=true
                />
            </>}
        }
        Err(err_msg) => html! { if let Some(err_msg) = err_msg {
            <p>{err_msg}</p>
        }},
    }
}
