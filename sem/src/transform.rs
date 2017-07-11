use mir;

use Result;
use typing::{Module, Node, NodeKind, Decl, DeclKind, Def, Let};

#[derive(Debug)]
pub struct Transform {
    program: mir::Program,
}

impl Transform {
    pub fn new() -> Self {
        Transform { program: mir::Program::new() }
    }

    fn node(&mut self, node: Node) -> Result<mir::Node> {
        unimplemented!()
    }

    fn def(&mut self, def: Def) -> Result<mir::Def> {
        unimplemented!()
    }

    fn register_decl(&mut self, name: String, decl: Decl) -> Result<()> {
        match decl.kind {
            DeclKind::Def(def) => {
                let is_main = def.is_main();
                let def = self.def(*def)?;
                if is_main {
                    self.program.main = Some(Box::new(def));
                } else {
                    self.program.define(name, mir::Decl::Def(Box::new(def)));
                }
                Ok(())
            }
            DeclKind::Let(let_) => {
                let l = *let_;
                let Let {
                    name: let_name,
                    value,
                    ..
                } = l;
                let node = self.node(value)?;
                let l = mir::Let {
                    name: let_name,
                    value: node,
                };
                self.program.define(name, mir::Decl::Let(Box::new(l)));
                Ok(())
            }
        }
    }

    pub fn module(&mut self, module: Module) -> Result<()> {
        for (decl_name, decl) in module.decls.into_iter() {
            self.register_decl(decl_name, decl)?;
        }
        Ok(())
    }
}

pub fn typed_ast_to_mir(module: Module) -> Result<mir::Program> {
    let mut transform = Transform::new();
    transform.module(module)?;
    Ok(transform.program)
}
