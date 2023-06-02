// May 15, 2021, 7:13 p.m. 
#include <bits/stdc++.h>

using namespace std;

int sieve[3000005];
vector<int> primes;

typedef unsigned long long UL;

void init()
{
    for(int i=2; i<3000005; i++)
    {
        if(sieve[i]) continue;
        for(int j=i; j<3000005; j+=i) sieve[j]=i;
        primes.push_back(i);
    }
}

void up()
{
    UL n;
    int k;
    cin>>n>>k;

    UL mult=1ull;
    
    for(int i=0; i<k; i++) mult*=primes[i];
    if(mult>n)
    {
        cout<<"-1\n";
        return;
    }

    int l=0;
    int r=primes.size()-k;

    while(l<r)
    {
        int m=(l+r+1)/2;
        mult=1ull;
        bool flag=1;

        for(int i=m; i<m+k; i++)
        {
            if(n/mult<(UL)primes[i])
            {
                flag=0;
                break;
            }
            else mult*=primes[i];
        }
        if(flag) l=m; else r=m-1;
    }

    mult=1ull;
    for(int i=l; i<l+k; i++) mult*=primes[i];
    cout<<mult<<"\n";
}

int main()
{
    ios::sync_with_stdio(0);
    cin.tie(0);

    init();

    int tc;
    cin>>tc;

    while(tc--) up();

    return 0;
}
