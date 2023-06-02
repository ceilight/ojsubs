// July 7, 2021, 11:10 a.m. 
#include <bits/stdc++.h>

using namespace std;

const long long LIM = 1e18;
const int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41};

long long solve(int n) {
    vector<long long> f(n + 1, LIM);
    f[1] = 1;

    for(int p : primes) {
        for(int i = n; i >= 1; i--) {
            long long div = p * 1LL;
            for(int j = i * 2; j <= n; j += i) {
                long long mult = f[i] * div;
                if(mult > LIM || mult < f[i])
                    mult = LIM;
                f[j] = min(f[j], mult);

                //div = div * 1LL * p;
                if(div * 1LL * p > LIM || div * 1LL * p < div) break;
                div *= 1LL * p;
            }
        }
    }
    return f[n];
}

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    int N; cin >> N;
    cout << solve(N);
    return 0;
}
