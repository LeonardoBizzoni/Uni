% sostituisci(Cosa, ConCosa, List, Res).

sostituisci(_, _, [], []).
sostituisci(X, Y, [X | T], [Y | Rest]) :-
    sostituisci(X, Y, T, Rest), !.
sostituisci(X, Y, [Z | T], [Z | Rest]) :-
    X \= Z,
    sostituisci(X, Y, T, Rest).
