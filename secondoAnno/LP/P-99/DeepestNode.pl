deepest_node(node(Key, Value, nil, nil),
	     node(Key, Value, nil, nil),
	     0).

deepest_node(node(_, _, Left, nil), DN, D) :-
    deepest_node(Left, DN, DLeft),
    D is DLeft + 1.

deepest_node(node(_, _, nil, Right), DN, D) :-
    deepest_node(Right, DN, DRight),
    D is DRight + 1.

deepest_node(node(_, _, Left, Right), DN, D) :-
    deepest_node(Left, DNLeft, DLeft),
    deepest_node(Right, DNRight, DRight),
    (
	DLeft >= DRight,
	D is DLeft + 1,
	DN = DNLeft;

	DLeft < DRight,
	D is DRight + 1,
	DN = DNRight
    ).
