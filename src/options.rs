
use std::env;
use std::process;

#[ derive( Debug ) ]
pub enum MappingMethod {
	Eiip,
	Integer,
	PseudoEiip,
	Paired,
}

#[ derive( Debug ) ]
pub enum ThresholdingMethod {
	Average,
}

#[ derive( Debug ) ]
pub enum Colorizing {
	Yes,
	No,
}

pub struct Options {
	pub input  : String,
	pub output : String,
	pub fs     : usize,
	pub mm     : MappingMethod,
	pub tm     : ThresholdingMethod,
	pub sr     : String,
	pub cl     : Colorizing,
}

impl Options {
	pub fn new() -> Options {

		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_o : &String = &String::new();
		let mut arg_f : &String = &( "81".to_string() );
		let mut arg_m : &String = &String::from( "eiip" );
		let mut arg_t : &String = &String::from( "average" );
		let mut arg_s : &String = &String::from( "no" );
		let mut arg_c : &String = &String::from( "no" );

		if argc < 5 { show_usage( &argv[ 0 ] ) };

		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"-i" => { i += 1; arg_i = &argv[ i ]; }
				"-o" => { i += 1; arg_o = &argv[ i ]; }
				"-f" => { i += 1; arg_f = &argv[ i ]; }
				"-m" => { i += 1; arg_m = &argv[ i ]; }
				"-t" => { i += 1; arg_t = &argv[ i ]; }
				"-s" => { i += 1; arg_s = &argv[ i ]; }
				"-c" => { i += 1; arg_c = &argv[ i ]; }
				"-h" => { show_usage( &argv[ 0 ] );   }
				_    => { show_usage( &argv[ 0 ] );   }
			}
			i += 1;
		}

		let mut mm : MappingMethod = MappingMethod::Eiip;
		match ( *arg_m ).as_str() {
			"eiip"       => mm = MappingMethod::Eiip,
			"integer"    => mm = MappingMethod::Integer,
			"pseudoeiip" => mm = MappingMethod::PseudoEiip,
			"paired"     => mm = MappingMethod::Paired,
			_            => show_usage( &argv[ 0 ] ),
		}

		let mut tm : ThresholdingMethod = ThresholdingMethod::Average;
		match ( *arg_t ).as_str() {
			"average" => tm = ThresholdingMethod::Average,
			_         => show_usage( &argv[ 0 ] ),
		}

		let mut sr : String = String::from( "no" );
		match ( *arg_s ).as_str() {
			"yes" => sr = "Yes".to_string(),
			"no"  => sr = "No".to_string(),
			_     => show_usage( &argv[ 0 ] ),
		}

		let mut cl : Colorizing = Colorizing::No;
		match ( *arg_c ).as_str() {
			"yes" => cl = Colorizing::Yes,
			"no"  => cl = Colorizing::No,
			_     => show_usage( &argv[ 0 ] ),
		}

		let input  : String = arg_i.to_string();
		let output : String = arg_o.to_string();
		let fs     : usize  = arg_f.parse().unwrap();

		if fs % 3 != 0 { show_usage( &argv[ 0 ] ); }

		Options {
			input  : input,
			output : output,
			fs     : fs,
			mm     : mm,
			tm     : tm,
			sr     : sr,
			cl     : cl,
		}
	}

	pub fn show_parameter( &self ) {

		println!( "\nParameter set :"                                );
		println!( "================================================" );
		println!( "Input filename      : {}", self.input             );
		println!( "Output filename     : {}", self.output            );
		println!( "Frame size          : {:?}", self.fs              );
		println!( "Mapping method      : {:?}", self.mm              );
		println!( "Thresholding Method : {:?}", self.tm              );
		println!( "Show the result     : {}", self.sr                );
		println!( "Colorize the result : {:?}", self.cl              );
		println!( "================================================" );
	}
}

fn show_usage( arg : &String ) {

	println!( "Usage: {} [Options] \n\nOptions :\n\n", *arg );
	println!( "    -i    Input filename in FASTA format, REQUIRED. 
          Note that a Multi-FASTA format is NOT acceptable, means the input file MUST have just 1 DNA sequence." );
	println!( "    -o    Output filename, REQUIRED." );
	println!( "    -f    Frame size ( default 81 ). Note that this value MUST be a multiple of 3 ( e.g. 81, 102 and 351 etc. )." );
	println!( "    -m    Mapping method to convert a DNA sequence into a numerical one :
              eiip       : Electron-ion interaction potential mapping technique, default,
              integer    : Integer mapping technique,
              pseudoeiip : Pseudoo-EIIP mapping technique,
              paired     : Paired numeric mapping technique," );
	println!( "    -t    Decision thresholding method to detect coding regions :
              average    : The average of the singular values, default," );
	println!( "    -s    Show a result ( yes or no, default no )." );
	println!( "    -c    Colorize the result ( yes or no, default no ).
          If yes, each position number is colorized based on the decision threshold value as follows :
              \x1b[101;37mPosition number\x1b[0m > decision threshold value,
              \x1b[102;30mPosition number\x1b[0m ≦ decision threshold value," );
	println!( "    -h    Print this help, ignore all other arguments." );
	println!( "\n" );

	process::exit( 1 );
}
