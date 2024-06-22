use charming::{Chart, WasmRenderer};
use leptos::*;

use lazy_static::lazy_static;
use std::collections::BTreeMap;

mod aria;
mod bar;
mod bar3d;
mod boxplot;
mod candlestick;
mod dataset;
mod funnel;
mod gauge;
mod geo;
mod graph;
mod heatmap;
mod line;
mod parallel;
mod pictorial_bar;
mod pie;
mod radar;
mod sankey;
mod scatter;
mod sunburst;
mod theme_river;
mod tree;
mod treemap;


use line::line_gradient::chart;
// use line::stacked_line::chart;

// use line::distribution_of_electricity::chart;
// use line::gradient_stacked_area::chart;
// use line::temperature_change::chart;

// use line::two_value_axes_in_polar::chart;
// use line::data_transform_filter::chart; //test failed


// use bar::set_style_of_single_bar::chart;
// use bar::world_population::chart;
// use bar::radial_polar_bar_label_position::chart;

// use bar3d::bar3d_with_dataset::chart;


// use candlestick::basic_candlestick::chart;
// use candlestick::ohlc::chart;
// use candlestick::shanghai_index::chart;

// use dataset::encode_and_matrix::chart;//json data , 4 table

// use funnel::funnel_chart::chart;
// use funnel::multiple_funnels::chart;

// use gauge::gauge_barometer::chart;
// use gauge::gauge_basic::chart;
// use gauge::gauge_simple::chart;
// use graph::hide_overlapped_label::chart;
// use heatmap::heatmap_on_cartesian::chart;


// use parallel::basic_parallel::chart;

// use pictorial_bar::water_content::chart;
// use pie::doughnut_chart_with_rounded_corner::chart;
// use radar::basic_radar::chart;
// use sankey::basic_sankey::chart;
// use scatter::basic_scatter::chart;
// use sunburst::drink_flavors::chart;
// use theme_river::theme_river_lastfm::chart;
// use tree::from_left_to_right_tree::chart;


// test failed

// use treemap::disk_usage::chart;
// use geo::organ_data::chart;
// use boxplot::boxplot_light_velocity::chart;
// use boxplot::boxplot_light_velocity2::chart;
// use boxplot::data_transform_simple_aggregate::chart;
// use boxplot::multiple_categories::chart;


#[component]
pub fn App() -> impl IntoView {
    let action = create_action(|_input: &()| async {
        let chart: Chart = chart(); //just change your chart
        WasmRenderer::new(600, 400).render("chart", &chart).unwrap();
    });
    action.dispatch(());//show chart on
    view! {
        <div>
            <button on:click=move |_| {action.dispatch(());}>"Show chart"</button>
            <div  id="chart"></div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
