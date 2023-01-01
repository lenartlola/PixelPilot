use std::cell::RefCell;
use std::rc::Rc;

// An enum representing the different types of nodes in the DOM
#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
    Comment(String),
    Document,
    Doctype,
}

// ElementData holds data for element nodes
#[derive(Debug)]
pub struct ElementData {
    pub(crate) tag_name: String,
    attributes: Vec<Attribute>,
}

// Attribute holds data for a single attribute
#[derive(Debug)]
#[derive(Clone)]
pub struct Attribute {
    name: String,
    value: String,
}

// A node has a zero or more children
#[derive(Debug)]
pub struct Node {
    pub(crate) children: Vec<Node>,
    pub(crate) node_type: NodeType,
    // A reference to a node in the DOM that can be borrowed mutably or immutably.
    parent: Option<Rc<RefCell<Node>>>,
    attributes: Option<Vec<Attribute>>,
    namespace: Option<String>,
    text: Option<String>,
}

impl Node {
    pub fn new_element(tag_name: String, attributes: Vec<Attribute>) -> Node {
        let attribute = attributes.clone();
        Node {
            children: Vec::new(),
            node_type: NodeType::Element(ElementData { tag_name, attributes }),
            parent: None,
            attributes: Some(attribute),
            namespace: None,
            text: None,
        }
    }


    pub fn new_text(text: &str) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(text.to_string()),
            parent: None,
            attributes: None,
            namespace: None,
            text: Some(text.to_string()),
        }
    }

    pub fn new_comment(text: &str) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Comment(text.to_string()),
            parent: None,
            attributes: None,
            namespace: None,
            text: Some(text.to_string()),
        }
    }

    pub fn new_document() -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Document,
            parent: None,
            attributes: None,
            namespace: None,
            text: None,
        }
    }
    
    pub fn new_doctype() -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Doctype,
            parent: None,
            attributes: None,
            namespace: None,
            text: None,
        }
    }
}
