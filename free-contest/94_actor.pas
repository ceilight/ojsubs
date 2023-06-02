// July 26, 2021, 11:59 a.m. 
uses math;
var r, t, n, a, b, c, i: longint;

begin
    read(r);
    if r = 2 then
    begin
        read(t);
        for i := 1 to t do
        begin
            read(n, a, b, c);
            writeln(min(a, min(b, c)));
        end;
    end
    else
    begin
        read(t);
        for i := 1 to t do
        begin
            read(n, a, b, c);
            writeln(max(0, a + b + c - 2 * n));
        end;
    end;
end.
