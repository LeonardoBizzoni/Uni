pari([], []).
pari([_], []).
pari([_, B | T], [B | Rest]) :-
    pari(T, Rest).
