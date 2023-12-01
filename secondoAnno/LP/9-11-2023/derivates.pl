derivata(X, X, 1) :- !.
derivata(e(X), X, e(X)) :- !.
derivata(ln(X), X, 1/X) :- !.
derivata(sin(X), X, cos(X)) :- !.
derivata(cos(X), X, -sin(X)) :- !.

derivata(F + G, X, DF + DG) :- !,
    derivata(F, X, DF),
    derivata(G, X, DG).

derivata(F * G, X, (DF * G) + (DG * F)) :- !,
    derivata(F, X, DF),
    derivata(G, X, DG).

derivata(F / G, X, ((DF * G) - (DG * F)) / (DG^2)) :- !,
    derivata(F, X, DF),
    derivata(G, X, DG).

derivata(FGX, X, DFGX * DGX) :- !,
    arg(1, FGX, GX),
    derivata(FGX, GX, DFGX),
    derivata(GX, X, DGX).

derivata(X, Z, 0) :-
    X \= Z, !,
    atomic(X).
