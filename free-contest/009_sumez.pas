// https://oj.vnoi.info/problem/fc009_sumez

var
    a: array[0..100007] of int64;
    tree: array[0..400030] of int64;
    n, m, i, lo, hi: longint; 

procedure build(i, l, r: longint);
var
    m: longint;
begin
    if l = r then 
    begin
        tree[i] := a[l];
        exit;
    end;
    m := (l + r) div 2;
    build(i * 2 + 1, l, m);
    build(i * 2 + 2, m + 1, r);
    tree[i] := tree[i * 2 + 1] + tree[i * 2 + 2];
end;

function get(i, l, r, u, v: longint): int64;
var
    m: longint;
begin
    if (u > r) or (v < l) then exit(0);
    if (u <= l) and (r <= v) then exit(tree[i]);
    m := (l + r) div 2;
    exit(get(i * 2 + 1, l, m, u, v) + get(i * 2 + 2, m + 1, r, u, v)); 
end;

begin
    read(n, m);
    for i := 1 to n do read(a[i]);
    build(0, 1, n);
    for i := 1 to m do
    begin
        read(lo, hi);
        writeln(get(0, 1, n, lo, hi));
    end;
end.
