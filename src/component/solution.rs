use std::borrow::Cow;
use std::{cmp::PartialEq, fmt::Display};

use lazy_static::lazy_static;
use nalgebra::{DMatrix, Scalar};
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use ratio_extension::{BigRationalExt, RatioExt};
use yew::{function_component, html, AttrValue, Html, Properties, UseStateHandle};

use crate::component::Math;

pub type SolutionOrError<T> = UseStateHandle<Result<(DMatrix<T>, T), Option<AttrValue>>>;

#[derive(Properties, PartialEq)]
pub struct Props<T: Scalar + Display> {
    pub is_loading: UseStateHandle<bool>,
    pub solution_or_err: SolutionOrError<T>,
}

lazy_static! {
    static ref ZERO: BigInt = Zero::zero();
}

#[function_component]
pub fn Solution(Props { is_loading, solution_or_err }: &Props<BigRationalExt>) -> Html {
    let ratio_to_latex = |ratio: &BigRationalExt| -> Cow<'static, str> {
        match ratio {
            RatioExt::Finite(ratio) => (if ratio.is_integer() {
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
            })
            .into(),
            RatioExt::Inf => r"\infty".into(),
            RatioExt::MinusInf => r"-\infty".into(),
            RatioExt::Nan => "NaN".into(),
        }
    };

    if **is_loading {
        return html! { <p>{"Йде обчислення..."}</p> };
    }
    match &**solution_or_err {
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
