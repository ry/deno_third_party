// compile-flags: -Z parse-only

type A = for<'a 'b> fn(); //~ ERROR expected one of `,`, `:`, or `>`, found `'b`

fn main() {}
