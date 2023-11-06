duplicate([], []).
duplicate([H | T], [H, H | X]) :-
    duplicate(T, X).
