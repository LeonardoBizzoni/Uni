penultimo([X, _], X).
<<<<<<< Updated upstream
<<<<<<< Updated upstream
penultimo([_, X | T], Res) :-
    penultimo([X | T], Res).
=======
penultimo([_ | T], Res) :-
    penultimo(T, Res).
>>>>>>> Stashed changes
=======
penultimo([_ | T], Res) :-
    penultimo(T, Res).
>>>>>>> Stashed changes
