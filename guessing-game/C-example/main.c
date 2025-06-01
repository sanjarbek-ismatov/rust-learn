#include <stdio.h>

int main()
{
    printf("Please enter your guess number. \n");
    short int const secret_number = 98;
    while (1)
    {
        short int guess;
        if (scanf("%d", &guess) != 1)
        {
            fprintf(stderr, "Type a number!\n");
            continue;
        }
        if (guess > secret_number)
            printf("It is low\n");
        else if (guess == secret_number)
        {
            printf("You won!\n");
            break;
        }
        else
        {
            printf("It is high\n");
        }
    }
    return 0;
}