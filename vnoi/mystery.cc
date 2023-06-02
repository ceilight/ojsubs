// May 17, 2021, 6:27 p.m.
#include <bits/stdc++.h>

using namespace std;

typedef int64_t L;

const L md=20122007;

L mod_pow(L n, L m=md)
{
    L res=1;
    L b=3;
    while(n>0)
    {
        if(n%2) res=(res*b)%m;
        n/=2;
        b=(b*b)%m;
    }
    return res;
}

void up()
{
    L n;
    cin>>n;

    L ans=1;
    for(L i=1; i*i<=n; i++)
        if(n%i==0)
        {
            ans=(ans*(mod_pow(i)-1))%md;
            if(n/i>i) ans=(ans*(mod_pow(n/i)-1))%md;
        }

    printf("%I64ld",ans);
}

int main()
{
    ios::sync_with_stdio(0);
    cin.tie(0);

    int tc=1;
    //cin>>tc; 
    while(tc--) up();

    return 0;
}
 