
/* Integer mappong technique */
const INTEGER_T : f64 = 0.0;
const INTEGER_C : f64 = 1.0;
const INTEGER_A : f64 = 2.0;
const INTEGER_G : f64 = 3.0;

/* EIIP mapping technique */
const EIIP_A : f64 = 0.1260;
const EIIP_T : f64 = 0.1335;
const EIIP_C : f64 = 0.1340;
const EIIP_G : f64 = 0.0806;

/* Pseudo EIIP mapping technique */
const PSEUDO_EIIP_A : f64 = 0.1994;
const PSEUDO_EIIP_T : f64 = 0.1933;
const PSEUDO_EIIP_C : f64 = 0.0692;
const PSEUDO_EIIP_G : f64 = 0.0123;

/* Paired numeric mapping technique */
const PAIRED_A : f64 = 1.0;
const PAIRED_T : f64 = 1.0;
const PAIRED_C : f64 = -1.0;
const PAIRED_G : f64 = -1.0;

use crate::options::MappingMethod;

pub struct Mapping {
	pub seq_digit : Vec<f64>,
}

impl Mapping {

	pub fn new() -> Mapping {
		let seq_digit : Vec<f64> = Vec::new();

		Mapping {
			seq_digit : seq_digit,
		}
	}

	pub fn convert_to_digit( &mut self, sequence : &String, mm : &MappingMethod ) {
		match *mm {
			MappingMethod::Integer    => self.seq_digit =     integer( sequence ),
			MappingMethod::Eiip       => self.seq_digit =        eiip( sequence ),
			MappingMethod::PseudoEiip => self.seq_digit = pseudo_eiip( sequence ),
			MappingMethod::Paired     => self.seq_digit =      paired( sequence ),
		}
	}
}

fn integer( sequence : &String ) -> Vec<f64> {

	let seq_vec : Vec<char>   = ( *sequence ).chars().collect();
	let mut output : Vec<f64> = vec![ 0.0; seq_vec.len() ];

	for i in 0 .. seq_vec.len() {
		match seq_vec[ i ] {
			'A' | 'a' => output[ i ] = INTEGER_A,
			'T' | 't' => output[ i ] = INTEGER_T,
			'C' | 'c' => output[ i ] = INTEGER_C,
			'G' | 'g' => output[ i ] = INTEGER_G,
			_         => (),
		}
	}

	//println!( "{:?}", output );

	output
}

fn eiip( sequence : &String ) -> Vec<f64> {

	let seq_vec : Vec<char>   = ( *sequence ).chars().collect();
	let mut output : Vec<f64> = vec![ 0.0; seq_vec.len() ];

	for i in 0 .. seq_vec.len() {
		match seq_vec[ i ] {
			'A' | 'a' => output[ i ] = EIIP_A,
			'T' | 't' => output[ i ] = EIIP_T,
			'C' | 'c' => output[ i ] = EIIP_C,
			'G' | 'g' => output[ i ] = EIIP_G,
			_         => (),
		}
	}

	//println!( "{:?}", output );

	output
}

fn pseudo_eiip( sequence : &String ) -> Vec<f64> {

	let seq_vec : Vec<char>   = ( *sequence ).chars().collect();
	let mut output : Vec<f64> = vec![ 0.0; seq_vec.len() ];

	for i in 0 .. seq_vec.len() {
		match seq_vec[ i ] {
			'A' | 'a' => output[ i ] = PSEUDO_EIIP_A,
			'T' | 't' => output[ i ] = PSEUDO_EIIP_T,
			'C' | 'c' => output[ i ] = PSEUDO_EIIP_C,
			'G' | 'g' => output[ i ] = PSEUDO_EIIP_G,
			_         => (),
		}
	}

	//println!( "{:?}", output );

	output
}

fn paired( sequence : &String ) -> Vec<f64> {

	let seq_vec : Vec<char>   = ( *sequence ).chars().collect();
	let mut output : Vec<f64> = vec![ 0.0; seq_vec.len() ];

	for i in 0 .. seq_vec.len() {
		match seq_vec[ i ] {
			'A' | 'a' => output[ i ] = PAIRED_A,
			'T' | 't' => output[ i ] = PAIRED_T,
			'C' | 'c' => output[ i ] = PAIRED_C,
			'G' | 'g' => output[ i ] = PAIRED_G,
			_         => (),
		}
	}

	//println!( "{:?}", output );

	output
}