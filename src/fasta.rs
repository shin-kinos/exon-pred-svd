
use std::fs::read_to_string;

use crate::error;
use crate::error::ErrorType;

pub struct Fasta {
	pub title   : String,
	pub seq     : String,
	pub num_seq : usize,
}

impl Fasta {
	pub fn new() -> Fasta {

		let title   : String = String::new();
		let seq     : String = String::new();
		let num_seq : usize  = 0;

		Fasta {
			title   : title,
			seq     : seq,
			num_seq : num_seq,
		}
	}

	pub fn read_fasta_info( &mut self, arg_i : &String ) {

		let fin = read_to_string( ( *arg_i ).as_str() ).expect( "FAILED to open input file" );

		for line in fin.lines() {
			if line.starts_with( ">" ) {
				self.title   += line;
				self.num_seq += 1;
			} else {
				self.seq += line;
			}
		}

	}

	pub fn check_fasta_info( &mut self ) {

		/* The number of the sequences in the input FASTA file MUST be 1. */
		if self.num_seq != 1 { error::error_bomb( ErrorType::NumSeqNotOne ); }

		/* Whether or not the input data includes only { A, T, C, G, a, t, c, g }. */
		check_symbol( &( self.seq ) );

	}

}

fn check_symbol( seq : &String ) {

	let dna_list : Vec<char> = ( *seq ).chars().collect();

	for i in 0 .. dna_list.len() {
		let dna : char = dna_list[ i ];
		match dna {
			'A' | 'T' | 'C' | 'G' | 'a' | 't' | 'c' | 'g' => (),
			'N' | 'U' | 'n' | 'u' | '-' | '.' => {
				println!( "\nFATAL :" );
				println!( "Non-standard letter was observed in the sequence : '{}'", dna );
				println!( "" );
				error::error_bomb( ErrorType::NonStandardLetter );
			},
			_ => {
				println!( "\nFATAL :" );
				println!( "Unexpected symbol was observed in the sequence : '{}'", dna );
				println!( "" );
				error::error_bomb( ErrorType::UnexpectedSymbol );
			},
		}
	}

}
