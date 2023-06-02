// July 27, 2021, 11:17 a.m. 
var cnt, i, len: longint;
    a: array[0..26] of boolean;
    s: string;

begin
    read(s);
    len := length(s);
    for i := 1 to len do
        if a[ord(s[i]) - 97] = false then a[ord(s[i]) - 97] := true;
    for i := 0 to 26 do
        if a[i] = true then inc(cnt);
    write(cnt);
end.
