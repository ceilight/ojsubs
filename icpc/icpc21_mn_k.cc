// Jan. 7, 2022, 7:40 a.m. 

#include <bits/stdc++.h>
using namespace std;

const int nmax = 100100;
int n, x[nmax];

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> n;
    for (int i = 0; i < n; i++) {
        cin >> x[i];
    }
    int y, z;
    cin >> y >> z;
    for (int i = 0; i < n; i++) {
        x[i] = abs(x[i]-z);
    }
    int ttl = abs(y-z);
    sort(x, x+n);
    int res = 0;
    for (int i = 0; i < n; i++) {
        if (ttl > x[i]) {
            ttl -= x[i];
            res++;
        } else break;
    }
    cout << res << '\n';
    return 0;
}
