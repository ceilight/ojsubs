// Jan. 7, 2022, 8:52 a.m. 

#include <bits/stdc++.h>
using namespace std;

const int nmax = 100100;
int n;
map<int, int> same_x, same_y;
map<pair<int, int>, int> same_xy;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> n;
    long long cnt = 0;
    for (int i = 0; i < n; i++) {
        int x, y;
        cin >> x >> y;
        cnt += same_x[x] + same_y[y] - same_xy[{x, y}];
        same_x[x]++;
        same_y[y]++;
        same_xy[{x, y}]++;
    }
    cout << cnt << '\n';
}
