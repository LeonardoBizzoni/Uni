% A
% subset(LContenuto, LContenente)

subset([], _).

subset([H | T], [H | Rest]) :-
    subset(T, Rest), !.

subset([H | T], [H1 | Rest]) :-
    subset([H], Rest),
    subset(T, [H1 | Rest]).

% B
% double(L, LL)

double([], []).
double([A | T], [A, A | Rest]) :-
    double(T, Rest).
