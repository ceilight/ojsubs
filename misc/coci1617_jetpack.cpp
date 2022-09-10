// https://oj.vnoi.info/problem/coci1617_r1_jetpack

#include <cstdio>
#include <vector>
#include <algorithm>

using namespace std;
using ll = long long;

const int N = 100100;

char a[11][N+2];
vector<int> hold;
int n;

bool back(int u, int v) {
    if (a[u][v] == 'X') return false;
    if (v == n) return true;

    a[u][v] = 'X';
    
    // not hold, goes downward
    if (back(min(10,u+1), v+1)) return true;

    // hold, goes upward
    if (back(max(1,u-1), v+1)) {
        hold.push_back(v-1);
        return true;
    }
    return false;
}

int main() {
    scanf("%d", &n);
    for (int i = 1; i <= 10; i++) {
        scanf("%s", a[i]+1);
    }
    
    back(10, 1);
    // printf("%d\n", t.size());
    int sz = (int)hold.size();
    int len = 1, st = hold[sz-1];

    vector<int> res1, res2;
    for (int i = sz-2; i >= 0; i--) {
        if (hold[i] - hold[i+1] == 1) {
            len++;
        } else {
            res1.push_back(st);
            res2.push_back(len);
            st = hold[i], len = 1;
        }
    }
    res1.push_back(st);
    res2.push_back(len);
    
    sz = (int)res1.size();
    printf("%d\n", sz);
    for (int i = 0; i < sz; i++) {
        printf("%d %d\n", res1[i], res2[i]);
    }

    return 0;
}
