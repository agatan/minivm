use std::rc::Rc;
use std::sync::{Once, ONCE_INIT};

use llvm::{self, Message, Context, Module, Builder, Value};
use sem::ir::*;

pub struct Compiler {
    ctx: Rc<Context>,
    module: Module,
    builder: Builder,
}

static LLVM_INIT: Once = ONCE_INIT;

impl Compiler {
    pub fn new() -> Self {
        LLVM_INIT.call_once(|| llvm::init());
        let ctx = Context::new();
        let module = Module::new(ctx.clone(), "main");
        let builder = Builder::new(ctx.clone());
        Compiler {
            ctx: ctx,
            module: module,
            builder: builder,
        }
    }

    pub fn test_run(&mut self) -> Result<(), Message> {
        let int_ty = self.ctx.int32_type();
        let fun_ty = self.ctx.function_type(int_ty, &[], false);
        let mut fun = self.module.add_function("main", fun_ty);
        let bb = fun.append_basic_block("entry");
        self.builder.position_at_end(&bb);
        let a = self.builder.alloca(int_ty, "a");
        let b = self.builder.alloca(int_ty, "b");
        let x: i32 = -10000;
        self.builder.store(int_ty.const_int(x as u64), a);
        self.builder.store(int_ty.const_int(11), b);
        let a = self.builder.load(a, "a");
        let b = self.builder.load(b, "b");
        let ret = self.builder.add(a, b, "addtmp");
        self.builder.ret(ret);
        self.module.verify()
    }

    pub fn dump_module(&self) {
        self.module.dump();
    }

    pub fn compile_node(&mut self, node: &Node) -> Value {
        match node.kind {
            NodeKind::Unit => self.unit(),
            NodeKind::Int(n) => self.int(n),
            NodeKind::Float(f) => self.float(f),
            _ => unimplemented!(),
        }
    }

    // helpers
    fn unit(&self) -> Value {
        self.ctx.unit_type().const_int(0)
    }

    fn int(&self, n: i32) -> Value {
        self.ctx.int32_type().const_int(n as ::libc::c_ulonglong)
    }

    fn float(&self, f: f64) -> Value {
        self.ctx.float_type().const_float(f)
    }
}
