// this is literally just mvc
mod state; // model
mod ui;    // view
mod controller;
mod navigator;


pub mod control;
pub use state::ExplorerState;
pub use ui::ExplorerUI;
pub use controller::Controller;