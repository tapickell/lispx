• Architecture Roadmap

  [x] rlisp/repl/src/main.rs:11 already gives you a clean REPL shell; keep it thin and push language logic into a new crate (lispr-core) so the CLI just wires IO
    to lispr_core::eval_str.
  - Phase the language into layers: 
    [ ] 1) tokenizer → sequence of Token enums (great for learning iterators and slices); 
    [ ] 2) parser → general Expr (Number, Symbol, List, Quote); 
    [ ] 3) evaluator → Value (Number, Bool, Symbol, List, Lambda, Builtin); 
    [ ] 4) environment → persistent scopes backed by HashMap and optional reference counting (Rc<RefCell<…>> or im for learning interior mutability); 
    [ ] 5) forms → core and user-facing standard library defined either in Rust or self-hosted Lisp.

  Rust-Learning Focus

  [ ] Tokenizer: implement by hand first to practice pattern matching and slice APIs, then try logos to compare declarative lexers.
    - Start by thinking of the tokenizer as a disciplined walk through the input string, turning stretches of characters into Token { kind, lexeme, span } values. A straightforward hand-rolled scanner gives you lots of Rust practice and keeps the logic easy to tweak later. Here’s a good sequence:
    1. Set up the loop
        - Work directly on the &str using indices, e.g. let chars: Vec<char> = input.chars().collect(); let mut i = 0;.
        - While i < chars.len(), inspect chars[i] and branch based on the current character.
    2. Skip trivia early
        - Whitespace (' ', '\t', '\n', etc.) just increments i.
        - Implement comments immediately so they don’t complicate later logic. For line comments (; → newline) advance
            until \n or EOF; for block comments (#| ... |#) find the matching |#, tracking nesting if you support it.
    3. Emit single-character tokens
        - When you meet delimiters (, ), [, ], {, }, or quote markers (') push the right TokenKind with span i..i+1 and
            bump i.
        - If you’re using literal quote/unquote/unquote_splicing rather than punctuation, treat them as Symbols for now—
            the parser can match them later.
    4. Read strings
        - On ", capture the start index, advance until you hit another unescaped ", building the lexeme along the way.
        - Handle escapes (\", \\, \n, etc.) as needed; convert invalid sequences into TokenError::InvalidEscape.
        - If EOF arrives before the closing ", return TokenError::UnterminatedString { span }.
    5. Numbers vs symbols
        - If the current character is digit (and optionally - followed by digit), scan forward while you see more digits
            (and maybe a single .). Collect the substring; leave parsing (int vs float) to later.
        - Otherwise treat everything up to the next delimiter/whitespace as a symbol lexeme. This covers atoms, keywords,
            result tags (Ok, {error:...}), etc. The parser will interpret the lexeme according to context, so the tokenizer
            stays simple.
    6. Add the spans
        - For each token, the span is the before/after indices (Span { start, end: i_after }). Don’t increment i twice—
            just track the start before scanning and set the end once you’re done.
    7. Return structured errors
        - When you encounter an unexpected character (e.g., control chars you don't support), return
            TokenError::UnexpectedChar { span, ch }.
        - Make TokenError printable (impl fmt::Display or thiserror::Error) so higher layers can surface friendly
            messages.
    8. Testing harness
        - Add initial tests in #[cfg(test)] mod tests right inside tokenizer.rs: assert that ( + 1 2 ) yields the
            expected tokens, check trimmed spans, confirm comments/spacing are skipped, and ensure errors are raised for
            bad input. This will catch regressions early as you expand features.

  As you build, keep the tokenizer laser-focused: no AST logic, no evaluation, just raw token production. Once stable,
  you can later investigate libraries (logos, nom-tokenize) to compare ergonomics, but writing it yourself first gives
  you valuable experience with Rust’s borrowing and pattern matching.
  
  [ ] Parser: keep nom (rlisp/repl/src/parser.rs:12) for complex forms; introduce custom error types via thiserror so parse failures are informative. Unit-test with
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
