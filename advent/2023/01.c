#include <stdio.h>
#include <ctype.h>
#include <string.h>

char s[255];

const char *arr[] = {"####", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

int main() {
    int ans = 0;
    while (fgets(s, sizeof s, stdin) != NULL && s[0] != '\n') {
        int nr = strlen(s);
        int x = 0;
        for (int i = 0; i < nr; i++) {
            for (int j = 0; j < 10; j++) {
                if (nr-i >= strlen(arr[j]) && !strncmp(s+i, arr[j], strlen(arr[j]))) {
                    x = j;
                    break;
                }
            }
            if (x != 0) break;
            else if (isdigit(s[i])) {
                x = (int)(s[i] - '0');
                break;
            }
        }
        x *= 10;
        for (int i = nr-1; i >= 0; i--) {
            for (int j = 0; j < 10; j++) {
                if (nr-i >= strlen(arr[j]) && !strncmp(s+i, arr[j], strlen(arr[j]))) {
                    x += j;
                    break;
                }
            }
            if (x % 10 != 0) break;
            else if (isdigit(s[i])) {
                x += (int)(s[i] - '0');
                break;
            }
        }
        ans += x;
    }
    printf("%d", ans);
}