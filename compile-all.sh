if [ $# -ne 1 ]; then
	echo "Enter a compiler name, either of: gcc rcc"
	exit -1
fi

ls c-source/ | cut -f1 -d'.' | parallel --jobs 1 make $1-{}
