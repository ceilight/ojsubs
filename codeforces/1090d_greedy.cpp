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

vector<int> vi[100100];
int res[100100];

int main() {
    int n, m;
    scanf("%d%d", &n, &m);
    for(int i = 0; i < m; i++) {
        int u, v;
        scanf("%d%d", &u, &v);
        --u, --v;
        vi[u].eb(v), vi[v].eb(u);
    }
    if(n == 1) puts("NO");
    else {
        int u, v;
        bool ok = false;
        for(int i = 0; i < n; i++) {
            vi[i].eb(i);
            sort(all(vi[i]));
            if(vi[i].front() != 0)
                u = i, v = 0, ok = true;
            for(int j = 0; j < sz(vi[i]) - 1; j++)
                if(vi[i][j + 1] != vi[i][j] + 1)
                    u = i, v = vi[i][j] + 1, ok = true;
            if(ok) break;
        }
        if(!ok) puts("NO");
        else {
            int cur = 3;
            res[u] = 1, res[v] = 2;
            for(int i = 0; i < n; i++) {
                if(i == u || i == v) continue;
                res[i] = cur++;
            }
            puts("YES");
            for(int i = 0; i < n; i++)
                printf("%d ", res[i]);
            res[v] = 1;
            puts("");
            for(int i = 0; i < n; i++)
                printf("%d ", res[i]);
        }
    }
    return 0;
}