#include <bits/stdc++.h>
// #include "dna.h"

int f[100005][3][3];

int get_pair_cnt(int ca, int cb, int l, int r) {
    return f[r + 1][ca][cb] - f[l][ca][cb];
}

int ord(char c) {
    return c == 'A' ? 0 : (c == 'T' ? 1 : 2);
}

void init(std::string a, std::string b) {
    int sz = a.size(), orda, ordb;

    for(int k = 1; k <= sz; k++) {
        orda = ord(a[k - 1]);
        ordb = ord(b[k - 1]);
        for(int i = 0; i < 3; i++) {
            for(int j = 0; j < 3; j++) {
                f[k][i][j] += f[k - 1][i][j];
            }
        }
        if(orda != ordb) f[k][orda][ordb]++;
    } 
}

int get_distance(int x, int y) {
    int res = 0, charbal[3] = {};

    for(int i = 0; i < 3; i++) {
        for(int j = 0; j < 3; j++) {
            int pairs = get_pair_cnt(i, j, x, y);
            charbal[i] += pairs;
            charbal[j] -= pairs;
        }
    }

    for(int i = 0; i < 3; i++) {
        if(charbal[i] != 0) return -1;
    }

    int tot = 0;
    for(int i = 0; i < 3; i++) {
        for(int j = 0; j < 3; j++) {
            tot += get_pair_cnt(i, j, x, y);
        }
    }
    for(int i = 0; i < 3; i++) {
        for(int j = i + 1; j < 3; j++) {
            int cm = std::min(get_pair_cnt(i, j, x, y), get_pair_cnt(j, i, x, y));
            tot -= cm * 2;
            res += cm;
        }
    }

    res += (tot / 3) * 2;

    return res;
}
