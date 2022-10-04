(define (has-path-sum node target)
    (if 
        (or (tree-node? (tree-node-left node)) (tree-node (tree-node-right node)))
        (
            or
            (and (tree-node? (tree-node-left  node)) (has-path-sum (tree-node-left  node) (- target (tree-node-val node))))
            (and (tree-node? (tree-node-right node)) (has-path-sum (tree-node-right node) (- target (tree-node-val node))))
        )
        (equal? target (tree-node-val node))
    )
)
