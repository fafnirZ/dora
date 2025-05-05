use ratatui::widgets::{Paragraph, StatefulWidget, Table};
use ratatui::prelude::*;

use crate::app::{self, App};



pub struct TableBanner {}



impl TableBanner {
    pub fn new() -> Self { Self{} }

    fn get_file_name(state: &<TableBanner as StatefulWidget>::State) -> String {
        let app_state = state;
        let df_state = &app_state.dataframe_state;
        df_state.get_file_name()
    }

    fn get_search_results(state: &<TableBanner as StatefulWidget>::State) -> String {
        let app_state = state;
        let result_state = &app_state.search_result_state;
        let results = &result_state.result_indices;
        let result_cursor = match result_state.result_cursor {
            Some(results) => results,
            None => return String::new(),
        };

        return format!("{}/{} results found", result_cursor, results.len());
    }
}

impl StatefulWidget for TableBanner {
    type State = App;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
       let file_name_str = TableBanner::get_file_name(state);
       let search_result_str = TableBanner::get_search_results(state);

       let string_to_render = file_name_str + "  " + &search_result_str;
       let text_para = Paragraph::new(string_to_render);
       text_para.render(area, buf);
    }
}