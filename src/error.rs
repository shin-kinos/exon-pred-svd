
use std::process;

pub enum ErrorType {
	NumSeqNotOne,
	NonStandardLetter,
	UnexpectedSymbol,
}

pub fn error_bomb( error_type : ErrorType ) {

	println!( "{}", "\n\x1b[31;1m!!! ERROR !!!\x1b[0m\n" );

	match error_type {
		ErrorType::NumSeqNotOne      => println!( "Multi-FASTA format is NOT acceptable, means the number of the sequences in the input FASTA file must be 1." ),
		ErrorType::NonStandardLetter => println!( "Non-standard letter was observed in the input FASTA file." ),
		ErrorType::UnexpectedSymbol  => println!( "Unexpected symbol was observed in the input FASTA file." ),
	}

	println!( "{}", "\n\x1b[31;1m!!! Program halted !!!\x1b[0m\n" );

	process::exit( 1 );
}
