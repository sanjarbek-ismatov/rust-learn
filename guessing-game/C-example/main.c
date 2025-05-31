#include <stdio.h>

int main()
{
    printf("Please enter your guess number: \n");
    char guess[100];
    if (scanf("%99s", guess) != 1)
    {
        fprintf(stderr, "I guess this is an error\n");
        return 1;
    }
    printf("Your guess %99s\n", guess);
    return 0;
}