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
    int n, k;
    scanf("%d%d", &n, &k);
    int pre, cur;
    scanf("%d", &pre);
    int len = 1, mx = 1;
    for(int i = 1; i < n; i++) {
        scanf("%d", &cur);
        if(cur != pre) ++len;
        else {
            chkmax(mx, len);
            len = 1;
        }
        pre = cur; 
    }
    chkmax(mx, len);
    printf("%d", mx);
    return 0;
}
