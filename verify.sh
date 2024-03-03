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
	else
		echo "SUCCES: " $1
	fi
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
assert multiplication 20
assert addition-multiplication 23
assert multiplication-addition 32
assert variable 8
assert scalar-variable 16 
assert two-variables 16
assert multiple-variables 14
assert multiple-expressions 144 
assert function-return-int 2
assert function-with-param 2
assert function-two-params 2
assert function-three-params 6
assert function-four-params 24
assert function-with-local-variables 12
assert function-calling-function 12
