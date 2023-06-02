// July 26, 2021, 3:19 p.m. 
var n, i: longint;
    sum, sum2, h: int64;

begin
    read(n);
    sum := 0;
    sum2 := 0;
    for i := 1 to n do
    begin
        read(h);
        sum := sum + h;
        sum2 := sum2 + h * h;
    end;
    writeln((sum * sum - sum2) div 2);
end.
