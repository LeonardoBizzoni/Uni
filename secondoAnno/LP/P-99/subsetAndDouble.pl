% A
% subset(LContenuto, LContenente)

subset([], _).

subset([H | T], [H | Rest]) :-
    subset(T, Rest), !.

subset([H | T], [_ | Rest]) :-
    subset([H | T], Rest).

% B
% double(L, LL)

double([], []).
double([A | T], [A, A | Rest]) :-
    double(T, Rest).
