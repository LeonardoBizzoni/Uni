is_bst(nil).
is_bst(node(K, _Value, Left, Right)) :-
    max_child(Left, LMax),
    min_child(Right, RMin),
    K @> LMax,
    K @< RMin,
    is_bst(Left),
    is_bst(Right).

max_child(nil, _).
max_child(node(K, _, Left, Right), Res) :-
    max_child(Left, LMax),
    max_child(Right, RMax),
    max(LMax, RMax, Max),
    max(Max, K, Res).

min_child(nil, inf).
min_child(node(K, _, Left, Right), Res) :-
    min_child(Left, LMin),
    min_child(Right, RMin),
    min(LMin, RMin, Min),
    min(Min, K, Res).

max(A, B, A) :- A @>= B, !.
max(A, B, B) :- A @< B.

min(A, B, A) :- A @=< B, !.
min(A, B, B) :- A @> B.

%% ----------------------------------------------
%% Solo numeri

is_bstv2(nil).
is_bstv2(node(Key, _, L, R)) :-
    Min is -inf,
    Max is inf,
    is_bstv2(node(Key, _, L, R), Min, Max).

is_bstv2(nil, _, _) :- !.
is_bstv2(node(Key, _, L, R), Kmin, Kmax) :-
    Kmin =< Key,
    Key =< Kmax, !,
    is_bstv2(L, Kmin, Key),
    is_bstv2(R, Key, Kmax).
