use nalgebra::{DMatrix, DVector, RowDVector};
use num_rational::BigRational;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    command,
    component::{Math, Solution, Table},
    reclone,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    pub fn log_json(json: &JsValue);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    pub async fn listen(name: &str, callback: &Closure<dyn FnMut(String)>) -> JsValue;
}

#[function_component]
pub fn App() -> Html {
    let available_ship_line =
        use_state_eq::<DMatrix<bool>, _>(|| DMatrix::from_row_slice(3, 4, &[true; 12]));

    let transport_rate = use_state_eq::<DMatrix<f64>, _>(|| {
        // DMatrix::from_row_slice(
        //     4,
        //     3,
        //     &[
        //         15., 30., 25., //
        //         10., 25., 50., //
        //         20., 10., 30., //
        //         50., 17., 45., //
        //     ],
        // )
        DMatrix::from_row_slice(
            3,
            4,
            &[
                25., 25., 35., 20., //
                30., 50., 40., 30., //
                15., 15., 25., 10., //
            ],
        )
    });
    let cost_rate = use_state_eq::<DMatrix<f64>, _>(|| {
        // DMatrix::from_row_slice(
        //     4,
        //     3,
        //     &[
        //         15., 70., 40., //
        //         20., 23., 70., //
        //         25., 15., 40., //
        //         40., 45., 65., //
        //     ],
        // )
        DMatrix::from_row_slice(
            3,
            4,
            &[
                15., 30., 10., 30., //
                20., 70., 20., 25., //
                40., 30., 15., 20., //
            ],
        )
    });
    let min_transport_per_line = use_state_eq::<DVector<f64>, _>(
        || /* DVector::from_column_slice(&[300., 200., 1000., 500.]), // */ DVector::from_column_slice(&[600., 2000., 1200.]),
    );
    let ships_count_per_type = use_state_eq::<RowDVector<u16>, _>(
        || /* RowDVector::from_row_slice(&[50, 20, 30]), // */ RowDVector::from_row_slice(&[40, 60, 20, 70]),
    );

    assert_eq!(transport_rate.shape(), cost_rate.shape());
    assert_eq!(ships_count_per_type.ncols(), transport_rate.ncols());
    assert_eq!(min_transport_per_line.nrows(), transport_rate.nrows());

    let i_max = min_transport_per_line.nrows();
    let j_max = ships_count_per_type.ncols();

    let is_loading = use_state_eq(|| false);

    let response =
        use_state::<Result<(DMatrix<BigRational>, BigRational), Option<AttrValue>>, _>(|| {
            Err(None)
        });

    let solve = {
        reclone!(
            available_ship_line,
            transport_rate,
            cost_rate,
            min_transport_per_line,
            ships_count_per_type,
            response,
            is_loading,
        );
        Callback::from(move |_| {
            reclone!(
                available_ship_line,
                transport_rate,
                cost_rate,
                min_transport_per_line,
                ships_count_per_type,
                response,
                is_loading
            );
            spawn_local(async move {
                is_loading.set(true);
                let solution = command::compute(
                    &available_ship_line,
                    &transport_rate,
                    &cost_rate,
                    &min_transport_per_line,
                    &ships_count_per_type,
                )
                .await;
                is_loading.set(false);
                match solution {
                    Ok(solution) => response.set(Ok(solution)),
                    Err(err_msg) => response.set(Err(Some(err_msg.into()))),
                }
            })
        })
    };

    html! {
        <main class={classes!("container")}>
            <Table
                {available_ship_line}
                {transport_rate}
                {cost_rate}
                {min_transport_per_line}
                {ships_count_per_type}
            />
            <div style="padding-top: 2em;"/>
            <Math
                expression={format!(
                    r"\sum\limits_{{i=1}}^{i_max}\sum\limits_{{j=1}}^{j_max} c_{{ij}}n_{{ij}}\rightarrow\min"
                )}
                centered=true
            />
            <p>{"За обмежень:"}</p>
            <Math
                expression={format!(
                    r"\sum\limits_{{i=1}}^{i_max}n_{{ij}}=N_j,\quad j=\overline{{1,{j_max}}}"
                )}
            />
            <Math
                expression={format!(
                    r"\sum\limits_{{j=1}}^{j_max}a_{{ij}}n_{{ij}}\ge a_i,\quad i=\overline{{1,{i_max}}}"
                )}
            />
            <Math
                expression={format!(
                    r"n_{{ij}}\text{{ - ціле}}\ge0,\quad i=\overline{{1,{i_max}}},~ j=\overline{{1,{j_max}}}"
                )}
            />
            <button onclick={solve}>
                {"Розв'язати"}
            </button>

            <Solution {is_loading} solution_or_err={response} />
        </main>
    }
}
