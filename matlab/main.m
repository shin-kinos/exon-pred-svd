
clear all;

fin = fastaread( "sequence_FZ412301.fasta", IgnoreGaps = true );

input_seq = fin.Sequence;

%disp( input_seq )

seq_length  = length( input_seq );
seq_numeric = assign_eiip( input_seq );
%disp( seq_numeric )

frame_size = 81; % or, frame_size = 351;

seq_numeric = horzcat( seq_numeric, zeros( 1, ( frame_size - 1 ) ) );
%disp( seq_numeric )

R     = 0.992;
theta = ( 2 * pi ) / 3;
b     = [ 1, 0, -1 ];
a     = [ 1, -2 * R * cos(theta), R^2 ];
seq_filtered = filter( b, a, seq_numeric );
%disp( seq_filtered )

for i = 1 : seq_length
    subseq        = seq_filtered( i : i + ( frame_size - 1 ) );
    A             = reshape( subseq, 3, frame_size / 3 );
    S             = svd( A );
    S_max         = max( S );
    S_maxima( i ) = S_max;
end

result = S_maxima / max( S_maxima );
%disp( result )

threshold = mean( result );
plot( result )
axis( [ 0, ( seq_length + 10 ), 0,  1.1 ] )
xlabel( "DNA Sequence (Nucleotide Position)" )
ylabel( "SVD" )
title( "Detection of Period-3 Behavior using SVD" )

fileID = fopen( 'output_matlab(SVD_EIIP_SK).txt','w' );
fprintf( fileID,'%f\n', result );

function seq_numeric = assign_eiip( input_seq )

    for i = 1 : length( input_seq )

        if input_seq( i ) == 'A' || input_seq( i ) == 'a'

            seq_numeric( i ) = 0.1260;

        elseif input_seq( i ) == 'T' || input_seq( i ) == 't'

            seq_numeric( i ) = 0.1335;

        elseif input_seq( i ) == 'C' || input_seq( i ) == 'c'

            seq_numeric( i ) = 0.1340;

        elseif input_seq( i ) == 'G' || input_seq( i ) == 'g'

            seq_numeric( i ) = 0.0806;

        end
    end
end
