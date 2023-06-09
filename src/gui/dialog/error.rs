use super::Dialog;
use crate::gui::app::message_handling::GuiMes;
use iced::{
    widget::{Button, Column, Text},
    Element, Renderer,
};
use iced_aw::style::CardStyles;
use iced_lazy::{component, Component};
use lorecore::errors::LoreCoreError;

#[derive(Debug, Clone)]
pub(crate) struct ErrorDialog {
    error: LoreCoreError,
}

impl ErrorDialog {
    pub(crate) fn new(error: LoreCoreError) -> Self {
        ErrorDialog { error }
    }
}

impl Dialog for ErrorDialog {
    fn card_style(&self) -> CardStyles {
        CardStyles::Danger
    }

    fn header(&self) -> String {
        "Error".to_string()
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone()).into()
    }
}

impl Component<GuiMes, Renderer> for ErrorDialog {
    type State = ();

    type Event = ErrorDialogMes;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<GuiMes> {
        Some(GuiMes::DialogClosed)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let text = Text::new(self.error.to_string());
        let button = Button::new(Text::new("Ok")).on_press(ErrorDialogMes::Close);
        Column::new().push(text).push(button).into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ErrorDialogMes {
    Close,
}
