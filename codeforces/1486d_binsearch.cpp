#include <bits/stdc++.h>

#define debug(aa...) fprintf(stderr, aa)
#define fi first
#define se second
#define pb push_back
#define mp make_pair
#define all(a) a.begin(), a.end()
#define reset(a, b) memset(a, b, sizeof(a))
template<typename T> bool chkmax(T &a, T b) {return a < b ? a = b, 1 : 0;}
template<typename T> bool chkmin(T &a, T b) {return a > b ? a = b, 1 : 0;}

using namespace std;

int a[200005], f[200005];

int main() {
    int n, k;
    scanf("%d%d", &n, &k);
    for(int i = 0; i < n; i++)
        scanf("%d", &a[i]);
    int lo = 1, hi = n;
    while(lo < hi) {
        int mid = (lo + hi + 1) / 2;
        reset(f, 0);
        for(int i = 1; i <= n; i++) {
            f[i] = f[i - 1];
            if(a[i - 1] >= mid) ++f[i]; else --f[i];
        }
        int res = 0;
        bool ok = false;
        for(int i = k; i <= n; i++) {
            chkmin(res, f[i - k]);
            if(res < f[i]) {
                ok = true; break;
            }
        }
        if(ok) lo = mid;
        else {
            chkmin(mid, hi - 1);
            hi = mid;
        }
    }
    printf("%d", lo);
    return 0;
}