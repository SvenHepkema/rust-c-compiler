./$1/return-ten

if [ $? -ne 10 ]; then
	echo "Failed return-ten"
	exit -1
fi

./$1/add-two-numbers

if [ $? -ne 3 ]; then
	echo "Failed add-two-numbers"
	exit -1
fi

./$1/function-call

if [ $? -ne 2 ]; then
	echo "Failed function-call"
	exit -1
fi

echo "Passed all tests succesfully"
