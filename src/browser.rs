
//use headless_chrome as chrome;
use super::tui;

use kuchikiki::{traits::*, NodeRef, NodeData};
use serde::{Serialize, Deserialize};


/// Filters out HTML elements & attributes that aren't needed to generate TUI.
pub fn filter_dom_html(html: &str) {
    let dom = parse_content(html);
    only_useful_children(&dom);
    // TBD Get CSS/style information & simplify to TuiStyle/TuiAttribute
}

/// Parse HTML string into HTML component tree.
pub fn parse_content(html: &str) -> NodeRef {
    kuchikiki::parse_html().one(html)
}

/// Remove HTML elements with types that aren't marked for inclusion.
pub fn only_useful_children(dom: &NodeRef) {
    let useful_kinds = &crate::CONFIG.useful_nodes;
    let mut useless: Vec<NodeRef> = Vec::new();
    for child in dom.children() {
        only_useful_children(&child);
        if !useful_kinds.contains(&NodeDataKind::type_of(&child.data())) && child.children().count() == 0 {
            useless.push(child)
    }   }
    for child in useless { child.detach() }
}


/// Data-less representation of NodeData.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum NodeDataKind {
    Element, Text, Comment, ProcessingInstruction, Doctype, Document, DocumentFragment
} impl NodeDataKind {
    pub fn type_of(data: &NodeData) -> Self {
        match data {
            NodeData::Element(_)                => Self::Element,
            NodeData::Text(_)                   => Self::Text,
            NodeData::Comment(_)                => Self::Comment,
            NodeData::ProcessingInstruction(_)  => Self::ProcessingInstruction,
            NodeData::Doctype(_)                => Self::Doctype,
            NodeData::Document(_)               => Self::Document,
            NodeData::DocumentFragment          => Self::DocumentFragment,
        }
    }
}


/// Utility trait for kuchikiki HTML types that need conversion to TUI.
pub trait ToTui {
    type TuiType;
    fn to_tui(&self) -> Self::TuiType;
}

/// HTML DOM node to TUI DOM node.
impl ToTui for kuchikiki::Node {
    type TuiType = tui::TuiNode;
    fn to_tui(&self) -> Self::TuiType {
        tui::TuiNode {
            children: if let Some(child) = self.first_child() {
                child.inclusive_following_siblings().map(|c| c.to_tui()).collect()
            } else { vec![] }
        }
    }
}

/// Actual HTML node data to TUI node data.
impl ToTui for NodeData {
    type TuiType = tui::TuiNodeData;
    fn to_tui(&self) -> Self::TuiType {
        match self {
            NodeData::Element(element) => tui::TuiNodeData::Element(element.to_tui()),
            NodeData::Text(content)    => tui::TuiNodeData::Text(content.clone().into_inner()),
            NodeData::Comment(content) => tui::TuiNodeData::Comment(content.clone().into_inner()),
            /* Chrome handles this */_ => tui::TuiNodeData::Phantom,
        }
    }
}

/// HTML element data to TUI widget data.
impl ToTui for kuchikiki::ElementData {
    type TuiType = tui::TuiElement;
    fn to_tui(&self) -> Self::TuiType {
        tui::TuiElement {
            qual_name: self.name.clone(),
            attributes: self.attributes.borrow().map.iter().map(|(_, a)| a.to_tui()).collect(),
            contents: {
                if let Some(node) = &self.template_contents {
                    node.inclusive_following_siblings().map(|c| c.to_tui()).collect()
                } else { vec![] }
            }
        }
    }
}

/// HTML attribute to TUI widget attribute.
impl ToTui for kuchikiki::Attribute {
    type TuiType = tui::TuiAttribute;
    fn to_tui(&self) -> Self::TuiType {
        tui::TuiAttribute(self.value.clone())
    }
}
