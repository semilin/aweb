use iced::widget::{button, container, text, text_input, column, row, scrollable};
use iced::{Element, Sandbox, Settings, Point, Theme, Length, Rectangle};
//use iced::{keyboard, mouse};
use serde::{Serialize, Deserialize};
use ron::ser::{to_string_pretty, PrettyConfig};
use directories::ProjectDirs;
use indexmap::IndexMap;
use std::fs::{self, File};
use std::path::Path;
use std::io::{Read, Write};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;


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
    filtered_nodes: Vec<String>,
    filtered_edges: Vec<[String; 3]>,
    node_input: String,
    search_input: String,
}

impl Aweb {
    fn filter(&mut self) {
	let matcher = SkimMatcherV2::default();
	self.filtered_nodes = self.graph.nodes.keys().flat_map(|s| {
	    match matcher.fuzzy_match(s, &self.search_input) {
		Some(_) => Some(s.clone()),
		None => None,
	    }
	}).collect();
	self.filtered_edges = self.graph.edges.iter().flat_map(|Edge {from, val, to}| {
	    if let Some(_) = matcher.fuzzy_match(from, &self.search_input) {
		Some([from.clone(), match val {
		    EdgeVal::Node(s) => s.clone(),
		    _ => "".to_string(),
		}, to.clone()])
	    } else if let Some(_) = matcher.fuzzy_match(to, &self.search_input) {
		Some([from.clone(), match val {
		    EdgeVal::Node(s) => s.clone(),
		    _ => "".to_string(),
		}, to.clone()])
	    } else if let EdgeVal::Node(s) = val {
		if let Some(_) = matcher.fuzzy_match(s, &self.search_input) {
		    Some([from.clone(), s.clone(), to.clone()])
		} else {
		    None
		}
	    } else {
		None
	    }
	}).collect();
    }
}

impl Sandbox for Aweb {
    type Message = Message;
    fn new() -> Self {
	let mut graph = Graph::new();

	if let Some(project_dirs) = ProjectDirs::from("com", "semi", "aweb") {
	    if let Ok(mut file) = File::open(project_dirs.data_dir().join(Path::new("graph.ron"))) {
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		graph = ron::from_str(&contents).unwrap();
	    }
	}

	graph.sort();

	let mut aweb = Self {
	    graph,
	    filtered_nodes: vec![],
	    filtered_edges: vec![],
	    node_input: "".to_string(),
	    search_input: "".to_string()
	};
	aweb.filter();
	aweb
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
		self.filter();
	    },
	    Message::SaveButtonPressed => {
		let pretty = PrettyConfig::new()
		    .depth_limit(2);
		let s = to_string_pretty(&self.graph, pretty).expect("Serialization failed");

		if let Some(project_dirs) = ProjectDirs::from("com", "semi", "aweb") {
		    fs::create_dir_all(project_dirs.data_dir()).unwrap();
		    let path = project_dirs.data_dir().join(Path::new("graph.ron"));
		    let mut output = File::create(path).unwrap();
		    write!(output, "{}", s).unwrap();
		}
	    },
	    Message::SearchInputChanged(s) => {
		self.search_input = s.clone();
		self.filter();
	    },
	}
    }

    fn view(&self) -> Element<Message> {
	let edge_list = column(self.filtered_edges.iter().map(|[from, val, to]| Element::from(
	    row![container(text(from)).width(Length::Fill),
		 container(text(val)).width(Length::Fill),
		 container(text(to)).width(Length::Fill)]
	)).collect())
	    .spacing(4);

	let node_list = column(self.filtered_nodes.iter().map(|s| Element::from(text(s))).collect())
	    .spacing(4);
	
	container(column!(
	    row![button("Save").on_press(Message::SaveButtonPressed)],
	    row![container(column![text("Edges").size(40),
				   scrollable(edge_list).height(Length::Fill)])
		 .width(Length::FillPortion(3)),
		 container(column![text("Nodes").size(40),
				   scrollable(node_list).height(Length::Fill),
				   text_input("add a node", &self.node_input, Message::NodeInputChanged)
				   .on_submit(Message::NodeInputSubmit),
		 ]).width(Length::Fill)].height(Length::FillPortion(5)),
	    container(text_input("search", &self.search_input, Message::SearchInputChanged)).width(Length::Fill)))
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
    SearchInputChanged(String),
}
