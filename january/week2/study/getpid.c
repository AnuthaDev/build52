#include <stdio.h>
#include <unistd.h>

int main()
{
    int pid = getpid();
    // This will change across multiple executions
    printf("pid is %d\n", pid);

    // This will stay the same across executions unless a different
    // shell is used to execute
    int parent_pid = getppid();
    printf("parent pid is %d\n", parent_pid);
}