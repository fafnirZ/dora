// this is literally just mvc
mod state; // model
mod ui;    // view
mod controller;
mod navigator;
mod errors;
mod colours;
mod infobar;
mod input;
mod mode;
mod filter;

pub mod control;
pub use state::ExplorerState;
pub use ui::ExplorerUI;
pub use controller::Controller;
pub mod exit;