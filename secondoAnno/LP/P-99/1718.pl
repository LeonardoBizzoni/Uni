% sostituisci(Cosa, ConCosa, List, Res).

sostituisci(_, _, [], []).
sostituisci(X, Y, [X | T], [Y | Rest]) :-
    sostituisci(X, Y, T, Rest).

sostituisci(X, Y, [H | T], [H | Rest]) :-
    X \= H,
    sostituisci(X, Y, T, Rest).
