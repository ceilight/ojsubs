#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;

    vector<int> a(n);
    map<int, int> odd, even;
    long long tot = 0;
    for (int i = 0; i < n; i++) {
        cin >> a[i];
        if (a[i] & 1) {
            odd[a[i]]++;
        } else {
            even[a[i]]++;
        }
        tot += a[i];
    }
    string s;
    cin >> s;
    for (char c : s) {
        if (c == '0') {
            map<int, int> temp;
            for (auto v : even) {
                int val = v.first / 2;
                int cnt = v.second;
                if (val & 1) {
                    odd[val] += cnt;
                } else {
                    temp[val] += cnt;
                }
                tot -= 1LL * val * cnt;
            }
            even.clear();
            for (auto v : temp) {
                even[v.first] = v.second;
            }
        } else {
            map<int, int> temp;
            for (auto v : odd) {
                int val = v.first - 1;
                int cnt = v.second;
                if (val & 1) {
                    temp[val] += cnt;
                } else {
                    even[val] += cnt;
                }
                tot -= 1LL * cnt;
            }
            odd.clear();
            for (auto v : temp) {
                odd[v.first] = v.second;
            }
        }
        cout << tot << '\n';
    }
}
