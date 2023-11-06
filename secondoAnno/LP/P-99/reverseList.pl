reverse(List, Res) :- reverse(List, Res, []).

reverse([], L, L).
reverse([H | T], Res, Acc) :-
    reverse(T, Res, [H | Acc]).
