count_leaves(nil, 0).
count_leaves(node(_, _, nil, nil), 1) :- !.
count_leaves(node(_, _, L, R), X) :-
    count_leaves(L, X1),
    count_leaves(R, X2),
    X is X1 + X2.
