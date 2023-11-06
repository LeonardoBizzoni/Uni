drop(L, N, Res) :-
    drop(L, N, Res, N).

drop([], _, [], _).

drop([_ | T], N, Res, 1) :-
    drop(T, N, Res, N).

drop([H | T], N, [H | Res], K) :-
    K > 1,
    K2 is K - 1,
    drop(T, N, Res, K2).
