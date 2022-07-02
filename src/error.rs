
use std::process;

pub fn error_bomb( arg : &str ) {

	println!( "{}", "\n\x1b[31;1m!!! ERROR !!!\x1b[0m\n" );

	match arg {
		"num_seq_not_one"     => println!( "Multi-FASTA format is NOT acceptable, means the number of the sequences in the input FASTA file must be 1." ),
		"non_standard_letter" => println!( "Non-standard letter was observed in the input FASTA file." ),
		"unexpected_symbol"   => println!( "Unexpected symbol was observed in the input FASTA file." ),
		_                     => (),
	}

	println!( "{}", "\n\x1b[31;1m!!! Program halted !!!\x1b[0m\n" );

	process::exit( 1 );
}
