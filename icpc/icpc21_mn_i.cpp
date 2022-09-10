// https://oj.vnoi.info/problem/icpc21_mn_i

#include <bits/stdc++.h>
using namespace std;

struct fenwick {
    vector<long long> bit;
    fenwick(int n): bit(n, 0) {} 
    
    void update(int u, int v) {
        for(; u < bit.size(); u += u & (-u)) {
            bit[u] += v;
        }
    }
    long long get(int u) {
        long long ret = 0;
        for(; u; u -= u & (-u)) {
            ret += bit[u];
        }
        return ret;
    }
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;

    vector<int> a(n+1), p(n+1);
    for (int i = 1; i <= n; i++) {
        cin >> a[i];
        p[a[i]] = i;
    }
    fenwick tr(n+1);

    vector<long long> pre(n+2);
    for (int i = 1; i <= n; i++) {
        pre[i] = pre[i-1] + tr.get(n) - tr.get(p[i]);
        tr.update(p[i], 1);
    }
    for (int i = 1; i <= n; i++) {
        tr.update(p[i], -1);
    }
    
    vector<long long> suf(n+2);
    for (int i = n; i >= 1; i--) {
        suf[i] = suf[i+1] + tr.get(p[i]);
        tr.update(p[i], 1);
    }
    long long res = 1e12;
    for (int i = 1; i <= n; i++) {
        res = min(res, pre[i-1] + suf[i+1]);
    }
    cout << res << '\n';
}
