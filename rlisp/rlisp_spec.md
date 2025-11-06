# RLisp / Lispr Language Spec
- No such thing as null/nil, unacceptable.
- Lisp-1, 
  single namespace,  
  “look up the symbol’s var in its namespace (which may contain a function value).” like clojure
  Clojure remains a Lisp-1 — even though it looks namespaced, the distinction is still lexical, not semantic like a Lisp-2.
  Like Elixir as well.
- Clojure like for simplistic syntax.
- function-names kabob case, Y shift if you don't have to, less keystrokes
- Atoms, registry maybe
- Threading, thread-first, thread-n, |>, |> then(&)
- Immutable, always (may have performance concerns), by default maybe
- Repl is good, like really good
- Docstrings """...""" but no comments, maybe do we really need comments in code
- Strings "..."
- anon functions or closures simple and first class
- Lists ( ), Vectors [ ]
- KV structure is List of Tuples [{key1: value1}, {key2: value2}]
- let's have the concept of function arity like Elixir, foo/2, is different from foo/3
