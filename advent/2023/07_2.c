#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <assert.h>

#define L 8
#define C 20
#define N 1024

int cards[C][2], b, n;
char s[L];

struct hand {
    int bid;
    // 5*4 lower bits store the value of 5 cards in a hand
    // The remaining higher bits store the coefficient of a hand
    // that determines the order of hands by types, based on 2 labels
    // that appear the most in that hand
    uint32_t mask;
} h[N];

uint32_t label_value(char c) {
    if (c >= '2' && c <= '9') return c - '0';
    switch (c) {
    case 'T': return 10;
    case 'J': return 0;
    case 'Q': return 12;
    case 'K': return 13;
    case 'A': return 14;
    default: return 15;
    }
}

int cmp_card_dsc(const void *a, const void *b) {
    const int *ai = (int *)a, *bi = (int *)b;
    return (ai[1] < bi[1]) - (ai[1] > bi[1]);
}

int cmp_hand_asc(const void *a, const void *b) {
    const struct hand *ah = (struct hand *)a, *bh = (struct hand *)b;
    return (ah->mask > bh->mask) - (ah->mask < bh->mask);
}

int main(void) {
    while (scanf("%s %d", s, &b) == 2) {
        uint32_t mask = 0;
        for (int i = 0; i < C; i++) {
            cards[i][0] = i;
            cards[i][1] = 0;
        }

        for (int i = 0; s[i]; i++) {
            uint32_t v = label_value(s[i]);
            if (v == 15)
                continue;
            mask = (mask << 4) | v;
            assert(cards[v][0] == v);
            cards[v][1]++;
        }

        qsort(cards, C, sizeof(*cards), cmp_card_dsc);

        int j_idx;
        for (j_idx = 0; j_idx < 5; j_idx++) {
            if (cards[j_idx][0] == 0)
                break;
        }
        if (j_idx < 5 && cards[j_idx][1] > 0) {
            int j_cnt = cards[j_idx][1];
            for (int i = j_idx; i < L-1; i++) {
                cards[i][0] = cards[i+1][0];
                cards[i][1] = cards[i+1][1];
            }
            cards[0][1] += j_cnt;
        }

        mask |= (uint32_t)(cards[0][1] * 2 + cards[1][1]) << 20;
        h[n].bid = b;
        h[n].mask = mask;
        n++;
    }

    long long ans = 0;
    qsort(h, n, sizeof(*h), cmp_hand_asc);
    for (int i = 0; i < n; i++) {
        ans += (long long)h[i].bid * (long long)(i + 1);
    }
    printf("%d\n", ans);
}