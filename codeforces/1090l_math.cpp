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
    int t, n, a, b, k;
    scanf("%d%d%d%d%d", &t, &n, &a, &b, &k);
    int na = n - n / 2, nb = n / 2;
    if(a > b) {
        swap(a, b);
        swap(na, nb);
    }
    int lo = 0, hi = t + 1;
    while(lo < hi) {
        int mid = (lo + hi) / 2;
        i64 cnt = 1LL * min(mid, a) * na + 1LL * min(mid, b) * nb;
        if(mid == 0 || cnt / mid >= 1LL * k)
            lo = mid + 1;
        else
            hi = mid;
    }
    printf("%d", lo - 1);
    return 0;
}