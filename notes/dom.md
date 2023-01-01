# What is a Web Engine?

A web engine is a portion of browser that fetches web pages from the internet and translate it to some readable forms.
Blink, Gecko, WebKit, and Trident are browser engines. In contrast, the browser's own UI—tabs, toolbar,
menu and such—is called the chrome.
Firefox and SeaMonkey are two browsers with different chrome but the same Gecko engine.

A browser engine includes many sub-components: an HTTP client, an HTML parser, a CSS parser, a JavaScript engine
(itself composed of parsers, interpreters, and compilers), and much more.
Those components involved in parsing web formats like HTML and CSS and translating them into what you see on-screen
are sometimes called the layout engine or rendering engine.

# Parsing html:

Most of the time the browser first receives the HTML code, which will then be used as a mold on top of which we will build the page’s UI.

So what does the browser do when it receives the HTML code?

The browser parses the code, building the DOM simultaneously.
In the DOM every HTML element is represented as a node (element) of that DOM.
The browser will find our HTML elements, such as div, p or span and assign/create a JavaScript object
called a Node to each of them. This process is called tokenization.

```html
<!doctype html>
<html>
<head>
    <title>Example Domain</title>
    <style type="text/css">
    </style>    
</head>

<body>
<div>
    <h1>Example</h1>
    <p>An example of html tokenization</p>
</div>
</body>
</html>
```

Illustration:
```
              +------+
              | html |
              +---+--+
                  |
         +--------+--------+
         |                 |
         v                 v
      +------+          +------+
      | head |          | body |
      +--+---+          +---+--+
         |                  |
         |                  |
    +----+------+           |
    v           v           v
+-------+   +-------+    +-------+
| title |   | style |    |  div  |
+-------+   +-------+    +-------+
```

The html is parent of head and body, head and body elements are siblings. every node except of `html` has a parent.

## A list of node types:
```
https://dom.spec.whatwg.org/#dom-node-nodetype

Element
    ELEMENT_NODE (1) 
Attr
    ATTRIBUTE_NODE (2); 
An exclusive Text node
    TEXT_NODE (3); 
CDATASection
    CDATA_SECTION_NODE (4); 
ProcessingInstruction
    PROCESSING_INSTRUCTION_NODE (7); 
Comment
    COMMENT_NODE (8); 
Document
    DOCUMENT_NODE (9); 
DocumentType
    DOCUMENT_TYPE_NODE (10); 
DocumentFragment
    DOCUMENT_FRAGMENT_NODE (11). 
```

So we need a structure of node containing children and the node type,
a parent node could be helpful to iterate on the structure later on.

So there is my complete structure
```rust
// An enum representing the different types of nodes in the DOM
enum NodeType {
    Element(ElementData),
    Text(String),
    Comment(String),
    Document,
    Doctype,
}

// ElementData holds data for element nodes
struct ElementData {
    tag_name: String,
    attributes: Vec<Attribute>,
}

// Attribute holds data for a single attribute
struct Attribute {
    name: String,
    value: String,
}

// A node has a zero or more children
struct Node {
    children: Vec<Node>,
    node_type: NodeType,
    parent: Option<Rc<RefCell<Node>>>,
    attributes: Option<Vec<Attribute>>,
    namespace: Option<String>,
    text: Option<String>,
}
```

First of all, some words about this strange thing `parent: Option<Rc<RefCell<Node>>>`:
These come all from the rust standard library, let's break them down one by one,

- `Option<T>`: Represents an optional value of type T.
  It has two variants: Some(T), which represents a value of type T, and None, which represents the absence of a value.
- `Rc<T>`: `Reference Counted` enables multiple ownership of a value.
   It keeps track of the number of references to a value and automatically
   deallocates the value when the reference count reaches zero.
- `RefCell<T>`: Allows you to borrow values immutably or mutably.

*Put it all together:*
- `parent: Option<Rc<RefCell<Node>>>`: Represents a reference to a node in the DOM that can be borrowed mutably or immutably.
  This can be useful for representing relationships between nodes in the DOM, such as parent-child relationships.

### Getting back to the struct:

As I said, I have a struct of node containing children and node_type, also the parent node is optional but would be
handy to iterate through the node structure later on.

- node_type: We will dig deeper into all the types later.
- attributes: Attributes are used to provide additional information about an element or to modify the default behavior of an element.
  An example would be `href`.
- namespace: Is a way to specify the context in which an element or attribute is defined.
  It is used to disambiguate elements or attributes that have the same name but are used in different contexts.
  For example, the xmlns attribute specifies the default namespace for an element and its descendants.
- text: Is the character data that appears within an element in the DOM.
  it is used to represent the content of an element that is not contained in other elements.
  For example, the text content of a p element is the text that appears between the opening and closing p tags.

Of course, I don't think handle all of these, but I want my struct to be flexible to scale up in the future.

*The types:*

- Element(ElementData): Represents an element node. It includes an ElementData struct that holds the tag name and attributes of the element. 
- Text(String): Represents a text node. It includes a String that holds the text content of the node. 
- Comment(String): Represents a comment node. It includes a String that holds the text of the comment. 
- Document: Represents the root node of the document. It does not include any additional data. 
- Doctype: Represents the document type declaration. It does not include any additional data.
