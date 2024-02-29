if [ $# -ne 1 ]; then
	echo "Wrong argument count. use:"
	echo $0 " <c-source-filename-without-extension>"
	exit -1
fi

cargo run -- c-source/$1.c generated-asm/$1.asm
