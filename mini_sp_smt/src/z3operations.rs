//! Z3 numerical operations for SP

use z3_sys::*;
use super::*;

pub struct MULZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub args: Vec<Z3_ast>,
    pub r: Z3_ast
}

pub struct DIVZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub arg1: Z3_ast,
    pub arg2: Z3_ast,
    pub r: Z3_ast
}

pub struct MODZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub arg1: Z3_ast,
    pub arg2: Z3_ast,
    pub r: Z3_ast
}

pub struct REMZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub arg1: Z3_ast,
    pub arg2: Z3_ast,
    pub r: Z3_ast
}

pub struct ADDZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub args: Vec<Z3_ast>,
    pub r: Z3_ast
}

pub struct SUBZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub args: Vec<Z3_ast>,
    pub r: Z3_ast
}

pub struct NEGZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub arg: Z3_ast,
    pub r: Z3_ast
}

pub struct POWZ3<'ctx> {
    pub ctx: &'ctx ContextZ3,
    pub arg1: Z3_ast,
    pub arg2: Z3_ast,
    pub r: Z3_ast
}

impl<'ctx> MULZ3<'ctx> {
    /// Create an AST node representing `args[0] * ... * args[num_args-1]`.
    ///
    /// The `args` is a rust vector, use `vec!()`.
    /// All arguments must have int or real sort.
    ///
    /// NOTE: Z3 has limited support for non-linear arithmetic.
    /// 
    /// NOTE: The number of arguments must be greater than zero.
    /// 
    /// NOTE: See macro! mul_z3!
    pub fn new(ctx: &'ctx ContextZ3, args: Vec<Z3_ast>) -> Z3_ast {
        let args_slice = &args;
        let z3 = unsafe {
            Z3_mk_mul(ctx.r, args_slice.len() as u32, args_slice.as_ptr())
        };
        MULZ3 {ctx, r: z3, args}.r
    }
}

impl <'ctx> DIVZ3<'ctx> {
    /// Create an AST node representing `arg1 div arg2`.
    ///
    /// NOTE: The arguments must either both have int type or both have real type.
    /// If the arguments have int type, then the result type is an int type, otherwise the
    /// the result type is real.
    /// 
    /// NOTE: See macro! div_z3!
    pub fn new(ctx: &'ctx ContextZ3, arg1: Z3_ast, arg2: Z3_ast) -> Z3_ast {
        let z3 = unsafe {
            Z3_mk_div(ctx.r, arg1, arg2)
        };
        DIVZ3 {ctx, r: z3, arg1, arg2}.r
    }
}

impl <'ctx> MODZ3<'ctx> {
    /// Create an AST node representing `arg1 mod arg2`.
    ///
    /// NOTE: The arguments must have int type.
    /// 
    /// NOTE: See macro! modz3!
    pub fn new(ctx: &'ctx ContextZ3, arg1: Z3_ast, arg2: Z3_ast) -> Z3_ast {
        let z3 = unsafe {
            Z3_mk_mod(ctx.r, arg1, arg2)
        };
        MODZ3 {ctx, arg1, arg2, r: z3}.r
    }
}

impl <'ctx> REMZ3<'ctx> {
    /// Create an AST node representing `arg1 rem arg2`.
    ///
    /// NOTE: The arguments must have int type.
    /// 
    /// NOTE: See macro! rem_z3!
    pub fn new(ctx: &'ctx ContextZ3, arg1: Z3_ast, arg2: Z3_ast) -> Z3_ast {
        let z3 = unsafe {
            Z3_mk_rem(ctx.r, arg1, arg2)
        };
        REMZ3 {ctx, arg1, arg2, r: z3}.r
    }
}

impl<'ctx> ADDZ3<'ctx> {
    /// Create an AST node representing `args[0] + ... + args[num_args-1]`.
    ///
    /// The `args` is a rust vector, use `vec!()`.
    /// 
    /// NOTE: All arguments must have int or real sort.
    ///
    /// NOTE: The number of arguments must be greater than zero.
    /// 
    /// NOTE: See macro! add_z3!
    pub fn new(ctx: &'ctx ContextZ3, args: Vec<Z3_ast>) -> Z3_ast {
        let args_slice = &args;
        let z3 = unsafe {
            Z3_mk_add(ctx.r, args_slice.len() as u32, args_slice.as_ptr())
        };
        ADDZ3 {ctx, r: z3, args}.r
    }
}

impl<'ctx> SUBZ3<'ctx> {
    /// Create an AST node representing `args[0] - ... - args[num_args - 1]`.
    ///
    /// The `args` is a rust vector, use `vec!()`.
    /// 
    /// NOTE: All arguments must have int or real sort.
    ///
    /// NOTE: The number of arguments must be greater than zero.
    /// 
    /// NOTE: See macro! sub_z3!
    pub fn new(ctx: &'ctx ContextZ3, args: Vec<Z3_ast>) -> Z3_ast {
        let args_slice = &args;
        let z3 = unsafe {
            Z3_mk_sub(ctx.r, args_slice.len() as u32, args_slice.as_ptr())
        };
        SUBZ3 {ctx, r: z3, args}.r 
    }
}

impl<'ctx> NEGZ3<'ctx> {
    /// Create an AST node representing `- arg`.
    ///
    /// NOTE: The arguments must have int or real type.
    /// 
    /// NOTE: See macro! neg_z3!
    pub fn new(ctx: &'ctx ContextZ3, arg: Z3_ast) -> Z3_ast {
        let z3 = unsafe {
            Z3_mk_unary_minus(ctx.r, arg)
        };
        NEGZ3 {ctx, r: z3, arg}.r  
    }
}

impl <'ctx> POWZ3<'ctx> {
    /// Create an AST node representing `arg1 ^ arg2`.
    ///
    /// NOTE: The arguments must have int or real type.
    /// 
    /// NOTE: See macro! pow_z3!
    pub fn new(ctx: &'ctx ContextZ3, arg1: Z3_ast, arg2: Z3_ast) -> Z3_ast {
        let z3 = unsafe {
            Z3_mk_power(ctx.r, arg1, arg2)
        };
        POWZ3 {ctx, arg1, arg2, r: z3}.r
    }
}

/// a * b * c * ...
/// 
/// Macro rule for:
/// ```text
/// z3operations::MULZ3::new(&ctx, vec!(a, b, c)).r
/// ```
/// Using a specific context and passing elements:
/// ```text
/// mul_z3!(&ctx, a, b, c)
/// ```
/// Or make a vector firts and pass it:
/// ```text
/// let some = vec!(a, b, c);
/// mul_z3!(&ctx, some)
/// ```
/// Requires that a, b, c... are of Real or Int sort.
#[macro_export]
macro_rules! mul_z3 {
    ($ctx:expr, $b:expr) => {
        MULZ3::new($ctx, $b)
    };
    ( $ctx:expr, $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            MULZ3::new($ctx, temp_vec)
        }
    };
}

/// a div b
/// 
/// Macro rule for:
/// ```text
/// z3operations::DIVZ3::new(&ctx, a, b).r
/// ```
/// Using a specific context:
/// ```text
/// div_z3!(&ctx, a, b)
/// ```
/// Requires that a and b are of Real or Int sort.
#[macro_export]
macro_rules! div_z3 {
    ($ctx:expr, $b:expr, $c:expr) => {
        DIVZ3::new($ctx, $b, $c)
    }
}

/// a mod b
/// 
/// Macro rule for:
/// ```text
/// z3operations::MODZ3::new(&ctx, a, b).r
/// ```
/// Using a specific context:
/// ```text
/// mod_z3!(&ctx, a, b)
/// ```
/// Requires that a and b are of Real or Int sort.
#[macro_export]
macro_rules! mod_z3 {
    ($ctx:expr, $b:expr, $c:expr) => {
        MODZ3::new($ctx, $b, $c)
    }
}

/// a rem b
/// 
/// Macro rule for:
/// ```text
/// z3operations::REMZ3::new(&ctx, a, b).r
/// ```
/// Using a specific context:
/// ```text
/// rem_z3!(&ctx, a, b)
/// ```
/// Requires that a and b are of Real or Int sort.
#[macro_export]
macro_rules! rem_z3 {
    ($ctx:expr, $b:expr, $c:expr) => {
        REMZ3::new($ctx, $b, $c)
    }
}

/// a + b + c + ...
/// 
/// Macro rule for:
/// ```text
/// z3operations::ADDZ3::new(&ctx, vec!(a, b, c)).r
/// ```
/// Using a specific context and passing elements:
/// ```text
/// add_z3!(&ctx, a, b, c)
/// ```
/// Or make a vector firts and pass it:
/// ```text
/// let some = vec!(a, b, c);
/// add_z3!(&ctx, some)
/// ```
/// Requires that a, b, c... are of Real or Int sort.
#[macro_export]
macro_rules! add_z3 {
    ($ctx:expr, $b:expr) => {
        ADDZ3::new($ctx, $b)
    };
    ( $ctx:expr, $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            ADDZ3::new($ctx, temp_vec)
        }
    };
}

/// a - b - c - ...
/// 
/// Macro rule for:
/// ```text
/// z3operations::SUBZ3::new(&ctx, vec!(a, b, c)).r
/// ```
/// Using a specific context and passing elements:
/// ```text
/// sub_z3!(&ctx, a, b, c)
/// ```
/// Or make a vector firts and pass it:
/// ```text
/// let some = vec!(a, b, c);
/// sub_z3!(&ctx, some)
/// ```
/// Requires that a, b, c... are of Real or Int sort.
#[macro_export]
macro_rules! sub_z3 {
    ($ctx:expr, $b:expr) => {
        SUBZ3::new($ctx, $b)
    };
    ( $ctx:expr, $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            SUBZ3::new($ctx, temp_vec)
        }
    };
}

/// -a
/// 
/// Macro rule for:
/// ```text
/// z3operations::NEGZ3::new(&ctx, a).r
/// ```
/// Using a specific context:
/// ```text
/// neg_z3!(&ctx, a)
/// ```
/// Requires that a is of Real or Int sort.
#[macro_export]
macro_rules! neg_z3 {
    ($ctx:expr, $b:expr) => {
        NEGZ3::new($ctx, $b)
    }
}

/// a ^ b
/// 
/// Macro rule for:
/// ```text
/// z3operations::POWZ3::new(&ctx, a, b).r
/// ```
/// Using a specific context:
/// ```text
/// pow_z3!(&ctx, a, b)
/// ```
/// Requires that a and b are of Real or Int sort.
#[macro_export]
macro_rules! pow_z3 {
    ($ctx:expr, $b:expr, $c:expr) => {
        POWZ3::new($ctx, $b, $c)
    }
}

#[test]
fn test_new_mul(){
    let conf = ConfigZ3::new();
    let ctx = ContextZ3::new(&conf);
    let intsort = IntSortZ3::new(&ctx);
    let realsort = RealSortZ3::new(&ctx);

    let int1 = IntZ3::new(&ctx, &intsort, 7);
    let int2 = IntZ3::new(&ctx, &intsort, -1012);
    let real1 = RealZ3::new(&ctx, &realsort, 7.361928);
    let real2 = RealZ3::new(&ctx, &realsort, -236.098364);

    let x1 = IntVarZ3::new(&ctx, &intsort, "x1");
    let x2 = IntVarZ3::new(&ctx, &intsort, "x2");
    let x3 = RealVarZ3::new(&ctx, &realsort, "x3");
    let x4 = RealVarZ3::new(&ctx, &realsort, "x4");

    let mul1 = MULZ3::new(&ctx, vec!(x1, x2, x3, x4, int1, int2, real1, real2));

    assert_eq!("(* (to_real x1)
   (to_real x2)
   x3
   x4
   (to_real 7)
   (to_real (- 1012))
   (/ 920241.0 125000.0)
   (- (/ 59024591.0 250000.0)))", ast_to_string_z3!(&ctx, mul1));
}

#[test]
fn test_new_div(){
    let conf = ConfigZ3::new();
    let ctx = ContextZ3::new(&conf);
    let intsort = IntSortZ3::new(&ctx);
    let realsort = RealSortZ3::new(&ctx);

    let int1 = IntZ3::new(&ctx, &intsort, 7);
    let int2 = IntZ3::new(&ctx, &intsort, -1012);
    let real1 = RealZ3::new(&ctx, &realsort, 7.361928);
    let real2 = RealZ3::new(&ctx, &realsort, -236.098364);

    let x1 = IntVarZ3::new(&ctx, &intsort, "x1");
    let x3 = RealVarZ3::new(&ctx, &realsort, "x3");

    let div1 = DIVZ3::new(&ctx, int1, int2);
    let div2 = DIVZ3::new(&ctx, int1, real2);
    let div3 = DIVZ3::new(&ctx, x1, real2);
    let div4 = DIVZ3::new(&ctx, x3, int1);
    let div5 = DIVZ3::new(&ctx, x3, real1);

    assert_eq!("(div 7 (- 1012))", ast_to_string_z3!(&ctx, div1));
    assert_eq!("(div 7 (to_int (- (/ 59024591.0 250000.0))))", ast_to_string_z3!(&ctx, div2));
    assert_eq!("(div x1 (to_int (- (/ 59024591.0 250000.0))))", ast_to_string_z3!(&ctx, div3));
    assert_eq!("(/ x3 (to_real 7))", ast_to_string_z3!(&ctx, div4));
    assert_eq!("(/ x3 (/ 920241.0 125000.0))", ast_to_string_z3!(&ctx, div5));
}

#[test]
fn test_new_mod(){
    let conf = ConfigZ3::new();
    let ctx = ContextZ3::new(&conf);
    let intsort = IntSortZ3::new(&ctx);

    let int1 = IntZ3::new(&ctx, &intsort, 7);
    let int2 = IntZ3::new(&ctx, &intsort, -1012);

    let x1 = IntVarZ3::new(&ctx, &intsort, "x1");
    let x2 = IntVarZ3::new(&ctx, &intsort, "x2");

    let mod1 = MODZ3::new(&ctx, int1, int2);
    let mod2 = MODZ3::new(&ctx, x1, x2);
    let mod3 = MODZ3::new(&ctx, x1, int1);
    let mod4 = MODZ3::new(&ctx, int2, x2);

    assert_eq!("(mod 7 (- 1012))", ast_to_string_z3!(&ctx, mod1));
    assert_eq!("(mod x1 x2)", ast_to_string_z3!(&ctx, mod2));
    assert_eq!("(mod x1 7)", ast_to_string_z3!(&ctx, mod3));
    assert_eq!("(mod (- 1012) x2)", ast_to_string_z3!(&ctx, mod4));
}

#[test]
fn test_new_rem(){
    let conf = ConfigZ3::new();
    let ctx = ContextZ3::new(&conf);
    let intsort = IntSortZ3::new(&ctx);

    let int1 = IntZ3::new(&ctx, &intsort, 7);
    let int2 = IntZ3::new(&ctx, &intsort, -1012);

    let x1 = IntVarZ3::new(&ctx, &intsort, "x1");
    let x2 = IntVarZ3::new(&ctx, &intsort, "x2");

    let rem1 = REMZ3::new(&ctx, int1, int2);
    let rem2 = REMZ3::new(&ctx, x1, x2);
    let rem3 = REMZ3::new(&ctx, x1, int1);
    let rem4 = REMZ3::new(&ctx, int2, x2);

    assert_eq!("(rem 7 (- 1012))", ast_to_string_z3!(&ctx, rem1));
    assert_eq!("(rem x1 x2)", ast_to_string_z3!(&ctx, rem2));
    assert_eq!("(rem x1 7)", ast_to_string_z3!(&ctx, rem3));
    assert_eq!("(rem (- 1012) x2)", ast_to_string_z3!(&ctx, rem4));
}

#[test]
fn test_new_add(){
    let conf = ConfigZ3::new();
    let ctx = ContextZ3::new(&conf);
    let intsort = IntSortZ3::new(&ctx);
    let realsort = RealSortZ3::new(&ctx);

    let int1 = IntZ3::new(&ctx, &intsort, 7);
    let int2 = IntZ3::new(&ctx, &intsort, -1012);
    let real1 = RealZ3::new(&ctx, &realsort, 7.361928);
    let real2 = RealZ3::new(&ctx, &realsort, -236.098364);

    let x1 = IntVarZ3::new(&ctx, &intsort, "x1");
    let x2 = IntVarZ3::new(&ctx, &intsort, "x2");
    let x3 = RealVarZ3::new(&ctx, &realsort, "x3");
    let x4 = RealVarZ3::new(&ctx, &realsort, "x4");

    let add1 = ADDZ3::new(&ctx, vec!(x1, x2, x3, x4, int1, int2, real1, real2));

    assert_eq!("(+ (to_real x1)
   (to_real x2)
   x3
   x4
   (to_real 7)
   (to_real (- 1012))
   (/ 920241.0 125000.0)
   (- (/ 59024591.0 250000.0)))", ast_to_string_z3!(&ctx, add1));
}

#[test]
fn test_new_sub(){
    let conf = ConfigZ3::new();
    let ctx = ContextZ3::new(&conf);
    let intsort = IntSortZ3::new(&ctx);
    let realsort = RealSortZ3::new(&ctx);

    let int1 = IntZ3::new(&ctx, &intsort, 7);
    let int2 = IntZ3::new(&ctx, &intsort, -1012);
    let real1 = RealZ3::new(&ctx, &realsort, 7.361928);
    let real2 = RealZ3::new(&ctx, &realsort, -236.098364);

    let x1 = IntVarZ3::new(&ctx, &intsort, "x1");
    let x2 = IntVarZ3::new(&ctx, &intsort, "x2");
    let x3 = RealVarZ3::new(&ctx, &realsort, "x3");
    let x4 = RealVarZ3::new(&ctx, &realsort, "x4");

    let sub1 = SUBZ3::new(&ctx, vec!(x1, x2, x3, x4, int1, int2, real1, real2));

    assert_eq!("(- (- (- (- (- (- (to_real (- x1 x2)) x3) x4) (to_real 7)) (to_real (- 1012)))
      (/ 920241.0 125000.0))
   (- (/ 59024591.0 250000.0)))", ast_to_string_z3!(&ctx, sub1));
}

#[test]
fn test_new_neg(){
    let conf = ConfigZ3::new();
    let ctx = ContextZ3::new(&conf);
    let intsort = IntSortZ3::new(&ctx);
    let realsort = RealSortZ3::new(&ctx);

    let int1 = IntZ3::new(&ctx, &intsort, 7);
    let int2 = IntZ3::new(&ctx, &intsort, -1012);
    let real1 = RealZ3::new(&ctx, &realsort, 7.361928);
    let real2 = RealZ3::new(&ctx, &realsort, -236.098364);

    let x1 = IntVarZ3::new(&ctx, &intsort, "x1");

    let neg1 = NEGZ3::new(&ctx, int1);
    let neg2 = NEGZ3::new(&ctx, int2);
    let neg3 = NEGZ3::new(&ctx, real1);
    let neg4 = NEGZ3::new(&ctx, real2);
    let neg5 = NEGZ3::new(&ctx, x1);

    assert_eq!("(- 7)", ast_to_string_z3!(&ctx, neg1));
    assert_eq!("(- (- 1012))", ast_to_string_z3!(&ctx, neg2));
    assert_eq!("(- (/ 920241.0 125000.0))", ast_to_string_z3!(&ctx, neg3));
    assert_eq!("(- (- (/ 59024591.0 250000.0)))", ast_to_string_z3!(&ctx, neg4));
    assert_eq!("(- x1)", ast_to_string_z3!(&ctx, neg5));
}

#[test]
fn test_new_pow(){
    let conf = ConfigZ3::new();
    let ctx = ContextZ3::new(&conf);
    let intsort = IntSortZ3::new(&ctx);

    let int1 = IntZ3::new(&ctx, &intsort, 7);
    let int2 = IntZ3::new(&ctx, &intsort, -1012);

    let x1 = IntVarZ3::new(&ctx, &intsort, "x1");
    let x2 = IntVarZ3::new(&ctx, &intsort, "x2");

    let pow1 = POWZ3::new(&ctx, int1, int2);
    let pow2 = POWZ3::new(&ctx, x1, x2);
    let pow3 = POWZ3::new(&ctx, x1, int1);
    let pow4 = POWZ3::new(&ctx, int2, x2);

    assert_eq!("(^ 7 (- 1012))", ast_to_string_z3!(&ctx, pow1));
    assert_eq!("(^ x1 x2)", ast_to_string_z3!(&ctx, pow2));
    assert_eq!("(^ x1 7)", ast_to_string_z3!(&ctx, pow3));
    assert_eq!("(^ (- 1012) x2)", ast_to_string_z3!(&ctx, pow4));
}

#[test]
fn test_mul_macro_1(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let some = vec!(
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142),
        int_var_z3!(&ctx, "y")
    );
    let mul1 = mul_z3!(&ctx, some);
    assert_eq!("(* x 142 y)", ast_to_string_z3!(&ctx, mul1));
}

#[test]
fn test_mul_macro_2(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let mul1 = mul_z3!(&ctx,
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142),
        int_var_z3!(&ctx, "y")
    );
    assert_eq!("(* x 142 y)", ast_to_string_z3!(&ctx, mul1));
}

#[test]
fn test_div_macro_1(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let div1 = div_z3!(&ctx,
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142)
    );
    assert_eq!("(div x 142)", ast_to_string_z3!(&ctx, div1));
}

#[test]
fn test_mod_macro_1(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let mod1 = mod_z3!(&ctx,
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142)
    );
    assert_eq!("(mod x 142)", ast_to_string_z3!(&ctx, mod1));
}

#[test]
fn test_rem_macro_1(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let rem1 = rem_z3!(&ctx,
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142)
    );
    assert_eq!("(rem x 142)", ast_to_string_z3!(&ctx, rem1));
}

#[test]
fn test_add_macro_1(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let some = vec!(
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142),
        int_var_z3!(&ctx, "y"),
        int_z3!(&ctx, 1213442)
    );
    let add1 = add_z3!(&ctx, some);
    assert_eq!("(+ x 142 y 1213442)", ast_to_string_z3!(&ctx, add1));
}

#[test]
fn test_add_macro_2(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let add1 = add_z3!(&ctx, 
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142),
        int_var_z3!(&ctx, "y"),
        int_z3!(&ctx, 1213442)
    );
    assert_eq!("(+ x 142 y 1213442)", ast_to_string_z3!(&ctx, add1));
}

#[test]
fn test_sub_macro_1(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let some = vec!(
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142),
        int_var_z3!(&ctx, "y"),
        int_z3!(&ctx, 1213442)
    );
    let sub1 = sub_z3!(&ctx, some);
    assert_eq!("(- (- (- x 142) y) 1213442)", ast_to_string_z3!(&ctx, sub1));
}

#[test]
fn test_sub_macro_2(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let sub1 = sub_z3!(&ctx, 
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142),
        int_var_z3!(&ctx, "y"),
        int_z3!(&ctx, 1213442)
    );
    assert_eq!("(- (- (- x 142) y) 1213442)", ast_to_string_z3!(&ctx, sub1));
}

#[test]
fn test_neg_macro_1(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let neg1 = neg_z3!(&ctx,
        int_z3!(&ctx, 1213442)
    );
    assert_eq!("(- 1213442)", ast_to_string_z3!(&ctx, neg1));
}

#[test]
fn test_pow_macro_1(){
    let cfg = cfg_z3!();
    let ctx = ctx_z3!(&cfg);
    let pow1 = pow_z3!(&ctx, 
        int_var_z3!(&ctx, "x"),
        int_z3!(&ctx, 142)
    );
    assert_eq!("(^ x 142)", ast_to_string_z3!(&ctx, pow1));
}