• Architecture Roadmap

  - rlisp/repl/src/main.rs:11 already gives you a clean REPL shell; keep it thin and push language logic into a new crate (lispr-core) so the CLI just wires IO
    to lispr_core::eval_str.
  - Phase the language into layers: 1) tokenizer → sequence of Token enums (great for learning iterators and slices); 2) parser → general Expr (Number, Symbol,
    List, Quote); 3) evaluator → Value (Number, Bool, Symbol, List, Lambda, Builtin); 4) environment → persistent scopes backed by HashMap and optional reference
    counting (Rc<RefCell<…>> or im for learning interior mutability); 5) forms → core and user-facing standard library defined either in Rust or self-hosted Lisp.

  Rust-Learning Focus

  - Tokenizer: implement by hand first to practice pattern matching and slice APIs, then try logos to compare declarative lexers.
  - Parser: keep nom (rlisp/repl/src/parser.rs:12) for complex forms; introduce custom error types via thiserror so parse failures are informative. Unit-test with
    rstest table cases and property checks via proptest.
  - Evaluation: model user functions as Rc<Lambda>; leverage pattern matching and Result<Value, RuntimeError> to drill error propagation. Introduce tail-call
    handling with trampoline pattern to explore enums & loops.
  - Environment: start with HashMap<String, Value> stored inside a Vec for lexical scopes; evolve to persistent maps (im::HashMap) to learn external crates and
    borrowing pitfalls.
  - Special forms & macros: implement define, set!, lambda, if, cond, let. Later add macro support (syntax-quote, quasi/unquote) to practice nested pattern
    matching.
  - Numeric tower: extend Value::Number to BigRational/BigInt using num crate; this teaches crates and trait impls. Add Display via fmt::Display to learn
    formatting traits.

  Project Structure

  - lispr-core/src/lib.rs: exports eval_str.
  - Modules: lexer, parser, ast, env, builtins, evaluator, reader (for quasiquote & macro expansion), errors.
  - rlisp/repl/src/main.rs: depends on lispr-core and handles history/prompt. Later add --file CLI parsing (learn clap).
  - Tests: unit tests per module; integration tests under lispr-core/tests loading sample programs. Snapshot expected REPL transcripts with insta.
  - Benchmarks: add criterion benches comparing builtin arithmetic vs user-defined recursion for performance insight.

  Learning Milestones

      1. Tokenizer & general AST: focus on slice iterators, enums, pattern matching.
      2. Parser rewrite: explore combinators vs hand-written recursive descent; compare ergonomics.
      3. Evaluator with environments: dive into ownership, references, and smart pointers.
      4. Standard library bootstrap: implement arithmetic, comparisons, list ops; learn module organization and public/private items.
      5. Macro system & error reporting: practice advanced enum usage, trait impls, and structured errors.
      6. Tooling polish: add cargo fmt, cargo clippy, feature flags for tracing (tracing crate) to learn debugging ecosystems.
      7. Interop experiments: serialize AST with serde to explore trait derivations; embed the engine in WASM later to apply Rust’s wasm toolchain.

  Next Actions

  - Flesh out design notes inside rlisp/repl/todo.org or create rlisp/ARCHITECTURE.md capturing the module breakdown.
  - Decide whether to split into workspace crates now or after the parser rewrite.
  - Start with tokenizer/AST refactor; update tests as you go and document findings for your Go port later.
