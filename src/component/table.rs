use std::ops::Deref;

use nalgebra::{DMatrix, DVector, RowDVector};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::html::onchange;
use yew::{classes, function_component, html, Callback, Html, Properties, UseStateHandle};

use crate::reclone;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub available_ship_line: UseStateHandle<DMatrix<bool>>,
    pub transport_rate: UseStateHandle<DMatrix<f64>>,
    pub cost_rate: UseStateHandle<DMatrix<f64>>,
    pub min_transport_per_line: UseStateHandle<DVector<f64>>,
    pub ships_count_per_type: UseStateHandle<RowDVector<u16>>,
}

#[function_component]
pub fn Table(
    Props {
        available_ship_line,
        transport_rate,
        cost_rate,
        min_transport_per_line,
        ships_count_per_type,
    }: &Props,
) -> Html {
    let n_ships = ships_count_per_type.ncols();
    let n_lines = min_transport_per_line.nrows();

    // Callbacks for inputs:

    let onsubmit = Callback::from(|e: SubmitEvent| e.prevent_default());

    let transport_rate1 = transport_rate.clone();
    let onchange_set_transport_rate =
        Callback::from(move |(e, i, j): (onchange::Event, usize, usize)| {
            let mut new_transport_rate = transport_rate1.deref().to_owned();
            new_transport_rate[(i, j)] = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value_as_number();
            UseStateHandle::set(&transport_rate1, new_transport_rate)
        });

    let cost_rate1 = cost_rate.clone();
    let onchange_set_cost_rate =
        Callback::from(move |(e, i, j): (onchange::Event, usize, usize)| {
            let mut new_cost_rate = cost_rate1.deref().to_owned();
            new_cost_rate[(i, j)] = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value_as_number();
            UseStateHandle::set(&cost_rate1, new_cost_rate)
        });

    let min_transport_per_line1 = min_transport_per_line.clone();
    let onchange_set_min_transport_per_line =
        Callback::from(move |(e, i): (onchange::Event, usize)| {
            let mut new_min_transport_per_line = min_transport_per_line1.deref().to_owned();
            new_min_transport_per_line[i] = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value_as_number();
            UseStateHandle::set(&min_transport_per_line1, new_min_transport_per_line)
        });

    let ships_count_per_type1 = ships_count_per_type.clone();
    let onchange_set_ships_count_per_type =
        Callback::from(move |(e, j): (onchange::Event, usize)| {
            let mut new_ships_count_per_type = ships_count_per_type1.deref().to_owned();
            new_ships_count_per_type[j] = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value_as_number() as u16;
            UseStateHandle::set(&ships_count_per_type1, new_ships_count_per_type)
        });

    let available_ship_line1 = available_ship_line.clone();
    let onchange_set_available_ship_line =
        Callback::from(move |(e, i, j): (onchange::Event, usize, usize)| {
            let mut new_available_ship_line = available_ship_line1.deref().to_owned();
            new_available_ship_line[(i, j)] = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .checked();
            UseStateHandle::set(&available_ship_line1, new_available_ship_line)
        });

    html! {<div class={classes!("input-table", "centered")}><table class={classes!("tg")}>
        <thead>
            <tr>
                <th class={classes!("tg-c3ow")} rowspan=2>{"Номер регулярної лінії"}</th>
                <th class={classes!("tg-c3ow")} colspan={n_ships.to_string()}>{"Тип судна"}</th>
                <th class={classes!("tg-c3ow")} rowspan=2>{"Мінімальний обсяг перевезень"}</th>
            </tr>
            <tr>
                {(1..=n_ships).map(|ship| html!{ // ship type
                    <th key={format!("ship_type_{ship}")} class={classes!("tg-baqh")}>{ship}</th>
                })
                .collect::<Html>()}
            </tr>
        </thead>
        <tbody>
            {{
                reclone!(onsubmit);
                (1..=n_lines).map(move |line| { // i
                    html! {<>
                        <tr>
                            <td class={classes!("tg-c3ow")}>{line}</td>
                            {{
                                reclone!(
                                    onsubmit,
                                    onchange_set_transport_rate,
                                    onchange_set_cost_rate,
                                    onchange_set_available_ship_line
                                );
                                (1..=n_ships).map(move |ship| { //       j, a_ij
                                    reclone!(
                                        onsubmit,
                                        onchange_set_transport_rate,
                                        onchange_set_cost_rate,
                                        onchange_set_available_ship_line,
                                    );
                                    let onchange_set_transport_rate = Callback::from(move |e: onchange::Event|
                                        onchange_set_transport_rate.emit((e, line - 1, ship - 1))
                                    );
                                    let onchange_set_cost_rate = Callback::from(move |e: onchange::Event|
                                        onchange_set_cost_rate.emit((e, line - 1, ship - 1))
                                    );
                                    let onchange_set_available_ship_line = Callback::from(move |e: onchange::Event|
                                        onchange_set_available_ship_line.emit((e, line - 1, ship - 1))
                                    );
                                    html!{<>
                                        <td
                                            key={format!("{line}_{ship}")}
                                            class={classes!("tg-c3ow")}
                                        >
                                            <input
                                                key={format!("{line}_{ship}_checkbox")}
                                                type="checkbox"
                                                checked={available_ship_line[(line - 1, ship - 1)]}
                                                onchange={onchange_set_available_ship_line}
                                            />
                                            <input
                                                key={format!("a_{line}_{ship}_input")}
                                                disabled={!available_ship_line[(line - 1, ship - 1)]}
                                                type="number"
                                                min=0
                                                max=9999
                                                value={transport_rate[(line - 1, ship - 1)].to_string()}
                                                onchange={onchange_set_transport_rate}
                                                onsubmit={onsubmit.clone()}
                                            />
                                            <input
                                                key={format!("c_{line}_{ship}_input")}
                                                disabled={!available_ship_line[(line - 1, ship - 1)]}
                                                type="number"
                                                min=0
                                                max=9999
                                                value={cost_rate[(line-1, ship-1)].to_string()}
                                                onchange={onchange_set_cost_rate}
                                                {onsubmit}
                                            />
                                        </td>
                                    </>}
                                })
                                .collect::<Html>()
                            }}{{
                                reclone!(onsubmit, onchange_set_min_transport_per_line);
                                let onchange = Callback::from(move |e: onchange::Event|
                                    onchange_set_min_transport_per_line.emit((e, line-1))
                                );
                                html! {<td class={classes!("tg-c3ow")}> // a_i
                                    <input
                                        key={format!("a_{line}_input")}
                                        type="number"
                                        min=0
                                        max=99999
                                        value={min_transport_per_line[line-1].to_string()}
                                        {onchange}
                                        {onsubmit}
                                    />
                                </td>}
                            }}
                        </tr>
                    </>}
                }).collect::<Html>()
            }}
            <tr>
                <td class={classes!("tg-c3ow")}>{"Число суден"}</td>
                {(1..=n_ships).map(move |ship| { // ships count, N_j
                    reclone!(onsubmit, onchange_set_ships_count_per_type);
                    let onchange = Callback::from(move |e|
                        onchange_set_ships_count_per_type.emit((e, ship-1))
                    );
                    html!{
                        <td
                            key={format!("n_{ship}")}
                            class={classes!("tg-c3ow")}
                        >
                            <input
                                key={format!("n_{ship}_input")}
                                type="number"
                                min=0
                                max=9999
                                value={ships_count_per_type[ship-1].to_string()}
                                {onchange}
                                {onsubmit}
                            />
                        </td>
                    }
                })
                .collect::<Html>()}
                <td class={classes!("tg-c3ow")}/>
            </tr>
        </tbody>
    </table></div>}
}
