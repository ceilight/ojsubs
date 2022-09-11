#include <iostream>
#include <cassert>

using namespace std;
using llong = long long;

const int N = 100100;

llong pre[N], suf[N];
llong a[N];

struct node {
    int cnt;
    node* child[2];
    node() {
        cnt = 0;
        child[0] = child[1] = nullptr;
    }
} *root;

void insert(llong x) {
    node* cur = root;
    for(int i = 48; i >= 0; i--) {
        int bit = (x >> i) & 1;
        if(!(cur->child[bit])) {
            cur->child[bit] = new node();
            assert(cur->child);
        }
        cur->child[bit]->cnt++;
        cur = cur->child[bit];
    }
}

// void remove(int x) {
//     node* cur = root;
//     for(int i = 48; i >= 0; i--) {
//         int bit = (x >> i) & 1;
//         if(!(cur->child[bit]) || !(cur->child[bit]->cnt))
//             return;
//         cur->child[bit]->cnt--;
//         cur = cur->child[bit];
//     }
// }

llong get(llong x) {
    node* cur = root;
    llong ret = 0LL;
    for(int i = 48; i >= 0; i--) {
        int bit = !((x >> i) & 1);
        if(!(cur->child[bit]) || !(cur->child[bit]->cnt)) {
            cur = cur->child[!bit];
            if(!cur || !(cur->cnt))
                break;
        } else {
            ret += 1LL << i;
            cur = cur->child[bit];
        }
    }
    return ret;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    root = new node();
    assert(root);
    insert(0LL);
    int n;
    cin >> n;
    for(int i = 1; i <= n; i++)
        cin >> a[i];
    for(int i = 1; i <= n; i++)
        pre[i] = pre[i - 1] ^ a[i];
    for(int i = n; i >= 1; i--)
        suf[i] = suf[i + 1] ^ a[i];
    llong ans = -1;
    for(int i = n; i >= 0; i--) {
        insert(suf[i]);
        ans = max(ans, get(pre[i]));
    }
    cout << ans << "\n";
    return 0;
}