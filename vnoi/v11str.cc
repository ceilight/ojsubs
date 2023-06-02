// July 11, 2021, 5:59 p.m. 
#include <bits/stdc++.h>

using namespace std;

const long long BASE = 1e9 + 7;
long long hp[500007], ht[500007], p[500007];

long long get_hash(long long* h, int l, int r) {
    if(l == 0) return h[r];
    return (BASE + h[r] - h[l - 1] * p[r - l + 1] % BASE) % BASE;
}

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    p[0] = 1;
    for(int i = 1; i < 500007; i++)
        p[i] = p[i - 1] * 65538 % BASE;

    int P, T; cin >> P >> T;
    int X;

    cin >> hp[0]; ++hp[0];
    for(int i = 1; i < P; i++) {
        cin >> X;
        hp[i] = (hp[i - 1] * 65538 + X + 1) % BASE;
    }

    cin >> ht[0]; ++ht[0];
    for(int i = 1; i < T; i++) {
        cin >> X;
        ht[i] = (ht[i - 1] * 65538 + X + 1) % BASE;
    }

    int ans = -1;
    for(int i = 0; i + P - 1 < T; i++) {
        if(ans != - 1) break;

        int l = 1, r = P;

        while(l <= r) {
            int m = (l + r) / 2;
            long long ppre = get_hash(hp, 0, m - 1);
            long long tpre = get_hash(ht, i, i + m - 1);
            if(ppre == tpre) l = m + 1; else r = m - 1;
        }

        if(r == P) ans = i + 1;
        else {
            long long psuf = get_hash(hp, r + 1, P - 1);
            long long tsuf = get_hash(ht, i + r + 1, i + P - 1);
            if(psuf == tsuf) ans = i + 1;
        }
    }

    cout << ans << "\n";

    return 0;
}
