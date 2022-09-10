// https://oj.vnoi.info/problem/icpc21_mn_j

#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    
    int m, n;
    cin >> m >> n;

    if (m == 0 || n == 0) {
        cout << "-1\n";
        return 0;
    }

    vector<string> a(m);
    vector<vector<int>> f(m+2, vector<int>(n+2, 0));
    vector<vector<int>> temp(m+2, vector<int>(n+2, 0));


    for (int i = 0; i < m; i++) {
        cin >> a[i];
    }

    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            f[i][j] = 1e9;
        }
    }
    // v
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (a[i-1][j-1] == '1') {
                temp[i][j] = temp[i-1][j]+1;
            } else {
                temp[i][j] = 0;
            }
        }
    }
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            f[i][j] = min(f[i][j], temp[i][j]);
        }
    }
    // ^
    for (int i = m; i >= 1; i--) {
        for (int j = n; j >= 1; j--) {
            if (a[i-1][j-1] == '1') {
                temp[i][j] = temp[i+1][j]+1;
            } else {
                temp[i][j] = 0;
            }
        }
    }
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            f[i][j] = min(f[i][j], temp[i][j]);
        }
    }
    // >
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (a[i-1][j-1] == '1') {
                temp[i][j] = temp[i][j-1]+1;
            } else {
                temp[i][j] = 0;
            }
        }
    }
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            f[i][j] = min(f[i][j], temp[i][j]);
        }
    }
    // <
    for (int i = m; i >= 1; i--) {
        for (int j = n; j >= 1; j--) {
            if (a[i-1][j-1] == '1') {
                temp[i][j] = temp[i][j+1]+1;
            } else {
                temp[i][j] = 0;
            }
        }
    }
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            f[i][j] = min(f[i][j], temp[i][j]);
        }
    }
    int resx = -1, resy = -1;
    int mx = 0;
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (f[i][j] > mx) {
                mx = f[i][j], resx = i, resy = j;
            }
        }
    }
    if (mx == 0) {
        cout << "-1\n";
    } else {
        cout << (mx -1)*4 + 1 << '\n' << resx << ' ' << resy << '\n';
    }
}
