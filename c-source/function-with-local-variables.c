int transform(int a, int b) {
	int increment = 1;
	int a_incremented = a + increment;
	int b_incremented = b + increment;
	int scalar = 2;
	return a_incremented * b_incremented * scalar;
}

int main() {
	return transform(1, 2);
}
