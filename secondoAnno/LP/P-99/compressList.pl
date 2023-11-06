compress([], []).
compress([A], [A]).

compress([X, Y | T], [X | Res]) :-
    X \= Y,
    compress([Y | T], Res).

compress([X, X | T], Res) :-
    compress([X | T], Res).
