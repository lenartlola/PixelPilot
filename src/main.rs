mod dom;
use dom::Node;
use dom::NodeType;
use dom::Attribute;

fn draw_node(node: &Node, indent: usize) {
    for _ in 0..indent {
        print!(" ");
    }
    match &node.node_type {
        NodeType::Element(element_data) => {
            println!("<{}>", element_data.tag_name);
            for child in &node.children {
                draw_node(child, indent + 2);
            }
        },
        NodeType::Text(text) => {
            println!("#text {}", text);
        },
        NodeType::Comment(comment) => {
            println!("<!-- {} -->", comment);
        },
        NodeType::Document => {
            println!("#document");
            for child in &node.children {
                draw_node(child, indent + 2);
            }
        },
        NodeType::Doctype => {
            println!("<!DOCTYPE html>");
        }
    }
}

fn main() {
    // Create an element node
    let div = Node::new_element("div".to_string(), Vec::new());

    // Create a text node
    let text = Node::new_text("Hello, world!");

    // Create a comment node
    let comment = Node::new_comment("This is a comment!");

    // Create a document node
    let document = Node::new_document();

    // Create a doctype node
    let doctype = Node::new_doctype();

    println!("Created element: {:?}", div);
    println!("Created text: {:?}", text);
    println!("Created comment: {:?}", comment);
    println!("Created document: {:?}", document);
    println!("Created doctype: {:?}", doctype);

    let mut div = Node::new_element("div".to_string(), Vec::new());
    let text = Node::new_text("Hello, world!");
    let comment = Node::new_comment("This is a comment!");
    let mut document = Node::new_document();
    let doctype = Node::new_doctype();

    div.children.push(text);
    div.children.push(comment);
    document.children.push(div);
    document.children.push(doctype);

    draw_node(&document, 0);

}

