use std::collections::hash_map::Entry::*;
use std::collections::hash_map::HashMap;

use core::Core;
use core::command::Command;
use core::keymap::{Keymap, Hotkey, HotkeySequence};
use core::context::{Context, Evaluate};

type PerformerNodeId = usize;

#[derive(Default, Debug)]
struct PerformerNode {
    children: HashMap<Hotkey, PerformerNodeId>,
    commands: Vec<(Command, Context)>
}

#[derive(Default, Debug)]
pub struct HotkeyPerformer {
    node_id: PerformerNodeId,
    nodes: Vec<PerformerNode>,
    hotkeys: HashMap<Command, HotkeySequence>
}

impl HotkeyPerformer {
    pub fn new() -> HotkeyPerformer {
        HotkeyPerformer {
            node_id: 0,
            nodes: vec![PerformerNode::default()],
            hotkeys: HashMap::new()
        }
    }

    pub fn add_keymap(&mut self, keymap: Keymap) {
        let mut next_id = self.nodes.len();
        for binding in keymap {
            self.hotkeys.insert(binding.command.clone(), binding.hotkeys.clone());
            let mut node_id = 0;
            for hotkey in binding.hotkeys {
                node_id = *self.nodes[node_id].children.entry(hotkey).or_insert(next_id);
                if node_id == next_id {
                    self.nodes.push(PerformerNode::default());
                    next_id += 1;
                }
            }
            self.nodes[node_id].commands.push((binding.command, binding.context));
        }
    }

    pub fn get_hotkeys(&self, command: &Command) -> Option<&HotkeySequence> {
        self.hotkeys.get(command)
    }

    pub fn perform_hotkey(&mut self, core: &Core, hotkey: &Hotkey) -> Option<Command> {
        if let Some(&node_id) = self.nodes[self.node_id].children.get(hotkey) {
            // Check whether there are commands that binded to the current state
            // If yes, return it
            for &(ref command, ref context) in self.nodes[node_id].commands.iter() {
                if context.evaluate(core) {
                    return Some((*command).clone());
                }
            }
            // If no, change current state
            self.node_id = node_id;
        } else {
            // There's no node for this hotkey, so we reset current status
            self.node_id = 0;
        }
        None
    }
}
