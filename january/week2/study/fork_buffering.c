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