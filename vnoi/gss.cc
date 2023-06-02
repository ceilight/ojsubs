// May 16, 2021, 10:33 a.m. 
#include <bits/stdc++.h>

using namespace std;

const int oo=1e9;

struct node
{
    int sum;
    int pre;
    int suf;
    int res;

    node(int _arg=-oo) 
    { 
        sum=pre=suf=res=_arg;
    }
};

node combine(node u, node v)
{
    if(u.res==-oo) return v;
    if(v.res==-oo) return u;
    node merge;
    merge.sum=u.sum+v.sum;
    merge.pre=max(u.pre,v.pre+u.sum);
    merge.suf=max(v.suf,u.suf+v.sum);
    merge.res=max(u.suf+v.pre,max(u.res,v.res));
    return merge;
}

struct segment_tree
{
    vector<node> tree;

    void build(int* a, int pos, int l, int r)
    {
        if(l>r) return;
        if(l==r)
        {
            tree[pos]=node(a[l]);
            return;
        }

        int m=(l+r)/2;
        build(a,pos*2+1,l,m);
        build(a,pos*2+2,m+1,r);
        tree[pos]=combine(tree[pos*2+1],tree[pos*2+2]);
    }

    void init(int* a, int n)
    {
        tree.resize(4*n);
        fill(tree.begin(),tree.end(),node());
        build(a,0,0,n-1);
    }

    node get(int pos, int l, int r, int L, int R)
    {
        if(L>R || l>r) return node();
        if(L>r || R<l) return node();
        if(L<=l && r<=R) return tree[pos];

        int m=(l+r)/2;
        node l_child=get(pos*2+1,l,m,L,R);
        node r_child=get(pos*2+2,m+1,r,L,R);
        return combine(l_child,r_child);
    }
};

void up()
{
    int n;
    cin>>n;

    int a[n];
    for(int i=0; i<n; i++) cin>>a[i];

    segment_tree smt;
    smt.init(a,n);

    int m;
    cin>>m;

    for(int i=0; i<m; i++)
    {
        int l;
        int r;
        cin>>l>>r;

        printf("%d\n",smt.get(0,0,n-1,l-1,r-1).res);
    }
}

int main()
{
    ios::sync_with_stdio(0);
    cin.tie(0);

    up();

    return 0;
}
