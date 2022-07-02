
use std::time::Instant;

mod error;
mod fasta;
mod filtering;
mod mapping;
mod options;
mod result;
mod svd;

fn main() {

	println!( "\nSingular Value Decomposition-based protein-coding region prediction.\n" );

	/* Elapsed time : Start */
	let start = Instant::now();

	/* Set options. */
	let opts = options::Options::new();
	opts.show_parameter();

	/* Read FASTA file and get sequence information. */
	let mut data = fasta::Fasta::new();
	data.read_fasta_info( &( opts.input ) );
	
	/* Check whether the input file is correct FASTA format. */
	data.check_fasta_info();
	//println!( "{:?}", data.seq );

	/* DNA numerical mapping and filtering. */
	let mut maps = mapping::Mapping::new();
	maps.convert_to_digit( &( data.seq ), &( opts.mm ) );
	//println!( "{:?}", maps.seq_digit );

	/* Emphasize the peaks using Anti-notch IIR filter */
	let mut filter       = filtering::Filter::new();
	let pi    : f64      = std::f64::consts::PI;
	let r     : f64      = 0.992;
	let theta : f64      = 2.0 * pi / 3.0;
	let b     : Vec<f64> = vec![ 1.0, 0.0, -1.0 ];
	let a     : Vec<f64> = vec![ 1.0, -2.0 * r * theta.cos(), r * r ];
	for _ in 0 .. ( opts.fs - 1 ) { ( maps.seq_digit ).push( 0.0 ); }
	//for elem in ( maps.seq_digit ).iter() { println!( "{:.4}", *elem ); }
	filter.run( &b, &a, &( maps.seq_digit ) );
	//for elem in ( filter.seq_filtered ).iter() { println!( "{:.4}", *elem ); }

	/* Singular Value decomposition and get the maxima of eigenvalues. */
	let mut svd = svd::Svd::new();
	svd.run( &( filter.seq_filtered ), opts.fs );
	svd.normalize();
	svd.thresholding( &( opts.tm ) );
	//for elem in ( svd.maxima_eigen ).iter() { println!( "{:.4}", *elem ); }
	println!( "\nThreshold : {:.4}\n", svd.threshold );

	/* Show result. */
	if opts.sr == "Yes".to_string() { result::show_result(  &( svd.maxima_eigen ), &( opts.cl ), svd.threshold ) };

	/* Save result. */
	result::save_result( &( svd.maxima_eigen ), &( opts.output ) );

	println!( "{}", "\n\x1b[32;1mProgram completed !!!\x1b[0m\n" );

	/* Elapsed time : End */
	let end = start.elapsed();
	println!( "Total elapsed time : {:?}", end );

}
