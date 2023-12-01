:- dynamic treedict_version/2.

is_node(nil).
is_node(node(_Key, _Value, _Left, _Right)).

node_is_leaf(node(_, _, nil, nil)).

node_insert(Key, Value, nil, node(Key, Value, nil, nil)) :- !.

node_insert(Key, NewValue,
	    node(Key, _, Left, Right),
	    node(Key, NewValue, Left, Right)) :- !.

node_insert(NewKey, NewValue,
	    node(Key, Value, Left, Right),
	    node(Key, Value, NewNode, Right)) :-
    NewKey @< Key, !,
    node_insert(NewKey, NewValue, Left, NewNode).

node_insert(NewKey, NewValue,
	    node(Key, Value, Left, Right),
	    node(Key, Value, Left, NewNode)) :-
    NewKey @> Key, !,
    node_insert(NewKey, NewValue, Right, NewNode).

node_search(Key, node(Key, Value, _, _), Value).
node_search(Key, node(K, _, Left, _), Res) :-
    Key @< K, !,
    node_search(Key, Left, Res).

node_search(Key, node(K, _, _, Right), Res) :-
    Key @> K, !,
    node_search(Key, Right, Res).

node_print(_, nil, _).

node_print(Stream, Node, NTabs) :-
    node_print(Stream, Node, NTabs, NTabs).

node_print(Stream, node(K, V, Left, Right), 0, CTabs) :-
    write(Stream, K),
    write(Stream, ": "),
    write(Stream, V),
    nl(Stream),
    ChildTabs is CTabs + 5,
    node_print(Stream, Left, ChildTabs),
    node_print(Stream, Right, ChildTabs).

node_print(Stream, Node, NTabs, CTabs) :-
    Node \= nil,
    NTabs > 0,
    write(Stream, " "),
    Tabs is NTabs - 1,
    node_print(Stream, Node, Tabs, CTabs).

node_min(node(K, _, nil, _), K) :- !.
node_min(node(_, _, L, _), Min) :-
    L \= nil, !,
    node_min(L, Min).

node_max(node(K, _, _, nil), K) :- !.
node_max(node(_, _, _, R), Max) :-
    R \= nil, !,
    node_max(R, Max).

is_treedict(treedict(_Nome, Root)) :-
    is_node(Root).

treedict_insert(Key, Value,
		treedict(Nome, Root),
		treedict(Nome, NewRoot)) :-
    node_insert(Key, Value, Root, NewRoot).

treedict_search(Key, treedict(_, Root), Value) :-
    node_search(Key, Root, Value).

treedict_print(Tree) :-
    treedict_print(user_output, Tree).

treedict_print(Stream, treedict(Name, Root)) :-
    write(Stream, 'Dict: '),
    writeln(Stream, Name),
    nl(Stream),
    node_print(Stream, Root, 0),
    writeln(Stream, "Dict end."),
    nl(Stream).
