range(A, A, [A]).

range(A, B, List) :-
    A > B,
    range(B, A, List).

range(A, B, [A | Rest]) :-
    A < B,
    A2 is A+1,
    range(A2, B, Rest), !.
