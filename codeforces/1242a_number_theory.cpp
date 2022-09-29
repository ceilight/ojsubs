#include <bits/stdc++.h>

#define fi first
#define se second
#define eb emplace_back
#define pb pop_back
#define mp make_pair
#define all(a) (a).begin(), (a).end()
#define reset(a, b) memset(a, b, sizeof(a))

template<typename T> bool chkmax(T &a, T b) { return a < b ? a = b, 1 : 0; }
template<typename T> bool chkmin(T &a, T b) { return a > b ? a = b, 1 : 0; }
using llong = long long;
using namespace std;

int main() {
    llong n;
    scanf("%lld", &n);
    vector<llong> l;
    for(llong i = 2; i * i <= n; i++)
        if(n % i == 0) {
            l.eb(i);
            if(i * i < n) l.eb(n / i);
        }
    llong ans = n;
    for(int i = 0; i < l.size(); i++)
        ans = __gcd(ans, l[i]);
    printf("%lld\n", ans);
    return 0;
}