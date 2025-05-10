use crate::{app::App, errors::DoraErrors};

pub struct CommandHandler {}

impl CommandHandler {
    pub fn try_execute(app_state: &mut App, command_str: &str) -> Result<(), DoraErrors> {
        // split string
        let args: Vec<&str> = command_str.split(' ').collect();
        match args[0] {
            "set" => return CommandHandler::handle_set_commands(app_state, args[1..].to_vec()),
            _ => return Err(
                DoraErrors::CommandError(format!("Unknown Command {}", command_str))
            )
        }
    }

    fn handle_set_commands(app_state: &mut App, args: Vec<&str>) -> Result<(), DoraErrors> {
        let command_prefix = "set";
        match args[0] {
            // set cell_width 10
            "cell-width" => {
                app_state.config_state.cell_width = args[1].parse::<u16>().unwrap();
                // need to refresh dataframe state to re-calculate the appropriate cursor
                // and view bounds such that the dataframe cursor and view operations
                // respect the new cell sizes in the new calculations.
                let config_state = &app_state.config_state;
                app_state.dataframe_state.refresh_renderable_table_size(config_state);
                return Ok(());
            }
            // set cell_height 3
            "cell-height" => {
                app_state.config_state.cell_height = args[1].parse::<u16>().unwrap();
                // need to refresh dataframe state to re-calculate the appropriate cursor
                // and view bounds such that the dataframe cursor and view operations
                // respect the new cell sizes in the new calculations.
                let config_state = &app_state.config_state;
                app_state.dataframe_state.refresh_renderable_table_size(config_state);
                return Ok(());
            }
            _ => {
                let command_str_reconstructed = command_prefix.to_owned() + &args.join(" ");
                Err(
                    DoraErrors::CommandError(format!(
                        "Unknown Command {}", command_str_reconstructed
                    ))
                )
            }
        }
    }
}
