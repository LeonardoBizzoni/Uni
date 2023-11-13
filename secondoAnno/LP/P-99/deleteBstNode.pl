delete_node(K, node(K, _, nil, nil), nil) :- !.
delete_node(K, node(K, _, Left, nil), Left) :- !.
delete_node(K, node(K, _, nil, Right), Right) :- !.

delete_node(K, node(K1, V, L, R), node(K1, V, NewLeft, R)) :-
    K @< K1, !,
    delete_node(K, L, NewLeft).

delete_node(K, node(K1, V, L, R), node(K1, V, L, NewRight)) :-
    K @> K1, !,
    delete_node(K, R, NewRight).

delete_node(K,
	    node(K, _, Left, Right),
	    node(K1, V, Left, R2)) :-
    Left \= nil,
    Right \= nil,
    node_succ(Right, node(K1, V, nil, _)),
    delete_node(K1, Right, R2).

node_succ(nil, nil) :- !.
node_succ(node(K, V, nil, R), node(K, V, nil, R)) :- !.
node_succ(node(_, _, L, _), Res) :-
    L \= nil,
    node_succ(L, Res).
