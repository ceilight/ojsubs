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

char s[1000100];
unordered_map<int, int> tb;
vector<int> mask;

int main() {
    int n;
    scanf("%d", &n);
    for(int i = 0; i < n; i++) {
        scanf("%s", s);
        int msk = 0;
        for(int j = 0; s[j] != '\0'; j++)
            msk ^= 1 << (s[j] - 'a');
        tb[msk]++;
        mask.eb(msk);
    }
    i64 res = 0;
    for(int i = 0; i < n; i++) {
        int msk = mask[i];
        res += (i64)(tb[msk] - 1);
        for(int j = 0; j < 26; j++) {
            res += (i64)tb[msk ^ (1 << j)];
        }
    }
    printf("%lld\n", res / 2);
    return 0;
}