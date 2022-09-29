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

int a[100005], cnt[100005], app[100005];

int main() {
    int n;
    scanf("%d", &n);
    for(int i = 1; i <= n; i++)
        scanf("%d", &a[i]);
    int pos = 100005, len = 0, mx = 0;
    for(int i = 1; i <= n; i++) {
        app[cnt[a[i]]]--;
        cnt[a[i]]++, app[cnt[a[i]]]++;
        chkmax(mx, cnt[a[i]]);
        if((app[i] == 1)
            || (app[1] == i) 
            || (app[mx] == 1 && app[mx - 1] * (mx - 1) == i - mx)
            || (app[1] == 1 && app[mx] * mx == i - 1)
            ) len = i;
    }
    printf("%d\n", len);
    return 0;
}