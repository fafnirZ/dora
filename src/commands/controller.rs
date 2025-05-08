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
            "cell_width" => {
                app_state.config_state.cell_width = args[1].parse::<u16>().unwrap();
            },
            // set cell_height 3
            "cell_height" => {
                app_state.config_state.cell_height = args[1].parse::<u16>().unwrap();
            },
            _ => {}
        }
    }
}




