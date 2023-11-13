coprime(X, Y) :-
    gcd(X, Y, 1), !.

gcd(0, Y, Y) :- Y > 0, !.

gcd(X, Y, Res) :-
    X < Y,
    Y2 is Y-X,
    gcd(X, Y2, Res).

gcd(X, Y, Res) :-
    X >= Y,
    X2 is X-Y,
    gcd(X2, Y, Res).
