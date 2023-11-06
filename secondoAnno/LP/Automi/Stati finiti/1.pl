initial(q0).
final(q2).

delta(q0, 0, q1).
delta(q0, 1, q0).

delta(q1, 0, q1).
delta(q1, 1, q2).

delta(q2, 0, q1).
delta(q2, 1, q0).

accept([], S) :- final(S).
accept([H | T], S) :-
    delta(S, H, N), !,
    accept(T, N).

recognize(List) :- initial(S), accept(List, S).
