use std::sync::{Arc, Mutex};

use midpoint_engine::{
    core::Viewport::Viewport,
    floem::{
        reactive::{create_effect, RwSignal, SignalGet},
        views::{container, label},
        GpuHelper, IntoView,
    },
    startup::{start, GameState},
};

pub struct RootNode {}

impl RootNode {
    pub fn new() -> Self {
        RootNode {}
    }

    pub fn on_frame(&self) {}
}

fn app_view(
    game_ready: RwSignal<bool>,
    game_state: Arc<Mutex<GameState>>,
    gpu_helper: Arc<Mutex<GpuHelper>>,
    viewport: Arc<Mutex<Viewport>>,
) -> impl IntoView {
    create_effect(move |_| {
        let game_ready = game_ready.get();

        if (game_ready) {
            let game_state = game_state.lock().unwrap();

            println!("Game Ready!");

            // game_state.root_node = RootNode::new();

            let renderer_state = game_state
                .renderer_state
                .as_ref()
                .expect("Couldn't get RendererState");
            let renderer_state = renderer_state.lock().unwrap();

            let model_ids: Vec<String> =
                renderer_state.models.iter().map(|m| m.id.clone()).collect();

            println!("Collected Model IDs {:?}", model_ids);

            drop(renderer_state);
            drop(game_state);
        } else {
            println!("Game Not Ready...");
        }
    });

    container((label(|| "Going Bananas / Game UI")))
}

#[tokio::main]
async fn main() {
    println!("Let's... Go Bananas!");

    let project_id = "9b32976f-529b-4e8e-9097-463d6ad9470a".to_string();

    start(app_view, project_id).await;
}
