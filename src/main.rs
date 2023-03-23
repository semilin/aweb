use iced::widget::{button, container, text, text_input, column, row, scrollable, vertical_rule};
use iced::{Element, Sandbox, Settings, Point, Theme, Length, Rectangle};
use iced::{keyboard, mouse};
use serde::{Serialize, Deserialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use directories::ProjectDirs;
use indexmap::IndexMap;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone, Debug)]
enum NodeVal {
    Empty
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum EdgeVal {
    Empty,
    Node(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Node {
    name: String,
    val: NodeVal,
}

impl Node {
    fn new(name: &str, val: NodeVal) -> Self {
        Node {
	    name: name.to_string(),
            val,    
	}
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Edge {
    from: String,
    val: EdgeVal,
    to: String,
}

impl Edge {
    fn new(from: &str, val: EdgeVal, to: &str) -> Self {
	Edge {
	    from: from.to_string(),
	    val,
	    to: to.to_string(),
	}
    }
}

#[derive(Serialize, Deserialize)]
struct Graph {
    nodes: IndexMap<String, Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn add_node(&mut self, name: &str, val: NodeVal) {
	self.nodes.insert(name.to_string(), Node::new(name, val));
    }
    fn add_edge(&mut self, edge: Edge) {
	self.edges.push(edge);
    }
    fn sort(&mut self) {
	self.nodes.sort_by(|_, a, _, b| a.name.cmp(&b.name));
    }
    fn new() -> Self {
	Graph {
	    nodes: IndexMap::new(),
	    edges: vec![]
	}
    }
}

pub fn main() -> iced::Result {
    Aweb::run(Settings {
	antialiasing: true,
	..Settings::default()
    })
}

struct Aweb {
    graph: Graph,
    node_input: String,
}

impl Aweb {
}

impl Sandbox for Aweb {
    type Message = Message;
    fn new() -> Self {
	let mut graph = Graph::new();
	graph.add_node("rex", NodeVal::Empty);
	graph.add_node("king", NodeVal::Empty);
	graph.add_node("Latin", NodeVal::Empty);
	graph.add_node("amor", NodeVal::Empty);
	graph.add_node("amoris", NodeVal::Empty);
	graph.add_node("invenio", NodeVal::Empty);
	graph.add_node("invenire", NodeVal::Empty);
	graph.add_node("inveni", NodeVal::Empty);
	graph.add_node("inventus", NodeVal::Empty);
	graph.add_node("genitive singular", NodeVal::Empty);
	graph.add_node("first declension", NodeVal::Empty);
	graph.add_node("infinitive", NodeVal::Empty);
	graph.add_node("third principle part", NodeVal::Empty);
	graph.add_node("perfect passive participle", NodeVal::Empty);
	graph.add_node("declension", NodeVal::Empty);
	graph.add_edge(Edge::new("king", EdgeVal::Node("Latin".to_string()), "rex"));
	graph.add_edge(Edge::new("amor", EdgeVal::Node("genitive singular".to_string()), "amoris"));
	graph.add_edge(Edge::new("amor", EdgeVal::Node("declension".to_string()), "first declension"));
	graph.add_edge(Edge::new("invenio", EdgeVal::Node("infinitive".to_string()), "invenire"));
	graph.add_edge(Edge::new("invenio", EdgeVal::Node("third principle part".to_string()), "inveni"));
	graph.add_edge(Edge::new("invenio", EdgeVal::Node("perfect passive participle".to_string()), "inventus"));

	graph.sort();

	Self {
	    graph,
	    node_input: "".to_string()
	}
    }

    fn title(&self) -> String {
        String::from("Aweb")
    }

    fn update(&mut self, message: Message) {
        match message {
	    Message::NodeInputChanged(s) => {
		self.node_input = s.clone();
	    },
	    Message::NodeInputSubmit => {
		self.graph.nodes.insert(self.node_input.clone(), Node::new(&self.node_input.clone(), NodeVal::Empty));
		self.node_input.clear();
		self.graph.sort();
	    },
	    Message::SaveButtonPressed => {
		let pretty = PrettyConfig::new()
		    .depth_limit(2);
		let s = to_string_pretty(&self.graph, pretty).expect("Serialization failed");

		if let Some(project_dirs) = ProjectDirs::from("com", "semi", "aweb") {
		    fs::create_dir_all(project_dirs.data_dir());
		    let path = project_dirs.data_dir().join(Path::new("graph.ron"));
		    let mut output = File::create(path).unwrap();
		    write!(output, "{}", s).unwrap();
		}
	    }
	}
    }

    fn view(&self) -> Element<Message> {
	let edge_list = column(self.graph.edges.iter().map(|edge| Element::from(
	    row![container(text(self.graph.nodes.get(edge.from.as_str()).unwrap().name.as_str())).width(Length::Fill),
		 container(text(match &edge.val {
		     EdgeVal::Empty => "",
		     EdgeVal::Node(n) => self.graph.nodes.get(n.as_str()).unwrap().name.as_str()
		 })).width(Length::Fill),
		 container(text(self.graph.nodes.get(edge.to.as_str()).unwrap().name.as_str())).width(Length::Fill)]
	)).collect())
	    .spacing(4);

	let node_list = column(self.graph.nodes.values().map(|node| Element::from(text(node.name.as_str()))).collect())
	    .spacing(4);
	
	container(column!(
	    row![button("Save").on_press(Message::SaveButtonPressed)],
	    row![container(column![text("Edges").size(40),
				   scrollable(edge_list).height(Length::Fill)])
		 .width(Length::FillPortion(3)),
		 container(column![text("Nodes").size(40),
				   scrollable(node_list).height(Length::Fill),
				   text_input("add a node", &self.node_input, Message::NodeInputChanged)
				   .on_submit(Message::NodeInputSubmit)]).width(Length::Fill)
		 .width(Length::FillPortion(1))]))
	    .width(Length::Fill)
	    .height(Length::Fill)
	    .padding(12)
	    .into()
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    NodeInputChanged(String),
    NodeInputSubmit,
    SaveButtonPressed,
}
