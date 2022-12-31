use std::{cmp::PartialEq, fmt::Display};

use nalgebra::{DMatrix, Scalar};
use yew::{function_component, html, AttrValue, Html, Properties, UseStateHandle};

use crate::component::Math;
use crate::helpers::f64_rounded_string;

#[derive(Properties, PartialEq)]
pub struct Props<T: Scalar + Display> {
    pub solution: UseStateHandle<Result<(DMatrix<T>, T), Option<AttrValue>>>,
}

#[function_component]
pub fn Solution(Props { solution }: &Props<f64>) -> Html {
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
                                    f64_rounded_string(&el.x, 3)
                                    // let Some(ratio) = BigRational::from_float(el.x) else {
                                    //     return el.x.to_string();
                                    // };
                                    // if ratio.trunc().is_integer() {
                                    //     ratio.to_integer().to_string()
                                    // } else {
                                    //     format!(
                                    //         r"{sign}\frac{{{numer}}}{{{denom}}}",
                                    //         sign = if ratio.numer() < &ZERO { "-" } else { "" },
                                    //         numer = ratio.numer().abs(),
                                    //         denom = ratio.denom()
                                    //     )
                                    // }
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
                    expression={format!("F={}", f64_rounded_string(function_value, 3))}
                    centered=true
                />
            </>}
        }
        Err(err_msg) => html! { if let Some(err_msg) = err_msg {
            <p>{err_msg}</p>
        }},
    }
}
