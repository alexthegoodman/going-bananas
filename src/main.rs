use std::sync::{Arc, Mutex};

use midpoint_engine::{
    core::Viewport::Viewport,
    floem::{
        views::{container, label},
        GpuHelper, IntoView,
    },
    startup::start,
};

fn app_view(gpu_helper: Arc<Mutex<GpuHelper>>, viewport: Arc<Mutex<Viewport>>) -> impl IntoView {
    container((label(|| "Going Bananas / Game UI")))
}

#[tokio::main]
async fn main() {
    println!("Let's... Go Bananas!");

    let project_id = "9b32976f-529b-4e8e-9097-463d6ad9470a".to_string();

    start(app_view, project_id).await;
}
