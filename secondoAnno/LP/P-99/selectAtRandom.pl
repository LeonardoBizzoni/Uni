rng_select(_, 0, []) :- !.
rng_select(List, N, [E | T]) :-
    N > 0,
    length(List, Len),

    random_between(1, Len, X),
    get_element(List, X, E),

    N2 is N - 1,
    rng_select(List, N2, T).

get_element([H | _], 1, H) :- !.
get_element([_ | T], N, E) :-
    N > 1,
    N2 is N - 1,
    get_element(T, N2, E).
