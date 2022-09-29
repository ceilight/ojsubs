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

char a[100100], b[100100];

int main() {
    int t;
    scanf("%d", &t);
    while(t--) {
        scanf("%s%s", a, b);
        int sz = strlen(b), lo, hi;
        for(int i = 1; i <= sz; i++)
            if(b[sz - i] == '1') {
                lo = i; break;
            }
        sz = strlen(a);
        for(int i = lo; i <= sz; i++)
            if(a[sz - i] == '1') {
                hi = i; break;
            }
        printf("%d\n", hi - lo);
    }
    return 0;
}