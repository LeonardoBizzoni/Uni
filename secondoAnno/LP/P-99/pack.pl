pack([], []).
pack([X], [[X]]).

pack([X, X | T], [[X | RestOfHead] | RestOfBody]) :-
    pack([X | T], [RestOfHead | RestOfBody]).

pack([X, Y | T], [[X] | Rest]) :-
    X \= Y,
    pack([Y | T], Rest).
