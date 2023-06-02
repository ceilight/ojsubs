// May 15, 2021, 9:04 p.m. 
#include <bits/stdc++.h>

using namespace std;

bitset<100000007> sieve;
vector<int> primes;

void init()
{
    for(int i=2; i*i<100000007; i++)
    {
        if(sieve[i]) continue;
        for(int j=i*i; j<100000007; j+=i) sieve[j]=1;
    }
    for(int i=2; i<100000007; i++)
    {
        if(!sieve[i]) primes.push_back(i);
    }
}

void up()
{
    int x;
    while(1)
    {
        cin>>x;

        if(x==0) return;

        int p=upper_bound(primes.begin(),primes.end(),x)-primes.begin();
        printf("%.1f\n",100.0*abs(p-x/log(x))/p);
    }
}

int main()
{
    ios::sync_with_stdio(0);
    cin.tie(0);

    init();
    up();

    return 0;
}
