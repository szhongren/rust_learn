// concrete type
struct A;
struct S(A);
// generic type
struct SGen<T>(T);

// the following functions all take ownership of the var and free it when they go out of scope
// first 3 are not generic
fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    // explicitly specified type param
    generic::<char>(SGen('a'));
    // implicitly derived type param
    generic(SGen('c'));
}