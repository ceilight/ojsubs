// https://oj.vnoi.info/problem/chain2

#include <iostream>
#include <algorithm>
#include <cstring>
#include <string>

using namespace std;

struct trie {
    trie *child[26];
    bool isword;

    trie() {
        for (int i = 0; i < 26; i++)
            child[i] = nullptr;
        isword = false;
    }
} *root;

int insert(string s) {
    trie* cur = root;
    int cnt = 0;
    for (int i = 0; i < s.size(); i++) {
        int idx = s[i] - 'a';
        if (!cur->child[idx])
            cur->child[idx] = new trie();
        cur = cur->child[idx];
        if (cur->isword) cnt++;
    }
    cur->isword = true;
    return ++cnt;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    root = new trie();
    int n;
    cin >> n;
    int res = 0;
    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        int cnt = insert(s);
        res = max(res, cnt);
    }
    cout << res << "\n";
    return 0;
}
