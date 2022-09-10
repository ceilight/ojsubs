// https://oj.vnoi.info/problem/hsgso20_baba

#include <stdio.h>
#include <string.h>
#include <stdbool.h>

int tot, n, p, mp[33], lo[33], hi[33];
char res[1000007], s[1000007], tmp[1000007];

bool chkmax(int* a, int b) {
    return (b > *a) ? *a = b, true : false;
}

int main() {
    scanf("%s%d", s, &n);

    int len = strlen(s);

    p = 1, mp[0] = 1, hi[0] = len;
    for(int i = len - 1; i >= 0; i--) {
        tot = (tot + (s[i] - '0') * p) % 33;
        if(mp[tot] == 0) hi[tot] = i; else lo[tot] = i;
        ++mp[tot];
        p = p * 10 % 33;
    }

    long long cnt = 0;
    for(int i = 0; i < 33; i++) {
        cnt += mp[i] * (mp[i] - 1) / 2;
    }

    if(n == 1) {
        printf("%lld\n", cnt);
    }
    else if(cnt == 0) {
        puts("-1");
    }
    else {
        int maxlen = 0;
        for(int i = 0; i < 33; i++) {
            if(mp[i] < 2) continue;
            chkmax(&maxlen, hi[i] - lo[i]);
        }
        strcpy(res, "");
        for(int i = 0; i < 33; i++) {
            if(hi[i] - lo[i] == maxlen) {
                strncpy(tmp, s + lo[i], maxlen);
                if(strcmp(tmp, res) > 0) {
                    strcpy(res, tmp);
                }
            }
        }
        printf("%s\n", res);
    }
    return 0;
}
