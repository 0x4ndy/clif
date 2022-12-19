/*
 * This is an example application vulnerable to BOF and 
 * crashes when a supplied argument is longer than 10 characters.
 */

#include <stdio.h>
#include <string.h>
int main(int argc, char *argv[])
{
    char buffer[10];
    if (argc < 2)
    {
        printf("Error - missing argument\n");
        return 1;
    }

    strcpy(buffer, argv[1]);
    
    return 0;
}
