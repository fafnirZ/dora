// this is literally just mvc
mod state; // model
mod ui;    // view
mod controller;
mod errors;
mod colours;
mod input;
mod mode;
mod filter;
mod internal;

pub mod control;
pub use state::ExplorerState;
pub use ui::ExplorerUI;
pub use controller::Controller;
