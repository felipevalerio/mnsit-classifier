struct Tensor2D{ //-> matrix

	data: Vec<f32>,
	rows: usize,
	cols: usize,
}

impl Tensor2D {

	fn new(data: Vec<f32>, rows: usize, cols: usize) -> Self {
		Tensor2D {data, rows, cols};
	}
}

