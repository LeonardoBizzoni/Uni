split(L, 0, [], L).
split([H | T], 1, [H], T).

split([H | T], X, [H | Res], L2) :-
    X > 1,
    X2 is X-1,
    split(T, X2, Res, L2).
