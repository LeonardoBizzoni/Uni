pack([], []).
pack([X], [[X]]).

pack([X, X | T], [[X | RestHead] | RestBody]) :- !,
    pack([X | T], [RestHead | RestBody]).

pack([X, Y | T], [[X] | Rest]) :-
    X \= Y,
    pack([Y | T], Rest).
