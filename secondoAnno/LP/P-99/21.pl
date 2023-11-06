insert_at(E, [H | T], 1, [E, H | T]).
insert_at(E, [H | T], N, [H | Rest]) :-
    N > 1,
    N2 is N-1,
    insert_at(E, T, N2, Rest).
