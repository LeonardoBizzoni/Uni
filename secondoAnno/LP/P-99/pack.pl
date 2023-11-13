pack([], []).
<<<<<<< Updated upstream
<<<<<<< Updated upstream
pack([X], [[X]]).

pack([X, X | T], [[X | RestOfHead] | RestOfBody]) :-
    pack([X | T], [RestOfHead | RestOfBody]).
=======
=======
>>>>>>> Stashed changes
pack([A], [[A]]).

pack([X, X | T], [[X | RestHead] | RestBody]) :- !,
    pack([X | T], [RestHead | RestBody]).
<<<<<<< Updated upstream
>>>>>>> Stashed changes
=======
>>>>>>> Stashed changes

pack([X, Y | T], [[X] | Rest]) :-
    X \= Y,
    pack([Y | T], Rest).
