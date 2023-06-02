// May 15, 2021, 1:54 p.m. 
#include <bits/stdc++.h>

using namespace std;

struct edge
{
    int u;
    int v;
    int w;

    bool operator <(edge e)
    {
        return w<e.w;
    }
};

struct dsu
{
    vector<int> p;
    vector<int> r;

    void build(int n)
    {
        p.resize(n);
        r.resize(n);
        for(int i=0; i<n; i++) p[i]=i;
        for(int i=0; i<n; i++) r[i]=0;
    }

    int find(int a)
    {
        while(p[a]!=a) a=p[a];
        return a;
    }

    bool join(int x, int y)
    {
        int u=find(x);
        int v=find(y);

        if(u==v) return 0;

        if(r[u]>r[v]) swap(u,v);
        if(r[u]==r[v]) r[v]++;
        p[u]=v;
        return 1;
    }
};

void up()
{
    int n;
    int m;
    cin>>n>>m;

    vector<edge> e;

    int ans=0;

    for(int i=0; i<m; i++)
    {
        int u;
        int v;
        int w;
        cin>>u>>v>>w;

        u--;
        v--;
        e.push_back({u,v,w});
        ans+=w;
    }

    sort(e.rbegin(),e.rend());

    dsu st;
    st.build(n);

    for(int i=0; i<m; i++) if(st.join(e[i].u,e[i].v)) ans-=e[i].w;
    printf("%d\n",ans);
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
