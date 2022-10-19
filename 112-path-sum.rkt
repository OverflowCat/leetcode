(
    define (has-path-sum root target) 
    (if 
        (not (tree-node? root)) #f 
        (
            if (and (= (tree-node-val root) target) (and (false? (tree-node-left root)) (false? (tree-node-right root))))
            #t
            (or
                (has-path-sum (tree-node-left root) (- target (tree-node-val root)))
                (has-path-sum (tree-node-right root) (- target (tree-node-val root)))
            )
        )
    )
)
