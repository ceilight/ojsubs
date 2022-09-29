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

int main() {
    int n, m, k;
    scanf("%d%d%d", &n, &m, &k);
    for(int i = 0; i + i < k; i++)
        scanf("%*d%*d");
    vector<char> res;
    for(int i = 0; i < m - 1; i++)
        res.eb('R');
    for(int i = 0; i < n - 1; i++)
        res.eb('D');
    for(int i = 0; i < n; i++) {
        for(int j = 0; j < m - 1; j++)
            res.eb(i % 2 ? 'R' : 'L');
        if(i != n - 1) res.eb('U');
    }
    printf("%d\n", sz(res));
    for(auto c : res)
        printf("%c", c);
    return 0;
}