
/*
* Anti-notch IIR filter calculated as follows :
*                            1 - (z^-2)                   
* H(z) = -------------------------------------------------
*        1 - 2*(0.992)*cos(2π/3)*(z^-1) + (0.992^2)*(z^-2)
*/

pub struct Filter {
	x1 : f64,
	x2 : f64,
	y1 : f64,
	y2 : f64,
	pub seq_filtered : Vec<f64>,
	pub seq_length   : usize,
}

impl Filter {
	pub fn new() -> Filter {
		let x1 : f64 = 0.0;
		let x2 : f64 = 0.0;
		let y1 : f64 = 0.0;
		let y2 : f64 = 0.0;
		let seq_filtered : Vec<f64> = Vec::new();
		let seq_length   : usize    = 0;

		Filter {
			x1 : x1,
			x2 : x2,
			y1 : y1,
			y2 : y2,
			seq_filtered : seq_filtered,
			seq_length   : seq_length,
		}
	}

	pub fn run( &mut self, b : &Vec<f64>, a : &Vec<f64>, seq_digit : &Vec<f64> ) {
		//let mut output_vec : Vec<f64> = Vec::new();
		let vec_len : usize = ( *seq_digit ).len();

		for i in 0 .. vec_len {
			let input : f64 = ( *seq_digit )[ i ];

			let output : f64 =
				( ( *b )[ 0 ] * input   ) +
				( ( *b )[ 1 ] * self.x1 ) +
				( ( *b )[ 2 ] * self.x2 ) -
				( ( *a )[ 1 ] * self.y1 ) -
				( ( *a )[ 2 ] * self.y2 );

			self.x2 = self.x1;
			self.x1 = input;
			self.y2 = self.y1;
			self.y1 = output;

			( self.seq_filtered ).push( output );
		}

	}

}
