#include <stdio.h>
#include <string.h>
int main(){
char age[103] = "";

fgets(age, sizeof(age), stdin);
age[strcspn(age, "\n")] = 0;printf("My age is %s\n", age);

return 0;
}