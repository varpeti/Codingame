#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <float.h>
#include <math.h>

struct Defib {
    int id;
    char *name;
    char *address;
    char *phoneNumber;
    double longitude;
    double latitude;
};

struct Person {
    double longitude;
    double latitude;
} myPerson;

double parseDouble(char *str)
{
    for (int i = 0; i < strlen(str); i++) 
    {
        if (str[i] == ',') {
            str[i] = '.';
            break;
        }
    }
    return atof(str);
}

struct Defib parse(char *str)
{
    struct Defib myDefib;
    char delim[] = ";";

	myDefib.id = atoi(strsep(&str, delim));
    myDefib.name = strsep(&str, delim);
    myDefib.address = strsep(&str, delim);
    myDefib.phoneNumber = strsep(&str, delim);
    myDefib.longitude = parseDouble(strsep(&str, delim));
    myDefib.latitude = parseDouble(strsep(&str, delim));
	
    return myDefib;
}

double calcDistance(struct Person myPerson, struct Defib myDefib)
{
    double x = (myDefib.longitude - myPerson.longitude) * cos( (myDefib.latitude + myPerson.latitude)/2 );
    double y = myDefib.latitude - myPerson.latitude;
    return sqrt(pow(x, 2) + pow(y, 2)) * 6371;
}

int main()
{
    char tmp[51];
    scanf("%s", tmp);
    myPerson.longitude = parseDouble(tmp);
    scanf("%s", tmp);
    myPerson.latitude = parseDouble(tmp);
    
    double minDistance = DBL_MAX;
    char closestDefib[257];
    int N;
    scanf("%d", &N); 
    fgetc(stdin); // trash \n
    fprintf(stderr, "%i %lf %lf\n", N, myPerson.longitude, myPerson.latitude);
    for (int i = 0; i < N; i++) 
    {
        char DEFIB[257];
        scanf("%[^\n]", DEFIB); 
        fgetc(stdin); // trash \n
        struct Defib curDefib = parse(DEFIB);
        double curDistance = calcDistance(myPerson, curDefib);
        if (curDistance < minDistance)
        {
            minDistance = curDistance;
            strcpy(closestDefib, curDefib.name);
            fprintf(stderr, "%s\n", closestDefib);
        }
    }
    
    printf("%s\n", closestDefib);
    return 0;
}
