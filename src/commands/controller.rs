use crate::app::App;

pub struct CommandHandler {}

impl CommandHandler {
    pub fn try_execute(app_state: &mut App, command_str: &str) {
        // split string
        let args: Vec<&str> = command_str.split(' ').collect();
        match args[0] {
            "set" => CommandHandler::handle_set_commands(app_state, args[1..].to_vec()),
            _ => {}
        }
    }

    fn handle_set_commands(app_state: &mut App, args: Vec<&str>) {
        match args[0] {
            // set cell_width 10
            "cell-width" => {
                app_state.config_state.cell_width = args[1].parse::<u16>().unwrap();
                // need to refresh dataframe state to re-calculate the appropriate cursor
                // and view bounds such that the dataframe cursor and view operations
                // respect the new cell sizes in the new calculations.
                let config_state = &app_state.config_state;
                app_state.dataframe_state.refresh_renderable_table_size(config_state);
            }
            // set cell_height 3
            "cell-height" => {
                app_state.config_state.cell_height = args[1].parse::<u16>().unwrap();
                // need to refresh dataframe state to re-calculate the appropriate cursor
                // and view bounds such that the dataframe cursor and view operations
                // respect the new cell sizes in the new calculations.
                let config_state = &app_state.config_state;
                app_state.dataframe_state.refresh_renderable_table_size(config_state);
            }
            _ => {}
        }
    }
}
