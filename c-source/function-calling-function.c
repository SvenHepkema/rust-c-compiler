int increment(int a) {
	int increment = 1;
	return a + increment;
}

int transform(int a, int b) {
	int scalar = 2;
	return increment(a) * increment(b) * scalar;
}

int main() {
	return transform(1, 2);
}
