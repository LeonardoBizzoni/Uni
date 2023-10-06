works_for(gino, ibm).
works_for(ugo, ibm).
works_for(emma, ibm).
works_for(eloise, apple).
works_for(sandro, meta).
works_for(enrico, meta).
works_for(fulvio, meta).
works_for(ahmed, meta).
works_for(jimmy, meta).
works_for(edo, rubinetteria_riunite_di_san_capuzzano).
works_for(walter, rubinetteria_riunite_di_san_capuzzano).
works_for(stefano, apple).

colleague(X, Y) :- works_for(X, Z), works_for(Y, Z), X \= Y.
