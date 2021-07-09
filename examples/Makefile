# Build the rust test programs 
# Run clean to remove artifacts

CC=rustc

all : file-io fp-calc

file-io : file-io.rs 
	$(CC) $< -o file-io

fp-calc : fp-calc.rs 
	$(CC) $< -o fp-calc

clean : 
	-rm read-write-test.txt file-io fp-calc

