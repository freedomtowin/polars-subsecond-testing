use crate::corr;
use dioxus::prelude::*;
use polars::prelude::*;
use subsecond::{call, HotFn};

#[component]
pub fn CorrelationTest() -> Element {
    let mut last_addr = use_signal(|| HotFn::current(corr::correlation_fn).ptr_address());
    let mut results = use_signal(|| "".to_string());
    let mut running = use_signal(|| false);

    let mut run_tests = move || {
        if *running.read() {
            return;
        }
        running.set(true);
        results.set("Running...".into());

        let pos = {
            let a = Series::new("a".into(), &[1, 2, 3, 4, 5]);
            let b = Series::new("b".into(), &[1, 2, 3, 4, 5]);
            call(|| corr::correlation_fn(&a, &b)).unwrap_or(f64::NAN)
        };

        let neg = {
            let a = Series::new("a".into(), &[1, 2, 3, 4, 5]);
            let b = Series::new("b".into(), &[5, 4, 3, 2, 1]);
            call(|| corr::correlation_fn(&a, &b)).unwrap_or(f64::NAN)
        };

        let zero = {
            let a = Series::new("a".into(), &[1, 2, 3, 4, 5]);
            let b = Series::new("b".into(), &[3, 3, 3, 3, 3]);
            call(|| corr::correlation_fn(&a, &b)).unwrap_or(f64::NAN)
        };

        let current_addr = HotFn::current(corr::correlation_fn).ptr_address();
        let patch_note = if current_addr != *last_addr.read() {
            last_addr.set(current_addr);
            "\nPATCH APPLIED! New function version loaded.\n"
        } else {
            ""
        };

        let output = format!(
            "Positive correlation : {:.10}\n\
             Negative correlation : {:.10}\n\
             Constant series      : {:.10}\n\
             \n\
            ",
            pos, neg, zero
        );

        results.set(output);
        running.set(false);
    };

    rsx! {
        div {
            style: "font-family: system-ui, sans-serif; padding: 40px; text-align: center;",
            h1 { "Rust Pearson Correlation Demo" }
            p { "Click the button to run the tests. Edit code → save → click again → see live changes!" }

            button {
                onclick: move |_| run_tests(),
                disabled: *running.read(),
                style: "
                    font-size: 24px;
                    padding: 20px 40px;
                    margin: 30px;
                    background: #007bff;
                    color: white;
                    border: none;
                    border-radius: 12px;
                    cursor: pointer;
                ",
                if *running.read() { "Running..." } else { "Run Tests" }
            }

            pre {
                style: "
                    text-align: left;
                    background: #f4f4f4;
                    padding: 20px;
                    border-radius: 8px;
                    margin: 20px auto;
                    max-width: 800px;
                    white-space: pre-wrap;
                    font-size: 16px;
                ",
                "{results.read()}"
            }

            p {
                style: "color: #666; font-size: 14px;",
                "Tip: Open src/corr.rs to edit the correlation function and trigger hot-reload."
            }
        }
    }
}
