use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        title: "Title".to_string(),
        contents: "Contents with\nnew line".to_string(),
    }
}

// ------ ------
//     Model
// ------ ------

type Model = Note;

struct Note {
    title: String,
    contents: String,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    TitleChanged(String),
    ContentsChanged(String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(new_title) => {
            model.title = new_title;
            log!(model.title, model.contents);
        }
        Msg::ContentsChanged(new_contents) => {
            model.contents = new_contents;
            log!(model.title, model.contents);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        input![
            C!["edit"],
            attrs! {At::Value => model.title.clone()},
            input_ev(Ev::Input, |value| { Msg::TitleChanged(value) })
        ],
        textarea![
            C!["edit"],
            attrs! {At::Value => model.contents.clone()},
            input_ev(Ev::Input, |value| { Msg::ContentsChanged(value) })
        ],
    ]
}

fn main() {
    App::start("app", init, update, view);
}
