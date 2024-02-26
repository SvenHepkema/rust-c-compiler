if [ $# -ne 1 ] && [ $# -ne 3 ] ; then
	echo "Wrong argument count. Either:"
	echo $0 " <binary-directory-name>                                           <= test all binaries in dir"
	echo $0 " <binary-directory-name> <binary-name> <expected-return-status>    <= test specific binary"
	exit -1
fi


DIRECTORY=$1

assert () {
	./$DIRECTORY/$1
	STATUS=$?

	if [ $STATUS -ne $2 ]; then
		echo "FAILED: " $1 " it returned " $STATUS " instead of " $2
		exit -1
	fi
	echo "SUCCES: " $1
}

if [ $# -eq 3 ]  ; then
	assert $2 $3
	exit 0
fi


assert return-ten 10
assert return-hundred 100
assert return-negative-one 255 # Exit codes are 0 to 255, so -1 gets wrapped around
assert addition 11
assert subtraction 9 
assert double-addition 14
assert triple-addition 25
assert addition-subtraction 9


echo "- Passed all tests succesfully"
exit 0
