#include <stdlib.h>

void fun(char *t)
{
    /* do some stuff here */
    
    free(t); //bug!

}

int main()
{
    char c;

    fun(&c);

}



