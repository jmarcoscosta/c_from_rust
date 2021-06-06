unsigned int fibonacci(unsigned int x)
{
	return x < 2 ? x : fibonacci(x - 1) + fibonacci(x - 2);
}
