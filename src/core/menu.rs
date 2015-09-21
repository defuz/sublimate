use rustc_serialize::json::Json;
use core::command::Command;

pub trait MenuItem : Sized {
    type M : Menu;

    fn divider() -> Option<Self>;
    fn button(caption: String, command: Command, is_checkbox: bool) -> Option<Self>;
    fn group(caption: String, submenu: Self::M) -> Option<Self>;

    fn from_json(mut json: Json) -> Option<Self> {
        match json.as_object_mut() {
            Some(obj) => {
                let caption = match obj.remove("caption") {
                    Some(Json::String(caption)) => caption,
                    _ => return None
                };
                if caption == "-" {
                    return Self::divider();
                }
                match obj.remove("children") {
                    menu_json @ Some(_) => {
                        return Self::group(caption, Self::M::from_json(menu_json))
                    }
                    _ => {}
                }
                let is_checkbox = obj.remove("checkbox") == Some(Json::Boolean(true));
                let command = match obj.remove("command") {
                    Some(Json::String(command)) => command,
                    _ => return None
                };
                let args = obj.remove("args");
                Self::button(caption, Command { name: command, args: args}, is_checkbox)
            },
            None => None
        }
    }
}

pub trait Menu : Sized {
    type I : MenuItem;

    fn from_vec(items: Vec<Self::I>) -> Self;

    fn from_json(json: Option<Json>) -> Self {
        let mut items = Vec::<Self::I>::new();
        match json {
            Some(Json::Array(array)) => {
                for item_json in array {
                    match Self::I::from_json(item_json) {
                        Some(item) => items.push(item),
                        None => {}
                    }
                }
            }
            _ => {}
        }
        return Self::from_vec(items);
    }
}
