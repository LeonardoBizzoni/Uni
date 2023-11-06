len([], 0).
len([_ | T], Res) :-
    len(T, R1),
    Res is R1+1.
