%% remove_at(E, List, N, NewList)

remove_at(H, [H|T], 1, T).
remove_at(E, [H|T], N, [H|Rest]) :-
    N > 1,
    N2 is N-1,
    remove_at(E, T, N2, Rest).
