#include <bits/stdc++.h>

#define i64 long long
#define fi first
#define se second
#define eb emplace_back
#define pb pop_back
#define mp make_pair
#define all(a) (a).begin(), (a).end()
#define sz(a) (a).size()
#define reset(a, b) memset(a, b, sizeof(a))
template<typename T> bool chkmax(T &a, T b) {return a < b ? a = b, 1 : 0;}
template<typename T> bool chkmin(T &a, T b) {return a > b ? a = b, 1 : 0;}
using namespace std;

const int inf = 1000001;
char s[2000100];
int f[12][12][12][12];

int main() {
    scanf("%s", s);
    for(int i = 0; i < 10; i++)
        for(int j = 0; j < 10; j++)
            for(int u = 0; u < 10; u++)
                for(int v = 0; v < 10; v++)
                    f[i][j][u][v] = inf;

    for(int i = 0; i < 10; i++)
        for(int j = 0; j < 10; j++) {
            for(int k = 0; k < 10; k++)
                f[i][j][k][(k + i) % 10] = f[i][j][k][(k + j) % 10] = 1;
            for(int k = 0; k < 10; k++)
                for(int u = 0; u < 10; u++)
                    for(int v = 0; v < 10; v++)
                        chkmin(f[i][j][u][v], f[i][j][k][v] + f[i][j][u][k]);
        }

    for(int i = 0; i < 10; i++) {
        for(int j = 0; j < 10; j++) {
            bool ok = true;
            int tot = 0;
            for(int k = 1; s[k] != '\0'; k++) {
                int u = s[k - 1] - '0', v = s[k] - '0';
                int dis = f[i][j][u][v];
                if(dis >= inf) {
                    ok = false; break;
                }
                tot += dis - 1;
            }
            printf("%d ", (ok ? tot : -1));
        }
        puts("");
    }
    return 0;
}