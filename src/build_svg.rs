use crate::game::Game;
use std::i16::{MAX as I16MAX, MIN as I16MIN};
use svg::Document;
use svg::node::element::Circle;
use svg::node::element::Line;

const SPACING: i32 = 25;
const RADIUS: i32 = 5;
const STROKE: i32 = 3;

pub fn display_game_as_svg(filename: &str, game: &Game) {
	// Using viewBox default from Inkscape on my machine.
	let mut document = Document::new()
		.set("viewBox", (0, 0, 210, 297));
	println!("Created document with viewBox");
	for point in game.points.keys() {
		let circle = Circle::new()
			.set("cx", point.x as i32 * SPACING)
			.set("cy", point.y as i32 * SPACING)
			.set("r", RADIUS)
			.set("fill", "black");
		document = document.add(circle);
	}
	println!("Added points to document.");
	for set in game.sets.iter() {
		let (x, y) = set.direction.full_step();
		let (x, y) = (x as i32, y as i32);
		let line = Line::new()
			.set("x1", set.start_x as i32 * SPACING)
			.set("y1", set.start_y as i32 * SPACING)
			.set("x2", (set.start_x as i32 + x) * SPACING)
			.set("y2", (set.start_y as i32 + y) * SPACING)
			.set("stroke-width", STROKE)
			.set("stroke", "black");
		document = document.add(line);
	}
	println!("Added lines to document.");
	svg::save(filename, &document).expect("Failed to save SVG document.");
}