// July 16, 2021, 10:05 a.m. 
#include <bits/stdc++.h>

using namespace std;
using lint = long long;

const lint mod = 1e9 + 7;

struct node {
    lint val[7];
    int lazy;
};

lint N, M, it[400009], a[100009], temp[8];
node t[400009];

void build2(int i, int l, int r) {
    if(l == r) {
        it[i] = 1; return;
    }

    int m = (l + r) / 2;
    build2(i * 2 + 1, l, m);
    build2(i * 2 + 2, m + 1, r);
    it[i] = it[i * 2 + 1] + it[i * 2 + 2];
}

void update2(int i, int l, int r, int p, lint v) {
    if(p < l || r < p) return;
    if(l == r) {
        it[i] = v; return;
    }

    int m = (l + r) / 2;
    update2(i * 2 + 1, l, m, p, v);
    update2(i * 2 + 2, m + 1, r, p, v);
    it[i] = it[i * 2 + 1] + it[i * 2 + 2];
}

int get(int i, int l, int r, lint k) {
    if(l == r) return l;
    int m = (l + r) / 2;
    if(it[i * 2 + 1] >= k)
        return get(i * 2 + 1, l, m, k);
    else
        return get(i * 2 + 2, m + 1, r, k - it[i * 2 + 1]); 
} 

void build(int i, int l, int r) {
    if(l == r) {
        for(int j = 0; j < 7; j++) t[i].val[j] = 1;
        t[i].val[l % 7] = a[l];
        return;
    }

    int m = (l + r) / 2;
    build(i * 2 + 1, l, m);
    build(i * 2 + 2, m + 1, r);

    for(int j = 0; j < 7; j++)
        t[i].val[j] = t[i * 2 + 1].val[j] * t[i * 2 + 2].val[j] % mod;
}

void push(int i) {
    int p = t[i].lazy; t[i].lazy = 0;

    for(int j = 0; j < 7; j++) temp[j] = t[i * 2 + 1].val[j];
    for(int j = 0; j < 7; j++) t[i * 2 + 1].val[j] = temp[(j + p) % 7];

    for(int j = 0; j < 7; j++) temp[j] = t[i * 2 + 2].val[j];
    for(int j = 0; j < 7; j++) t[i * 2 + 2].val[j] = temp[(j + p) % 7];

    t[i * 2 + 1].lazy += p, t[i * 2 + 2].lazy += p;
}

void update(int i, int l, int r, int p, int pp, lint v) {
    if(p < l || r < p) return;
    if(l == r) {
        for(int j = 0; j < 7; j++) t[i].val[j] = 1;
        t[i].val[pp % 7] = v;
        return;
    }

    if(t[i].lazy > 0) push(i);
    int m = (l + r) / 2;
    update(i * 2 + 1, l, m, p, pp, v);
    update(i * 2 + 2, m + 1, r, p, pp, v);

    for(int j = 0; j < 7; j++)
        t[i].val[j] = (t[i * 2 + 1].val[j] * t[i * 2 + 2].val[j]) % mod;       
}

void shift(int i, int l, int r, int L, int R) {
    if(L > R || l > r) return;
    if(l > R || r < L) return;
    if(L <= l && r <= R) {
        int temp = t[i].val[0];
        for(int j = 0; j < 6; j++) t[i].val[j] = t[i].val[j + 1];
        t[i].val[6] = temp;

        ++t[i].lazy;
        return;
    }

    if(t[i].lazy > 0) push(i);
    int m = (l + r) / 2;
    shift(i * 2 + 1, l, m, L, R);
    shift(i * 2 + 2, m + 1, r, L, R);

    for(int j = 0; j < 7; j++)
        t[i].val[j] = t[i * 2 + 1].val[j] * t[i * 2 + 2].val[j] % mod;
}

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    cin >> N >> M;
    for(int i = 0; i < N; i++) cin >> a[i];
    build(0, 0, N - 1);
    build2(0, 0, N - 1);

    int type, p;
    lint val, v;
    for(int q = 0; q < M; q++) {
        cin >> type;
        if(type == 1) {
            cin >> p;
            v = get(0, 0, N - 1, (lint)(p + 1));
            update2(0, 0, N - 1, v, 0);
            update(0, 0, N - 1, v, p, 1);
            shift(0, 0, N - 1, v, N - 1);
        }
        else if(type == 2) {
            cin >> p >> val;
            v = get(0, 0, N - 1, (lint)(p + 1));
            update(0, 0, N - 1, v, p, val);
        }
        else {
            cin >> p;
            cout << t[0].val[p % 7] << "\n";
        }
    }

    return 0;
}
