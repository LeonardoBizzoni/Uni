(defun fib (x)
  (cond ((> x 2) (+ (fib (- x 1)) (fib (- x 2))))
	((<= x 2) 1)))
