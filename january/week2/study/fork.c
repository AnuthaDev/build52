#include <stdio.h>
#include <unistd.h>

int main()
{
    int x = 13;

    int pid = fork();

    if (pid == -1)
    {
        printf("fork failed\n");
    }
    else if (pid == 0)
    {
        // Child process
        printf("I am child\n");
    }
    else
    {
        // Parent Process
        printf("I am parent\n");
    }
    return 0;
}