// July 10, 2021, 7:42 a.m. 
#include <bits/stdc++.h>

using namespace std;

const long long BASE = 1e9 + 7;
long long hashst_a[1000007], hashst_b[1000007], p[1000007];

long long get_hash(long long* hashst, int i, int x) {
    if(i == 0) return hashst[x];
    return (BASE + hashst[x] - hashst[i - 1] * p[x - i + 1] % BASE) % BASE;
}

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    int SA, SB; cin >> SA >> SB;
    char A[SA], B[SB]; cin >> A >> B;

    p[0] = 1;
    for(int i = 1; i < 1000007; i++)
        p[i] = (p[i - 1] * 27) % BASE;

    hashst_a[0] = A[0] - 'a' + 1;
    for(int i = 1; i < SA; i++)
        hashst_a[i] = (hashst_a[i - 1] * 27 + A[i] - 'a' + 1) % BASE;

    hashst_b[0] = B[0] - 'a' + 1;
    for(int i = 1; i < SB; i++)
        hashst_b[i] = (hashst_b[i - 1] * 27 + B[i] - 'a' + 1) % BASE;

    int Q; cin >> Q;
    for(int q = 0; q < Q; q++) {
        int LA, RA, LB, RB; cin >> LA >> RA >> LB >> RB;
        --LA, --RA, --LB, --RB;

        // find first index with different char
        int lim = min(RA - LA + 1, RB - LB + 1);

        int l = 1, r = lim;
        while(l <= r) {
            int m = (l + r) / 2;
            long long suba = get_hash(hashst_a, LA, LA + m - 1);
            long long subb = get_hash(hashst_b, LB, LB + m - 1);
            if(suba == subb) l = m + 1; else r = m - 1;
        }

        if(r == lim) { // none found, compare by substring's size
            if(RA - LA == RB - LB) cout << "=";
            else if(RA - LA > RB - LB) cout << ">"; else cout << "<";
        }
        else {
            if(A[LA + r] > B[LB + r]) cout << ">"; else cout << "<";
        }
    }

    return 0;
}
