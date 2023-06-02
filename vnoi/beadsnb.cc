// July 18, 2021, 9:35 a.m. 
#include <bits/stdc++.h>

using namespace std;

const int inf = 1e9 + 7;
int N, a[100007], res;

vector<int> lis(int* a) {
    vector<int> res, last(N + 1, inf);
    last[0] = -inf;

    for(int i = N - 1; i >= 0; i--) {
        int d = lower_bound(last.begin(), last.end(), a[i]) - last.begin();
        last[d] = a[i];
        res.push_back(d);
    }
    return res;
}

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    cin >> N;
    for(int i = 0; i < N; i++) cin >> a[i];
    
    vector<int> f = lis(a);
    for(int i = 0; i < N; i++) a[i] *= -1;
    vector<int> nf = lis(a);
    
    for(int i = 0; i < N; i++)
        res = max(res, f[i] + nf[i] - 1);
    cout << res << "\n";

    return 0;
}
