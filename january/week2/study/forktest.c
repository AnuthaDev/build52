#include <stdio.h>
#include <unistd.h>

int main()
{
    int x = 1;

    if (fork() == 0)
        printf("p1: x=%d\n", ++x);

    printf("p2: x=%d\n", --x);
}