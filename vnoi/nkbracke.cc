// July 6, 2021, 6:22 p.m. 
#include <bits/stdc++.h>

using namespace std;

struct node {
    int open, close;
    node() { open = 0, close = 0; };
};

node combine(node l, node r) {
    node res;
    int temp = min(l.open, r.close);
    res.open = l.open + r.open - temp;
    res.close = l.close + r.close - temp;
    return res;
}

struct segment {
    vector<node> T;

    void build(char* a, int pos, int l, int r) {
        if(l > r) return;
        if(l == r) {
            if(a[l] == '(') T[pos].open = 1;
            else T[pos].close = 1;
            return;
        }

        int m = (l + r) / 2;
        build(a, pos * 2 + 1, l, m);
        build(a, pos * 2 + 2, m + 1, r);
        T[pos] = combine(T[pos * 2 + 1], T[pos * 2 + 2]);
    }

    segment(char* a, int n) {
        T.resize(n * 4);
        fill(T.begin(), T.end(), node());
        build(a, 0, 0, n - 1);
    }

    void update(int pos, int l, int r, int P, char C) {
        if(l > P || r < P) return;
        if(l == P && r == P) {
            if(C == '(')
                T[pos].open = 1, T[pos].close = 0;
            else 
                T[pos].open = 0, T[pos].close = 1;
            return;         
        }
        
        int m = (l + r) / 2;
        update(pos * 2 + 1, l, m, P, C);
        update(pos * 2 + 2, m + 1, r, P, C);
        T[pos] = combine(T[pos * 2 + 1], T[pos * 2 + 2]);         
    }

    node get(int pos, int l, int r, int L, int R) {
        if(L > R || l > r) return node();
        if(L > r || R < l) return node();
        if(L <= l && r <= R) return T[pos];

        int m = (l + r) / 2;
        node LN = get(pos * 2 + 1, l, m, L, R);
        node RN = get(pos * 2 + 2, m + 1, r, L, R);
        return combine(LN, RN);
    }
};


int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    int N, M; cin >> N >> M;
    char a[N]; cin >> a;

    segment tr = segment(a, N);

    for(int i = 0; i < M; i++) {
        int Q; cin >> Q;
        if(Q == 0) {
            int P; char C; cin >> P >> C;
            tr.update(0, 0, N - 1, --P, C);
        }
        else {
            int L, R; cin >> L >> R;
            node sub = tr.get(0, 0, N - 1, --L, --R);
            if(sub.open == 0 && sub.close == 0)
                cout << "1";
            else
                cout << "0";
        }
    }
    return 0;
}
