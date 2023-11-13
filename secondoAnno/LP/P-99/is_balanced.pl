is_bal(nil) :- !.
is_bal(node(_, _, nil, nil)) :- !.

is_bal(node(_, _, L, R)) :-
    count_nodes(L, Lnode),
    count_nodes(R, Rnode),

    Lnode - Rnode < 2,
    Lnode - Rnode > -2,

    is_bal(L),
    is_bal(R).

count_nodes(nil, 0) :- !.
count_nodes(node(_, _, nil, nil), 1) :- !.
count_nodes(node(_, _, L, R), X) :-
    count_nodes(L, NL),
    count_nodes(R, NR),
    X is NL + NR + 1.
