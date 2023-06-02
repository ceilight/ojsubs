// July 26, 2021, 6:32 p.m. 
var n, x, i: longint;
    one, zero, res: int64;

begin
    read(n);
    for i := 1 to n do
    begin
        read(x);
        if x = 1 then inc(one) else inc(zero);
    end;
    writeln(one * (one - 1) div 2 + zero * (zero - 1) div 2);
end.
