
mod gcloud;
mod local;
mod file_type;
mod path_location;
mod handler;
mod excel;


pub use handler::read_from_any_path;
pub use file_type::FileType;
