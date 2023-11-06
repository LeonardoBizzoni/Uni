penultimo([X, _], X).
penultimo([_, X | T], Res) :-
    penultimo([X | T], Res).
