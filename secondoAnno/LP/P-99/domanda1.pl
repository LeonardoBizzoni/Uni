domanda1([], _, []).

domanda1([[N1, N2] | T], N, [[N1, N2] | Rest]) :-
    N is N1 + N2, !,
    domanda1(T, N, Rest).

domanda1([_ | T], N, L2) :-
    writeln("Caso generico."),
    domanda1(T, N, L2).
