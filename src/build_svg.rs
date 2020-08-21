use crate::game::Game;
use std::i16::{MAX as I16MAX, MIN as I16MIN};
use svg::Document;
use svg::node::element::Circle;
use svg::node::element::Line;

const SPACING: i32 = 25;
const RADIUS: i32 = 5;
const STROKE: i32 = 3;

pub fn display_game_as_svg(filename: &str, game: &Game) {
	let mut x_min = I16MAX as i32;
	let mut x_max = I16MIN as i32;
	let mut y_min = x_min;
	let mut y_max = x_max;
	for point in game.points.keys() {
		x_min = x_min.min(point.x as i32);
		y_min = y_min.min(point.y as i32);
		x_max = x_max.max(point.x as i32);
		y_max = y_max.max(point.y as i32);
	}
	// Using viewBox default from Inkscape on my machine.
	let mut document = Document::new()
		.set(
			"viewBox",
			(0, 0, (x_max - x_min) * SPACING + 2 * RADIUS, (y_max - y_min) * SPACING + 2 * RADIUS)
		)
		.set("width", (x_max - x_min) * SPACING + 2 * RADIUS)
		.set("height", (y_max - y_min) * SPACING + 2 * RADIUS)
		.set("x", 0)
		.set("y", 0);
	for point in game.points.keys() {
		let circle = Circle::new()
			.set("cx", (point.x as i32 - x_min) * SPACING + RADIUS)
			.set("cy", (point.y as i32 - y_min) * SPACING + RADIUS)
			.set("r", RADIUS)
			.set("fill", "black");
		document = document.add(circle);
	}
	for set in game.sets.iter() {
		let (x, y) = set.direction.full_step();
		let (x, y) = (x as i32, y as i32);
		let line = Line::new()
			.set("x1", (set.start_x as i32 - x_min) * SPACING + RADIUS)
			.set("y1", (set.start_y as i32 - y_min) * SPACING + RADIUS)
			.set("x2", (set.start_x as i32 + x - x_min) * SPACING + RADIUS)
			.set("y2", (set.start_y as i32 + y - y_min) * SPACING + RADIUS)
			.set("stroke-width", STROKE)
			.set("stroke", "black");
		document = document.add(line);
	}
	svg::save(filename, &document).expect("Failed to save SVG document.");
}