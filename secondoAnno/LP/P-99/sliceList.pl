slice([H | _], X, X, [H]).

slice(L, X, Y, Res) :-
    X > Y,
    slice(L, Y, X, Res).

slice([_ | T], X, Y, Res) :-
    X > 1,
    X < Y,
    X2 is X-1,
    Y2 is Y-1,
    slice(T, X2, Y2, Res).

slice([H | T], 1, Y, [H | Res]) :-
    Y > 1,
    Y2 is Y-1,
    slice(T, 1, Y2, Res).
