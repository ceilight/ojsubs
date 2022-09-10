// https://oj.vnoi.info/problem/fc009_circles

var n, ans: int64;

function pow(base, exp: int64): int64;
var res: int64;
begin
    res := 1;
    while exp > 0 do
    begin
        if exp mod 2 <> 0 then res := res * base;
        base := base * base;
        exp := exp div 2;
    end;
    exit(res);
end;

begin
    readln(n);
    if n = 0 then ans := 1
    else ans := pow(n, 2) - n + 2;
    writeln(ans);
end.
