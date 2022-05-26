use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use wasm_zkp_challenge::{compute_msm, PointVectorInput, ScalarVectorInput};
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

static REPEAT: usize = 5;

use thirtyfour::{TimeoutConfiguration, WebDriver};
use std::time::Duration;

#[test]
fn some_print_test() {
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps).await?;
    let timeouts = driver.get_timeouts().await?;
    println!("Page load timeout = {:?}", timeouts.page_load());
}

#[wasm_bindgen_test]
fn benchmark() {
    for size in (8..10).step_by(2) {
        let point_vec = PointVectorInput::new(2 << size);
        let scalar_vec = ScalarVectorInput::new(2 << size);
        let start_time = instant::Instant::now();
        for _ in 0..REPEAT {
            compute_msm(&point_vec, &scalar_vec);
        }
        let end_time = instant::Instant::now();
        console::log_1(
            &format!(
                "Input vector length: 2^{:?}, Latency: {:?}",
                size,
                ((end_time - start_time) / REPEAT as u32)
            )
            .into(),
        );
    }
}
