import io, os
input = io.BytesIO(os.read(0,os.fstat(0).st_size)).readline


def compute_lps(pat):
    '''compute the failure function of a pattern'''
    lps = [-1] * len(pat)
    cand = -1
    for i in range(1, len(pat)):
        while cand >= 0 and pat[i] != pat[cand + 1]:
            cand = lps[cand]
        if pat[i] == pat[cand + 1]:
            cand += 1
        lps[i] = cand
    return lps


def kmp(pat, word):
    '''KMP pattern search algorithm'''
    cand = -1
    lps = compute_lps(pat)
    for i in range(len(word)):
        while cand >= 0 and word[i] != pat[cand + 1]:
            cand = lps[cand]
        if word[i] == pat[cand + 1]:
            cand += 1
    return cand + 1


def main():
    n = int(input())
    word = input().decode().split()
    ans = []
    for c in word[0]:
        ans.append(c)
    for i in range(1, n):
        tomerge = word[i]
        start = len(ans) - min(len(ans), len(word[i]))
        max_share = ans[start:]
        matching = kmp(tomerge, max_share)
        for j in range(matching, len(word[i])):
            ans.append(word[i][j])
            
    print(''.join(map(str, ans)))


if __name__ == "__main__":
    main()