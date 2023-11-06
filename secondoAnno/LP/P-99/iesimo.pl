iesimo([X | _], 1, X).
iesimo([_ | T], I, Res) :-
    I > 1,
    I2 is I-1,
    iesimo(T, I2, Res).
