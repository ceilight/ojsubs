#include <bits/stdc++.h>

using namespace std;

void up()
{
    int n;
    cin>>n;
    
    int a[2*n];
    int b[2*n];
    int tot=0;
    for(int i=0; i<n*2; i++)
    {
        cin>>a[i]>>b[i];
        tot+=a[i];
        b[i]-=a[i];
    }

    sort(b,b+2*n);
    for(int i=0; i<n; i++)
        tot+=b[i];

    printf("%d\n",tot);
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
