#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define N 255
char s[N][N];
int dx[] = {-1, -1, -1, 0, 1, 1, 1, 0};
int dy[] = {-1, 0, 1, 1, 1, 0, -1, -1};
int n, m, sum;

int main(void) {
    while (fgets(s[n], N, stdin) != NULL && s[n][0] != '\n') n++;
    m = strlen(s[0]) - 1;
    for (int i = 0; i < n; i++) {
        int num_end = 1, part = 0, num = 0;
        for (int j = 0; j <= m; j++) {
            if (!num_end && (j == m || !isdigit(s[i][j]))) {
                if (part) sum += num;
                num = 0;
                num_end = 1;
                part = 0;
                continue;
            }
            if (isdigit(s[i][j])) {
                num_end = 0;
                num = num * 10 + s[i][j] - '0';

                for (int k = 0; k < 8; k++) {
                    int ii = dx[k] + i, jj = dy[k] + j;
                    if (ii >= 0 && ii < n && jj >= 0 && jj < m
                        && !isdigit(s[ii][jj]) && s[ii][jj] != '.') part = 1;
                }
            }
        }
    }
    printf("%d\n", sum);
}