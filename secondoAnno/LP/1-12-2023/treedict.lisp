(defun make-treedict (name root)
  (list :name name
	:tree root))

(defun treedict-insert (dict key value)
  (defun node-insert (tree key value)
    (defun make-node (key value left right)
      (list :key key
	    :value value
	    :children (list left right)))

    (cond ((null tree) (make-node key value nil nil))
	  ((< key (getf tree :key))
	   (list :key (getf tree :key)
		 :value (getf tree :value)
		 :children (list (node-insert (first (getf tree :children))
					      key value)
				 (second (getf tree :children)))))
	  ((> key (getf tree :key))
	   (list :key (getf tree :key)
		 :value (getf tree :value)
		 :children (list (first (getf tree :children))
				 (node-insert (second (getf tree :children))
					      key value))))
	  ((= key (getf tree :key))
	   (list :key key
		 :value value
		 :children (getf tree :children)))))

  (list :name (getf dict :name)
	:tree (node-insert (getf dict :tree) key value)))

(defun treedict-search (dict key)
  (defun node-search (tree key)
    (cond ((null tree) nil)
	  ((< key (getf tree :key))
	   (node-search (first (getf tree :children)) key))
	  ((> key (getf tree :key))
	   (node-search (second (getf tree :children)) key))
	  ((= key (getf tree :key))
	   (getf tree :value))))

  (node-search (getf dict :tree) key))
