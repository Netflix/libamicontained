#include <stdio.h>

#include "cpuinfo.h"

int main(int argc, char *argv[])
{
	printf("num_cpus %d\n", num_cpus());
	printf("recommended_threads %d\n", recommended_threads());
	return 0;
}
