#include <stdio.h>
#include <unistd.h>

int main()
{
    printf("before fork\n");

    if (fork() == 0)
    {
        printf("child\n");
    }
    else
    {
        printf("parent\n");
    }

    printf("hulu\n");
}

// Running the binary vs redirecting its output to a file give
// different results
//
// ./fork_buffering
// ./fork_buffering > data.txt && cat data.txt
//
// This difference is due to how printf handles buffering in case
// of stdout vs writing to a file