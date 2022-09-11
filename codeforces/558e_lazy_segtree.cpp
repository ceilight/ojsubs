#include <iostream>
#include <cstring>
#include <cassert>

using namespace std;

const int N = 100100;
char s[N];

struct node {
    int cnt[26];
    int lazy;
    node() { lazy = -1; }
} seg[N << 2], nil;

void assign(int i, int l, int r, int val) {
    memset(seg[i].cnt, 0, 4 * 26);
    seg[i].cnt[val] = r - l + 1;
    seg[i].lazy = val;
}

node combine(node a, node b) {
    node ret;
    for(int i = 0; i < 26; i++)
        ret.cnt[i] = a.cnt[i] + b.cnt[i];
    return ret;
}

void push(int i, int l, int r) {
    if(seg[i].lazy == -1) return;
    int x = seg[i].lazy;
    int m = l + r >> 1;
    assign(i << 1, l, m, x);
    assign(i << 1 | 1, m + 1, r, x);
    seg[i].lazy = -1;
}

void build(int i, int l, int r) {
    if(l > r) return;
    if(l == r) {
        seg[i].cnt[s[l] - 'a']++;
        return;
    }
    int m = l + r >> 1;
    build(i << 1, l, m);
    build(i << 1 | 1, m + 1, r);
    seg[i] = combine(seg[i << 1], seg[i << 1 | 1]);
}

void update(int i, int l, int r, int u, int v, int val) {
    if(l > r || l > v || r < u) return;
    if(u <= l && r <= v) {
        assign(i, l, r, val);
        return;
    }
    push(i, l, r);
    int m = l + r >> 1;
    update(i << 1, l, m, u, v, val);
    update(i << 1 | 1, m + 1, r, u, v, val);
    seg[i] = combine(seg[i << 1], seg[i << 1 | 1]);
}

node get(int i, int l, int r, int u, int v) {
    if(l > r || l > v || r < u) return nil;
    if(u <= l && r <= v) return seg[i];
    push(i, l, r);
    int m = l + r >> 1;
    return combine(get(i << 1, l, m, u, v),
                   get(i << 1 | 1, m + 1, r, u, v));
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, q;
    cin >> n >> q;
    cin >> s;
    build(1, 0, n - 1);

    while(q--) {
        int i, j, k;
        cin >> i >> j >> k;
        --i, --j;
        int l = i, r;
        node it = get(1, 0, n - 1, i, j);
        if(k == 1) {
            for(int c = 0; c < 26; c++) {
                r = l + it.cnt[c] - 1;
                update(1, 0, n - 1, l, r, c);
                l += it.cnt[c];
            }
        } else {
            for(int c = 25; c >= 0; c--) {
                r = l + it.cnt[c] - 1;
                update(1, 0, n - 1, l, r, c);
                l += it.cnt[c];
            }
        }
        assert(r == j);
    }
    for(int i = 0; i < n; i++) {
        node tmp = get(1, 0, n - 1, i, i);
        for(int j = 0; j < 26; j++)
            if(tmp.cnt[j]) {
                s[i] = j + 'a';
                break;
            }
    }
    cout << s << "\n";
    return 0;
}