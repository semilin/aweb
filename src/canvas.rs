struct Camera {
    pos: Point,
    zoom: f32,
}

impl Camera {
    fn adjusted_pos(&self, p: Point) -> Point {
	Point {
	    x: p.x - self.pos.x,
	    y: p.y + self.pos.y
	}
    }
}

impl Camera {
    fn new() -> Self {
	Camera {
	    pos: Point::new(0.0, 0.0),
	    zoom: 1.0
	}
    }
}

impl canvas::Program<Message> for Aweb<'_> {
    type State = ();

    fn update(
	&self,
	state: &mut Self::State,
	event: Event,
	bounds: Rectangle,
	cursor: Cursor
    ) -> (event::Status, Option<Message>) {
	let cursor_position =
	    if let Some(position) = cursor.position_in(&bounds) {
		position
	    } else {
		return (event::Status::Ignored, None);
	    };
	match event {
	    Event::Keyboard(kb_event) => {
		let message = match kb_event {
		    keyboard::Event::KeyPressed {key_code: keyboard::KeyCode::Up, ..} => {
			Some(Message::CameraUp)
		    },
		    keyboard::Event::KeyPressed {key_code: keyboard::KeyCode::Left, ..} => {
			Some(Message::CameraLeft)
		    },
		    keyboard::Event::KeyPressed {key_code: keyboard::KeyCode::Right, ..} => {
			Some(Message::CameraRight)
		    },
		    keyboard::Event::KeyPressed {key_code: keyboard::KeyCode::Down, ..} => {
			Some(Message::CameraDown)
		    },
		    _ => None
		};
		(event::Status::Captured, message)
	    },
	    Event::Mouse(mouse_event) => {
		let message = match mouse_event {
		    mouse::Event::WheelScrolled { delta } => {
			Some(Message::CameraZoom(match delta {
			    mouse::ScrollDelta::Lines{y, ..} => y,
			    mouse::ScrollDelta::Pixels{y, ..} => y,
			}))
		    }
		    _ => None
		};
		(event::Status::Captured, message)
	    }
	    _ => (event::Status::Ignored, None)
	}
    }

    fn draw(
	&self,
	_state: &Self::State,
	_theme: &Theme,
	bounds: Rectangle,
	_cursor: Cursor,
    ) -> Vec<Geometry> {
	let graph = self.graph_display.draw(bounds.size(), |frame| {
	    let circles = Path::new(|p| {
		for (_, node) in &self.graph.nodes {
		    p.circle(self.camera.adjusted_pos(node.pos),
			     10.0*self.camera.zoom);
		}
	    });
	    let lines = Path::new(|p| {
		for edge in &self.graph.edges {
		    let from = self.graph.nodes.get(edge.from.as_str()).unwrap();
		    let to = self.graph.nodes.get(edge.to.as_str()).unwrap();
		    p.move_to(self.camera.adjusted_pos(from.pos));
		    p.line_to(self.camera.adjusted_pos(to.pos));
		    println!("({:?}, {:?})", from.pos, to.pos);
		}
	    });

	    frame.stroke(&lines, Stroke::default().with_width(1.5));
	    frame.stroke(&circles, Stroke::default().with_width(1.5));
	});
	vec![graph]
    }

    fn mouse_interaction(
	&self,
	_state: &Self::State,
	bounds: Rectangle,
	cursor: Cursor,
    ) -> mouse::Interaction {
	 mouse::Interaction::default()
    }
}
