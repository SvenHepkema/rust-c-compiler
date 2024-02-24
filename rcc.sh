if [ $# -ne 1 ]; then
	echo "Enter a filename of any of the c source files as the first positional argument."
	exit -1
fi

cargo run -- c-source/$1.c generated-asm/$1.asm

echo "="
echo "="
echo "Assembling..."

make $1.asm 

echo "="
echo "="
echo "Executing:"

./rust-binaries/$1
