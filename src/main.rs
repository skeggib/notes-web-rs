pub mod model;

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        notes: model::Model::new(),
        selected_note_index: 0,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    notes: model::Model,
    selected_note_index: usize,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    TitleChanged(String),
    ContentsChanged(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    let (current_filename, current_note) = model
        .notes
        .notes
        .iter()
        .nth(model.selected_note_index)
        .unwrap();
    let mut updated_note = current_note.clone();
    match msg {
        Msg::TitleChanged(new_title) => {
            updated_note.title = new_title;
            log!(format!("{}", model.notes));
        }
        Msg::ContentsChanged(new_contents) => {
            updated_note.body = new_contents;
            log!(format!("{}", model.notes));
        }
    }
    *model
        .notes
        .notes
        .entry(current_filename.to_string())
        .or_insert(model::Note::new()) = updated_note;
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    note_editor(
        model
            .notes
            .notes
            .iter()
            .nth(model.selected_note_index)
            .unwrap()
            .1,
    )
}

fn note_editor(note: &model::Note) -> Node<Msg> {
    div![
        input![
            C!["edit"],
            attrs! {At::Value => note.title.clone()},
            input_ev(Ev::Input, |value| { Msg::TitleChanged(value) })
        ],
        textarea![
            C!["edit"],
            attrs! {At::Value => note.body.clone()},
            input_ev(Ev::Input, |value| { Msg::ContentsChanged(value) })
        ],
    ]
}

fn main() {
    App::start("app", init, update, view);
}
