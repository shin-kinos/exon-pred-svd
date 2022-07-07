
#########################################################################################
## A transition animation of line plot for https://github.com/shin-kinos/exon-pred-svd ##
## A line plot with ggplot2 in R visualizing predicted exons by SVD-based approach.    ##
#########################################################################################

######################################################################
## Parameter settings !                                             ##
## You can edit following parameters for appropriate datavis.       ##
## You MUST edit INPUT_FILE_NAME variant into your input file name. ##
######################################################################
INPUT_FILE_NAME  <- "input_data.txt" # Input file name, REQUIRED.
THRESHOLD        <- 0.6              # Decision threshold value.
LINE_SIZE        <- 2.5              # width of line in a plot.
DURATION         <- 2                # Duration of output GIF animation.
FPS              <- 60               # FPS of output GIF animation.
WIDTH            <- 800              # Width of output image size.
HEIGHT           <- 780              # Height of output image size.
OUTPUT_FILE_NAME <- "ProtAnim.gif"   # Output file name.

###############################################
## Install following packages if neccessary. ##
###############################################
#install.packages( "ggplot2" )
#install.packages( "ggthemes" )
#install.packages( "gganimate" )
#install.packages( "gifski" )

library( "ggplot2" )
library( "ggthemes" )
library( "gganimate" )
library( "gifski" )

input <- read.table( INPUT_FILE_NAME, header = TRUE )

pos_svd <- ifelse( input$svd > THRESHOLD, input$svd, NA )
input <- cbind( input, pos_svd )
str( input )

plot <- ggplot( data = input, aes( x = position, y = svd ) )
plot <- plot +
	theme_igray() +
	ggtitle( "Normalized period-3 signals obtained \n using SVD-based approach") +
	ylim( 0, 1.05 ) +
	xlim( 0, length( input$position ) + 10 ) +
	xlab( "Nucleotide position" ) +
	ylab( "SVD" ) +
	geom_line( aes( x = position, y = svd     ), colour = "#3EC70B", size = LINE_SIZE ) +
	geom_line( aes( x = position, y = pos_svd ), colour = "#FF4949", size = LINE_SIZE + .05 ) +
	geom_hline( yintercept = THRESHOLD, linetype = "dashed" ) +
	transition_reveal( position )

animate( plot , duration = DURATION, fps = FPS, width = WIDTH, height = HEIGHT )
anim_save( OUTPUT_FILE_NAME )

#print( plot )
