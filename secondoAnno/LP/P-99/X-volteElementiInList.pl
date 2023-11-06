dupli(L, X, Res) :-
    dupli(L, X, Res, X).

dupli([], _, [], _).
dupli(L, 1, L, _).
dupli([H | T], X, [H | Res], K) :-
    K > 0,
    K2 is K-1,
    dupli([H | T], X, Res, K2).

dupli([_ | T], X, Res, 0) :-
    dupli(T, X, Res, X).
