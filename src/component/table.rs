use nalgebra::{DMatrix, DVector, RowDVector};
use yew::{classes, function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub transport_rate: UseStateHandle<DMatrix<f64>>,
    pub cost_rate: UseStateHandle<DMatrix<f64>>,
    pub min_transport_per_line: UseStateHandle<DVector<f64>>,
    pub ships_count_per_type: UseStateHandle<RowDVector<u16>>,
}

#[function_component]
pub fn Table(
    Props {
        transport_rate,
        cost_rate,
        min_transport_per_line,
        ships_count_per_type,
    }: &Props,
) -> Html {
    let n_ships = ships_count_per_type.ncols();
    let n_lines = min_transport_per_line.nrows();

    // let 

    html! {<div class={classes!("input-table", "centered")}><table class={classes!("tg")}>
        <thead>
            <tr>
                <th class={classes!("tg-c3ow")} rowspan=2>{"Номер регулярної лінії"}</th>
                <th class={classes!("tg-c3ow")} colspan={n_ships.to_string()}>{"Тип судна"}</th>
                <th class={classes!("tg-c3ow")} rowspan=2>{"Мінімальний обсяг перевезень"}</th>
            </tr>
            <tr>
                {(1..=n_ships) // ship type
                    .map(|ship| html!{ 
                        <th key={format!("ship_type_{ship}")} class={classes!("tg-baqh")}>{ship}</th>
                    })
                    .collect::<Html>()}
            </tr>
        </thead>
        <tbody>
            {(1..=n_lines).map(|line| html! {<> // i
                <tr>
                    <td class={classes!("tg-c3ow")} rowspan=2>{line}</td>
                    {(1..=n_ships) //       j, a_ij
                        .map(|ship| html!{
                            <td
                                key={format!("a_{line}_{ship}")}
                                class={classes!("tg-c3ow")}
                            >
                                {&transport_rate[(line-1, ship-1)]}
                            </td>
                        })
                        .collect::<Html>()}
                    <td class={classes!("tg-c3ow")} rowspan=2> // a_i
                        {&min_transport_per_line[line-1]}
                    </td>
                </tr>
                <tr>
                    {(1..=n_ships) //       j, c_ij
                        .map(|ship| html!{
                            <td
                                key={format!("c_{line}_{ship}")}
                                class={classes!("tg-c3ow")}
                            >
                                {&cost_rate[(line-1, ship-1)]}
                            </td>
                        })
                        .collect::<Html>()}
                </tr>
            </>}).collect::<Html>()}
            <tr>
                <td class={classes!("tg-c3ow")}>{"Число суден"}</td>
                { // ships count, N_j
                    (1..=n_ships)
                        .map(|ship| html!{
                            <td
                                key={format!("n_{ship}")}
                                class={classes!("tg-c3ow")}
                            >
                                {&ships_count_per_type[ship-1]}
                            </td>
                        })
                        .collect::<Html>()
                }
                <td class={classes!("tg-c3ow")}/>
            </tr>
        </tbody>
    </table></div>}
}
