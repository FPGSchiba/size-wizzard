use eframe::egui::{CollapsingHeader, Ui};

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Keep,
    Delete,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Tree {
    children: Vec<Tree>,
    name: String,
    size: i128,
}

impl Tree {
    pub fn empty() -> Self {
        Self {
            children: vec![],
            name: "root".to_string(),
            size: 0,
        }
    }

    pub fn construct() -> Self {
        Self {
            children: vec![],
            name: "root".to_string(),
            size: 0,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Action {
        self.ui_impl(ui, 0)
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::empty()
    }
}

impl Tree {
    fn ui_impl(&mut self, ui: &mut Ui, depth: usize) -> Action {
        CollapsingHeader::new(format!("{} ({})", self.name, self.size))
            .default_open(depth < 1)
            .show(ui, |ui| self.children_ui(ui, depth))
            .body_returned
            .unwrap_or(Action::Keep)
    }

    fn children_ui(&mut self, ui: &mut Ui, depth: usize) -> Action {
        self.children = std::mem::take(self)
            .children
            .into_iter()
            .enumerate()
            .filter_map(|(i, mut tree)| {
                if tree.ui_impl(ui, depth + 1) == Action::Keep {
                    Some(tree)
                } else {
                    None
                }
            })
            .collect();
        Action::Keep
    }
}
