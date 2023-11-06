my_flatten([], []).

my_flatten([H | T], [H | Res]) :-
    \+ is_list(H),
    my_flatten(T, Res).

my_flatten([H | T], List) :-
    my_flatten(H, List1),
    my_flatten(T, List2),
    append(List1, List2, List).
