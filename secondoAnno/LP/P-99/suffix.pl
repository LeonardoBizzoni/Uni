suffix([], [_ | _]).
suffix(Suff, [_ | Suff]) :- is_list(Suff), !.
%% suffix([A | T], [_ | [A | T]]) :- !.

suffix(Suffix, [_, H | Rest]) :-
    suffix(Suffix, [H | Rest]).
