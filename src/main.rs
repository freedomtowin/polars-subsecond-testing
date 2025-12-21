mod corr;
mod float_sum;
mod test_components;

use dioxus::prelude::*;

use subsecond::{call, HotFn};
use dioxus_devtools::connect_subsecond;
use dioxus::desktop::{Config, WindowBuilder};


fn main() {
    // Enable hot-patching
    connect_subsecond();

    // Show initial address (optional)
    #[cfg(feature = "test_correlation")]
    {
        let hotfn = HotFn::current(corr::correlation_fn);
        println!("correlation_fn ready at {:?}", hotfn.ptr_address());
    }

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new()
            .with_window(WindowBuilder::new()
                .with_resizable(true)
            ))
        .launch(app)    
}


pub fn TestApplication() -> Element {
    #[cfg(feature = "test_correlation")]
    {
        // Import and render the full test component when feature is enabled
        return rsx! { test_components::test_corr::CorrelationTest {} };
    }


    #[cfg(not(feature = "test_correlation"))]
    {
        // Render nothing
        rsx! { "" }
    }
}

fn app() -> Element {
    rsx! {
        TestApplication {}
    }
}