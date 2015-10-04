use rustc_serialize::json::Json;
use core::command::Command;

#[derive(Debug)]
pub struct Menu(Box<[MenuItem]>);

#[derive(Debug)]
pub enum MenuItem {
    Button(String, Command, bool),
    Group(String, Menu),
    Divider
}

impl MenuItem {
    pub fn from_json(mut json: Json) -> Option<MenuItem> {
        match json.as_object_mut() {
            Some(obj) => {
                let caption = match obj.remove("caption") {
                    Some(Json::String(caption)) => caption,
                    _ => return None
                };
                if caption == "-" {
                    return Some(MenuItem::Divider);
                }
                match obj.remove("children") {
                    Some(menu_json) => {
                        let submenu = Menu::from_json(menu_json);
                        return Some(MenuItem::Group(caption, submenu));
                    }
                    _ => {}
                }
                let is_checkbox = obj.remove("checkbox") == Some(Json::Boolean(true));
                let command = match obj.remove("command") {
                    Some(Json::String(command)) => command,
                    _ => return None
                };
                let args = obj.remove("args");
                Some(MenuItem::Button(caption, Command { name: command, args: args}, is_checkbox))
            },
            None => None
        }
    }
}

impl Menu {
    pub fn new() -> Menu {
        Menu(Vec::new().into_boxed_slice())
    }

    pub fn from_json(json: Json) -> Menu {
        let mut items = Vec::<MenuItem>::new();
        if let Json::Array(array) = json {
            for item_json in array {
                match MenuItem::from_json(item_json) {
                    Some(item) => items.push(item),
                    None => {}
                }
            }
        }
        Menu(items.into_boxed_slice())
    }
}
