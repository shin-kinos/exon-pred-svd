
use std::fs::File;
use std::io::Write;
use crate::options::Colorizing;

pub fn show_result( 
	maxima_eigen : &Vec<f64>,
	cl           : &Colorizing,
	threshold    : f64,
) {
	println!( "\nRESULT :\n");

	println!( "position\tsvd");
	match *cl {
		Colorizing::Yes => colorized_result( maxima_eigen, threshold ),
		Colorizing::No  => non_colorized_result( maxima_eigen ),
	}
}

pub fn save_result(
	maxima_eigen : &Vec<f64>,
	arg_o        : &String
) {

	let mut fout = File::create( ( *arg_o ).as_str() ).expect( "FAILED to open output file" );

	writeln!( fout, "{}", "position\tsvd" ).expect( "FAILED to write" );

	for i in 0 .. ( *maxima_eigen ).len() {
		writeln!( fout, "{}\t{:.4}", i + 1, ( *maxima_eigen )[ i ] ).expect( "FAILED to write" );
	}

	println!( "\nThe output file was correctly written.\n" );
}

fn colorized_result( maxima_eigen : &Vec<f64>, threshold : f64 ) {

	for i in 0 .. ( *maxima_eigen ).len() {
		if ( *maxima_eigen )[ i ] > threshold { println!( "{}\t\x1b[101;37m{:.4}\x1b[0m", i + 1, ( *maxima_eigen )[ i ] ); }
		else                                  { println!( "{}\t\x1b[102;30m{:.4}\x1b[0m", i + 1, ( *maxima_eigen )[ i ] ); }
	}

}

fn non_colorized_result( maxima_eigen : &Vec<f64> ) {

	for i in 0 .. ( *maxima_eigen ).len() {
		println!( "{}\t{:.4}", i + 1, ( *maxima_eigen )[ i ] );
	}

}
