use ratatui::widgets::{Paragraph, StatefulWidget, Table};
use ratatui::prelude::*;

use crate::app::App;



pub struct TableBanner {}



impl TableBanner {
    pub fn new() -> Self { Self{} }

    fn get_file_name(state: &<TableBanner as ratatui::prelude::StatefulWidget>::State) -> String {
        let app_state = state;
        let df_state = &app_state.dataframe_state;
        df_state.get_file_name()
    }
}

impl StatefulWidget for TableBanner {
    type State = App;
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
       let file_name_str = TableBanner::get_file_name(state);
       let text_para = Paragraph::new(file_name_str);
       text_para.render(area, buf);
    }
}