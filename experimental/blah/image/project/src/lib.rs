#![feature(proc_macro_hygiene, decl_macro)]

use rocket_prometheus::PrometheusMetrics;
#[macro_use] extern crate rocket;

#[get("/")]
fn health() -> &'static str {
    "{ status: UP }"
}

pub fn initialize_metrics() -> rocket::Rocket {
    let prometheus = PrometheusMetrics::new();
    rocket::ignite()
        .mount("/health-blah", routes![health])
        .attach(prometheus.clone())
        .mount("/metrics-blah", prometheus)
}

#[cfg(test)]
mod tests;