print_node(Stream, K, V) :-
    write(Stream, "("),
    write(Stream, K),
    write(Stream, ": "),
    write(Stream, V),
    write(Stream, ")").

print_preorder(_Stream, nil) :- !.
print_preorder(Stream, node(K, V, Left, Right)) :-
    print_node(Stream, K, V),
    print_preorder(Stream, Left),
    print_preorder(Stream, Right).

print_inorder(_Stream, nil) :- !.
print_inorder(Stream, node(K, V, Left, Right)) :-
    print_inorder(Stream, Left),
    print_node(Stream, K, V),
    print_inorder(Stream, Right).

print_postorder(_Stream, nil) :- !.
print_postorder(Stream, node(K, V, Left, Right)) :-
    print_postorder(Stream, Left),
    print_postorder(Stream, Right),
    print_node(Stream, K, V).

treedict_traversal_preorder(treedict(_Name, Root)) :-
    print_preorder(user_output, Root).

treedict_traversal_inorder(treedict(_Name, Root)) :-
    print_inorder(user_output, Root).

treedict_traversal_postorder(treedict(_Name, Root)) :-
    print_postorder(user_output, Root).
