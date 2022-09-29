#include <bits/stdc++.h>

#define debug(aa...) fprintf(stderr, aa)
#define fi first
#define se second
#define eb emplace_back
#define pb pop_back
#define mp make_pair
#define all(a) a.begin(), a.end()
#define reset(a, b) memset(a, b, sizeof(a))
template<typename T> bool chkmax(T &a, T b) {return a < b ? a = b, 1 : 0;}
template<typename T> bool chkmin(T &a, T b) {return a > b ? a = b, 1 : 0;}

using namespace std;

int main() {
    int n, m, k;
    scanf("%d%d%d", &n, &m, &k);
    if(n * m * 4 - n * 2 - m * 2 < k) puts("NO");
    else {
        int cur = 0;
        vector<pair<char, int>> res;
        auto move = [&](char dir, int step) -> bool {
            if(step == 0) return false;
            if(cur >= k) return true;
            if(cur + step <= k) 
                res.eb(mp(dir, step));
            else
                res.eb(mp(dir, k - cur));
            cur += step;
            return cur >= k;
        };
        for(int i = 0; i < n - 1; i++) {
            if(move('R', m - 1)) break;
            if(move('L', m - 1)) break;
            if(move('D', 1)) break;
        }
        for(int i = 0; i < m - 1; i++) {
            if(move('R', 1)) break;
            if(move('U', n - 1)) break;
            if(move('D', n - 1)) break;
        }
        if(!move('L', m - 1)) move('U', n - 1);
        printf("YES\n%d\n", res.size());
        for(auto x : res)
            printf("%d %c\n", x.se, x.fi);
    }
    return 0;
}

// rld rud lu