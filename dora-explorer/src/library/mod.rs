// this is literally just mvc
mod state; // model
mod ui;    // view
mod controller;


pub mod control;
pub use state::ExplorerState;
pub use controller::Controller;