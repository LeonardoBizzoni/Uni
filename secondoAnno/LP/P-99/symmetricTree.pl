symmetric(nil).
symmetric(node(_, _, nil, nil)).

symmetric(node(_, _, L, R)) :-
    L \= nil,
    R \= nil,
    mirror(R, L).

mirror(nil, nil) :- !.
mirror(node(K, V, nil, nil), node(K, V, nil, nil)) :- !.
mirror(node(K, V, L, R), node(K, V, NewL, NewR)) :-
    mirror(L, NewR),
    mirror(R, NewL).
