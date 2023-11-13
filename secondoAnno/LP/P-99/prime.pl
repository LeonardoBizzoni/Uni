prime_list_in(X, X, []) :- not(prime(X)), !.
prime_list_in(X, X, [X]) :- prime(X), !.
prime_list_in(X, Y, [X | T]) :-
    X < Y,
    prime(X),
    X2 is X+1,
    prime_list_in(X2, Y, T), !.
prime_list_in(X, Y, List) :-
    X < Y,
    not(prime(X)),
    X2 is X+1,
    prime_list_in(X2, Y, List), !.

prime(2).
prime(X) :- X < 2,!,false.
prime(X) :- not(divisible(X, 2)).

divisible(X,Y) :- 0 is X mod Y, !.
divisible(X,Y) :- X > Y+1, divisible(X, Y+1).
