// Sept. 21, 2021, 10:50 a.m. 
#include <iostream>
#include <algorithm>

using namespace std;
using llong = long long;

const int N = 100100;

string pat;
int pre[N << 2], suf[N << 2]; 

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s, t;
    cin >> s >> t;
    int len = s.size();
    pat += "#";
    pat += t; pat += s; pat += s; pat += t;
    int plen = pat.size() - 1;
    pre[1] = 0;
    int l = 0, r = 0;
    for (int i = 2; i <= plen; i++) {
        int u = i;
        if (u >= r) {
            while (u <= plen && pat[u] == pat[u-i+1])
                u++;
            pre[i] = u - i;
            if (u - 1 > r)
                l = i, r = u - 1;
        } else {
            int k = i - l + 1;
            if (pre[k] < r - i + 1)
                pre[i] = pre[k];
            else {
                u = max(u, r + 1);
                while (u <= plen && pat[u] == pat[u-i+1])
                    u++;
                pre[i] = u - i;
                if (u - 1 > r)
                    l = i, r = u - 1;
            }
        }
    }
    suf[plen] = 0;
    l = r = plen + 1;
    for (int i = plen - 1; i >= 1; i--) {
        int u = i;
        if (u <= l) {
            while (u > 0 && pat[u] == pat[plen-i+u])
                u--;
            suf[i] = i - u;
            if (u + 1 < l)
                l = u + 1, r = i;
        } else {
            int k = r - i + 1;
            k = plen - k + 1;
            if (suf[k] < i - l + 1)
                suf[i] = suf[k];
            else {
                u = min(u, l - 1);
                while (u > 0 && pat[u] == pat[plen-i+u])
                    u--;
                suf[i] = i - u;
                if (u + 1 < l)
                    l = u + 1, r = i;
            }
        }
    }
    llong res = 0;
    for (int i = 1; i <= len; i++) {
        int x = pre[i + len] + suf[i + len * 2 - 1];
        if (x >= len)
            res += len;
        else if (x == len - 1)
            res++;
    }
    cout << res << "\n";
    return 0;
}
