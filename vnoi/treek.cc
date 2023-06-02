// June 6, 2021, 2:40 p.m. 
#include <bits/stdc++.h>

using namespace std;
using ll = long long;

ll f[503][1007];

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    int n, k; cin >> n >> k;
    if(n % 2 == 0) {
        cout << "0\n";
        return 0;
    }
    for(int i = 1; i <= k; i++) {
        for(int j = 1; j <= n; j++) {
            f[i][j] = -1;
        }
    }
    for(int i = 1; i <= k; i++) {
        f[i][1] = 1;
    }
    for(int i = 1; i <= k; i++) {
        for(int j = 1; j <= n; j++) {
            if(f[i][j] == -1) {
                f[i][j] = 0;
                if(j % 2 != 0) {
                    for(int t = 1; t < j; t += 2) {
                        f[i][j] = (f[i][j] + f[i - 1][j - t - 1] * f[i - 1][t] % 9901) % 9901;
                    }
                }
            }
        }
    }
    cout << (9901 + f[k][n] - f[k - 1][n]) % 9901 << "\n";
    return 0;
}
