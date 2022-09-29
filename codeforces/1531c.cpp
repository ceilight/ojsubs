#include <bits/stdc++.h>

using namespace std;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    int n; cin >> n;
    if(n == 1) {
        cout << "1\no\n";
        return 0;
    }
    if(n == 2) {
        cout << "-1\n";
        return 0;
    }

    char ans[15][15];
    for(int i = 0; i < 15; i++) {
        for(int j = 0; j < 15; j++) {
            ans[i][j] = 'o';
        }
    }
    int a = 1;
    while(a * a < n) a++;
    int re = a * a - n;

    if(re == 2) {
        for(int i = 2; i <= a + 1; i++) {
            ans[1][i] = '.';
            ans[i - 1][a + 1] = '.';
        }
        ans[2][a - 1] = ans[2][a] = '.';
        ans[3][a - 1] = ans[3][a] = '.';
        cout << a + 1 << "\n";
        for(int i = 1; i <= a + 1; i++) {
            for(int j = 1; j <= a + 1; j++) {
                cout << ans[i][j];
            }
            cout << "\n";
        }
        return 0;
    }

    if(re % 2 == 1) {
        int row = 1;
        int col = a;
        ans[1][a] = '.';
        re--;
        while(re > 0) {
            ans[++row][a] = '.';
            ans[1][--col] = '.';
            re -= 2;
        }
    }
    else if(re > 2) {
        int row = 2;
        int col = a - 1;
        ans[1][a - 1] = ans[1][a] = '.';
        ans[2][a - 1] = ans[2][a] = '.';
        re -= 4;
        while(re > 0) {
            ans[++row][a] = '.';
            ans[1][--col] = '.';
            re -= 2;
        }
    }

    cout << a << "\n";
    for(int i = 1; i <= a; i++) {
        for(int j = 1; j <= a; j++) {
            cout << ans[i][j];
        }
        cout << "\n";
    }
    return 0;
}