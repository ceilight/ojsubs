#include <bits/stdc++.h>

using namespace std;

int S1, S2, S3, f[100];

int main() {
    cin >> S1 >> S2 >> S3;
    for(int i = 1; i <= S1; i++)
        for(int j = 1; j <= S2; j++)
            for(int k = 1; k <= S3; k++)
                f[i + j + k]++;
    cout << (int)(max_element(f, f + 100) - f);
    return 0;
}
