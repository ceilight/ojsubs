// https://oj.vnoi.info/problem/fc128_lock

const 
    weight: array[0..9] of longint = (6, 2, 5, 5, 4, 5, 6, 3, 7, 6);

var 
    n, m, i, j, x: longint;
    last, len: array[0..100007] of longint;
    d: array[0..10] of longint;

begin
    read(n, m);
    for i := 0 to m do read(d[i]);
    for i := 1 to n do len[i] := -100000007;
    for i := 1 to n do 
        for j := 0 to m - 1 do
        begin
            x := d[j];
            if (i = n) and (x = 0) and (n <> 6) then continue;
            if i < weight[x] then continue;
            if len[i] <= len[i - weight[x]] + 1 then
            begin
                len[i] := len[i - weight[x]] + 1;
                last[i] := x;
            end;
        end;

    repeat
    begin
        x := last[n];
        write(x);
        n := n - weight[x];
    end;
    until n <= 0;
end.
