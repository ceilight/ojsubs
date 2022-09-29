#include <bits/stdc++.h>

using namespace std;
using lint = long long;

string a, b;
lint c, k, d, res;

lint modexp(lint a, lint b, lint m) {
    lint ans = 1;
    while(b > 0) {
        if(b % 2) ans = ans * a % m;
        a = a * a % m, b /= 2;
    }
    return ans;
}

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    cin >> a >> b >> c;

    k = 0;
    for(auto x : a) {
        d = x - '0';
        k = (k * 10 + d) % c;
    }
    res = 1;
    for(auto x : b) {
        d = x - '0';
        res = modexp(res, 10, c);
        res = res * modexp(k, d, c) % c;
    }

    cout << res << "\n";

    return 0;
}
