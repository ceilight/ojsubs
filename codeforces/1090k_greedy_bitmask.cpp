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

map<pair<string, int>, vector<int>> tb;

int main() {
    int n;
    scanf("%d", &n);
    for(int i = 1; i <= n; i++) {
        string s, t;
        cin >> s >> t;
        int mask = 0;
        for(int i = 0; i < sz(t); i++)
            mask |= 1 << (t[i] - 'a');
        int r = -1;
        for(int i = sz(s) - 1; i >= 0; i--) {
            if((mask & (1 << (s[i] - 'a'))) == 0) {
                r = i; break;
            }
        }
        string st = s.substr(0, r + 1);
        tb[mp(st, mask)].eb(i);
    }
    printf("%d\n", tb.size());
    for(auto u : tb) {
        printf("%d", u.se.size());
        for(auto v : u.se)
            printf(" %d", v);
        puts("");
    }
    return 0;
}