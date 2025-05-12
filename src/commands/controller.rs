use std::process;

use crate::{app::App, errors::DoraErrors};

pub struct CommandHandler {}

impl CommandHandler {
    pub fn try_execute(app_state: &mut App, command_str: &str) -> Result<String, DoraErrors> {
        // split string
        let args: Vec<&str> = command_str.split(' ').collect();
        match args[0] {
            "set" => return CommandHandler::handle_set_commands(app_state, args[1..].to_vec()),
            "toggle" => return CommandHandler::handle_toggle_commands(app_state, args[1..].to_vec()),
            _ => {
                return Err(DoraErrors::CommandError(format!(
                    "Unknown Command {}",
                    command_str
                )));
            }
        }
    }

    fn handle_set_commands(app_state: &mut App, args: Vec<&str>) -> Result<String, DoraErrors> {
        let command_prefix = "set";
        match args[0] {
            // set cell_width 10
            "cell-width" | "width" => {
                app_state.config_state.cell_width = CommandHandler::try_get_argument(args, 1)?
                    .parse::<u16>()
                    .map_err(|e| DoraErrors::CommandError(e.to_string()))?;

                // need to refresh dataframe state to re-calculate the appropriate cursor
                // and view bounds such that the dataframe cursor and view operations
                // respect the new cell sizes in the new calculations.
                let config_state = &app_state.config_state;
                app_state
                    .dataframe_state
                    .refresh_renderable_table_size(config_state);
                return Ok("".to_string());
            }
            // set cell_height 3
            "cell-height" | "height" => {
                let input = CommandHandler::try_get_argument(args, 1)?
                    .parse::<u16>()
                    .map_err(|e| DoraErrors::CommandError(e.to_string()))?;
                if input % 2 == 0 {
                    return Err(DoraErrors::CommandError(
                        "Cell-height must be set to an odd number for aesthetic reasons."
                            .to_string(),
                    ));
                }
                app_state.config_state.cell_height = input;
                // need to refresh dataframe state to re-calculate the appropriate cursor
                // and view bounds such that the dataframe cursor and view operations
                // respect the new cell sizes in the new calculations.
                let config_state = &app_state.config_state;
                app_state
                    .dataframe_state
                    .refresh_renderable_table_size(config_state);
                return Ok("".to_string());
            }
            "word-wrap" | "wrap" => {
                // toggle
                if args.len() == 1 {
                    app_state.config_state.word_wrap = !app_state.config_state.word_wrap;
                    return Ok("".to_string());
                }
                let input = CommandHandler::try_get_argument(args, 1)?
                    .parse::<bool>()
                    .map_err(|e| DoraErrors::CommandError(e.to_string()))?;

                app_state.config_state.word_wrap = input;
                return Ok("".to_string());
            }
            _ => {
                let command_str_reconstructed = command_prefix.to_owned() + " " + &args.join(" ");
                Err(DoraErrors::CommandError(format!(
                    "Unknown Command {}",
                    command_str_reconstructed
                )))
            }
        }
    }

    fn handle_toggle_commands(app_state: &mut App, args: Vec<&str>) -> Result<String, DoraErrors> {
        let command_prefix = "toggle";
        match args[0] {
            "word-wrap" | "wrap" => {
                // toggle
                if args.len() == 1 {
                    app_state.config_state.word_wrap = !app_state.config_state.word_wrap;
                    return Ok("".to_string());
                }
                let input = CommandHandler::try_get_argument(args, 1)?
                    .parse::<bool>()
                    .map_err(|e| DoraErrors::CommandError(e.to_string()))?;

                app_state.config_state.word_wrap = input;
                return Ok("".to_string());
            }
            "fzf" => {
                app_state.search_result_state.search_algorithm = 
                    app_state.search_result_state.search_algorithm.next();
                
                let new_search_mode = &app_state.search_result_state.search_algorithm;
                return Ok(format!(
                    "new search mode: {}",
                    new_search_mode.name(),
                )) 
            },

            _ => {
                let command_str_reconstructed = command_prefix.to_owned() + " " + &args.join(" ");
                Err(DoraErrors::CommandError(format!(
                    "Unknown Command {}",
                    command_str_reconstructed
                )))
            }
        }
    }

    fn try_get_argument(args: Vec<&str>, index: usize) -> Result<&str, DoraErrors> {
        match args.get(index) {
            Some(element) => Ok(*element),
            None => Err(DoraErrors::CommandError(format!(
                "Argument in index: {} not found",
                index,
            ))),
        }
    }
}
