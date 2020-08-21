use crate::{
    components::SmmdbCoursePanel,
    widgets::{SaveWidget, SmmdbWidget},
};

use iced::{Element, Row};
use indexmap::IndexMap;
use std::path::PathBuf;

pub struct SavePage {
    save: smmdb_lib::Save,
    location: PathBuf,
    save_widget: SaveWidget,
    smmdb_widget: SmmdbWidget,
}

impl SavePage {
    pub fn new(save: smmdb_lib::Save, location: PathBuf) -> SavePage {
        SavePage {
            save_widget: SaveWidget::new(&save),
            save,
            location,
            smmdb_widget: SmmdbWidget::new(),
        }
    }

    pub fn view<'a>(
        &'a mut self,
        smmdb_course_panels: &'a mut IndexMap<String, SmmdbCoursePanel>,
    ) -> Element<crate::Message> {
        Row::new()
            .push(self.save_widget.view(&self.location))
            .push(self.smmdb_widget.view(smmdb_course_panels))
            .into()
    }
}
