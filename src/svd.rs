
use peroxide::prelude::*;
use crate::options::ThresholdingMethod;

pub struct Svd {
	pub maxima_eigen : Vec<f64>,
	pub threshold    : f64,
}

impl Svd {
	pub fn new() -> Svd {
		let maxima_eigen : Vec<f64> = Vec::new();
		let threshold    : f64      = 0.0;

		Svd {
			maxima_eigen : maxima_eigen,
			threshold    : threshold,
		}
	}

	pub fn run( &mut self, seq_filtered : &Vec<f64>, frame_size : usize ) {

		let seq_length : usize = ( *seq_filtered ).len() - ( frame_size - 1 );

		for i in 0 .. seq_length {
			let subseq : Vec<f64> = ( *seq_filtered )[ i .. ( i + frame_size ) ].to_vec();
			//println!( "{:?}", subseq );
			let a : Matrix = rearrange_to_matrix( &subseq, frame_size ); /*zeros( 3, frame_size / 3 );*/
			//a.print();

			let a_trans          : Matrix   = ( &a ).transpose();
			let a_eigen          : Vec<f64> = eigen( &( ( &a_trans ) * ( &a ) ) ).eigenvalue;
			let a_eigen_max      : f64      = a_eigen.iter().fold( 0.0 / 0.0, | m, v | v.max( m ) );
			let a_eigen_max_sqrt : f64      = a_eigen_max.sqrt();

			//println!( "{:.4}", a_eigen_max_sqrt );
			( self.maxima_eigen ).push( a_eigen_max_sqrt );
		}
	}

	pub fn normalize( &mut self ) {
		let val_max : f64 = ( self.maxima_eigen ).iter().fold( 0.0 / 0.0, | m, v | v.max( m ) );

		for i in 0 .. ( self.maxima_eigen ).len() {
			let val_norm = ( self.maxima_eigen )[ i ] / val_max;
			self.maxima_eigen[ i ] = val_norm;
		}
	}

	pub fn thresholding( &mut self, tm : &ThresholdingMethod ) {
		match *tm {
			ThresholdingMethod::Average => self.threshold = average( &( self.maxima_eigen ) ),
		}
	}

}

fn rearrange_to_matrix( subseq : &Vec<f64>, frame_size : usize ) -> Matrix {

	let mut a : Matrix = zeros( 3, frame_size / 3 );

	for col in 0 .. frame_size / 3 {
		for row in 0 .. 3 {
			a[ ( row, col ) ] = ( *subseq )[ ( col * 3 ) + row ];
		}
	}

	//a.print();

	a
}

fn average( maxima_eigen : &Vec<f64> ) -> f64 {
	
	let mut sum : f64 = 0.0;
	for elem in ( *maxima_eigen ).iter() { sum += *elem; }

	sum / ( ( *maxima_eigen ).len() as f64 )

}
