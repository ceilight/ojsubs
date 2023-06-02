// July 9, 2021, 4:50 p.m. 
#include <bits/stdc++.h>

using namespace std;

const long long BASE = 1e9 + 7;
long long hashst[50007], p[50007];
int N, K;

long long get_hash(int i, int x) {
    if(i == 0) return hashst[x];
    return (BASE + hashst[x] - hashst[i - 1] * p[x - i + 1] % BASE) % BASE;
}

bool check(int len) {
    vector<long long> st;
    for(int i = 0; i + len - 1 < N; i++)
        st.push_back(get_hash(i, i + len - 1));

    sort(st.begin(), st.end());
    int res = 1;
    for(int i = 1; i + len - 1 < N; i++) {
        if(st[i] == st[i - 1]) res++; else res = 1;
        if(res >= K) return true;
    }
    return false;
}

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    cin >> N >> K;
    char S[N]; cin >> S;

    if(K == 1) {
        cout << N << "\n";
        return 0;
    }

    p[0] = 1;
    for(int i = 1; i <= N; i++)
        p[i] = (p[i - 1] * 27) % BASE;

    hashst[0] = S[0] - 'a' + 1;
    for(int i = 1; i < N; i++)
        hashst[i] = (hashst[i - 1] * 27 + S[i] - 'a' + 1) % BASE;

    int l = 1, r = N - K + 1;
    while(l <= r) {
        int m = (l + r) / 2;
        if(check(m)) l = m + 1; else r = m - 1;
    }
    
    cout << r << "\n";

    return 0;
}
