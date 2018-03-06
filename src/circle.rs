use rand;
use rand::Rng;

use network::Network;

use train;
use steps::{TRANSFER, TRANSFER_DERIVITIVE};
use plot;

pub fn dist((px, py): (f64, f64), (zx, zy): (f64, f64)) -> f64 {
	((px - zx).powi(2) + (py - zy).powi(2)).sqrt()
}

pub const RANDOM_INPUT: &'static Fn() -> Vec<f64> = &|| {
	let mut rng = rand::thread_rng();
	[rng.gen::<f64>() * 100.0, rng.gen::<f64>() * 100.0].to_vec()
};

pub const CLASSIFY_FUNCTION: &'static Fn(f64, f64) -> Vec<f64> = &|x, y| {

	if dist((x, y), (50.0, 40.0)) < 35.0 {
		[1.0, 0.0].to_vec()
	} else {
		[0.0, 1.0].to_vec()
	}
};

pub fn gen_guide_points() -> Vec<Vec<f64>> {
	let mut points = Vec::new();

	for x in 0..100 {
		for y in 0..100 {
			if CLASSIFY_FUNCTION(x as f64, y as f64)[0] == 1.0 {
				points.push([x as f64, y as f64].to_vec());
			}
		}
	}

	points
}

pub fn gen(side: bool) -> Vec<f64> {
	let input = RANDOM_INPUT();
	let results = CLASSIFY_FUNCTION(input[0], input[1]);
	let expected = if side { 1.0 } else { 0.0 };
	if results[0] == expected {
		input
	} else {
		gen(side)
	}
}

pub fn gen_even_set(num: usize) -> Vec<Vec<f64>> {
	(0..num).map(|_| gen(true)).chain((0..num).map(|_| gen(false))).collect()
}