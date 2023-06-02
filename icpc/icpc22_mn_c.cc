// Jan. 6, 2022, 6:59 p.m. 

#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s; cin >> s;
    int n; cin >> n;

    int fi = 0, se = 1;
    for (int i = 1; i < (int)s.size(); i++) {
        if (s[i] < s[fi] || (s[i] == s[fi] && (s[i-1] < s[se] || s[i+1] < s[se]))) {
            fi = i;
            if (i == s.size()-1 || s[i-1] <= s[i+1]) {
                se = i-1;
            } else {
                se = i+1;
            }
        }
    }
    for (int i = 0; i < n; i++) {
        cout << (i & 1 ? s[se] : s[fi]);
    }
}
