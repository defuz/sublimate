use std::convert::From;
use std::slice::Iter;
use rustc_serialize::json::Json;
use core::command::Command;

#[derive(Debug, Default)]
pub struct Menu(Box<[MenuItem]>);

#[derive(Debug)]
pub enum MenuItem {
    Button(Option<String>, Command, bool),
    Group(String, Menu),
    Divider,
}

impl Menu {
    pub fn iter<'a>(&'a self) -> Iter<'a, MenuItem> {
        match *self {
            Menu(ref items) => items.iter(),
        }
    }
}

impl From<Json> for Menu {
    fn from(json: Json) -> Menu {
        let mut items = Vec::new();
        if let Json::Array(array) = json {
            for mut item_json in array {
                if let Some(obj) = item_json.as_object_mut() {
                    let caption = match obj.remove("caption") {
                        Some(Json::String(caption)) => Some(caption),
                        _ => None,
                    };
                    if caption == Some("-".to_string()) {
                        items.push(MenuItem::Divider);
                    } else if let Some(menu_json) = obj.remove("children") {
                        items.push(MenuItem::Group(caption.unwrap_or_default(),
                                                   Menu::from(menu_json)));
                    } else if let Some(Json::String(command)) = obj.remove("command") {
                        let is_checkbox = obj.remove("checkbox") == Some(Json::Boolean(true));
                        let args = obj.remove("args");
                        items.push(MenuItem::Button(caption,
                                                    Command {
                                                        name: command,
                                                        args: args,
                                                    },
                                                    is_checkbox))
                    } else {
                        error!("Incorrect menu item: {:?}", obj)
                    }
                }
            }
        }
        Menu(items.into_boxed_slice())
    }
}
