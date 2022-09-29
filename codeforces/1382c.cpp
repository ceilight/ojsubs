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

char sa[100005], sb[100005];

int main() {
    int t;
    scanf("%d", &t);
    while(t--) {
        int n;
        scanf("%d", &n);
        vector<int> a, b;
        scanf("%s%s", sa, sb);
        sa[n] = sb[n] = '0';
        for(int i = 1; i <= n; i++) {
            if(sa[i] != sa[i - 1]) a.eb(i);
            if(sb[i] != sb[i - 1]) b.eb(i);
        }
        while(b.size()) {
            a.eb(b.back());
            b.pb();
        }
        printf("%d", a.size());
        for(auto x : a)
            printf(" %d", x);
        puts("");
    }
    return 0;
}