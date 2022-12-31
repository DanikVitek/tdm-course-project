use std::fmt::Display;

use num_integer::Integer;
use num_rational::Ratio;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq + Clone + Integer + Display,
{
    pub rational: Ratio<T>,
}

#[function_component]
pub fn Rational<T>(Props { rational }: &Props<T>) -> Html
where
    T: PartialEq + Clone + Integer + Display,
{
    html! {<>{ rational }</>}
}
