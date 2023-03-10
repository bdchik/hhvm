// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
use std::ops::ControlFlow;

use hash::HashSet;
use naming_special_names_rust as sn;
use oxidized::naming_error::NamingError;
use oxidized::nast::Class_;
use oxidized::nast::FunDef;
use oxidized::nast::Gconst;
use oxidized::nast::Hint;
use oxidized::nast::Hint_;
use oxidized::nast::Id;
use oxidized::nast::Method_;
use oxidized::nast::ModuleDef;
use oxidized::nast::Pos;
use oxidized::nast::Tparam;
use oxidized::nast::Tprim;
use oxidized::nast::Typedef;

use crate::env::Env;
use crate::Pass;

#[derive(Clone, Default)]
pub struct ElabHintHapplyPass {
    tparams: HashSet<String>,
}

impl ElabHintHapplyPass {
    pub fn extend_tparams(&mut self, tps: &[Tparam]) {
        tps.iter().for_each(|tparam| {
            self.tparams.insert(tparam.name.1.clone());
        })
    }
    pub fn reset_tparams(&mut self) {
        self.tparams.clear()
    }
    pub fn set_tparams(&mut self, tps: &[Tparam]) {
        self.reset_tparams();
        self.extend_tparams(tps);
    }
    pub fn tparams(&self) -> &HashSet<String> {
        &self.tparams
    }
}

impl Pass for ElabHintHapplyPass {
    // We can't write this - how can we make the contexts modular?
    // type Ctx = impl CanonicalHapplyCtx;

    fn on_ty_typedef_top_down(&mut self, elem: &mut Typedef, _env: &Env) -> ControlFlow<()> {
        self.set_tparams(&elem.tparams);
        ControlFlow::Continue(())
    }

    fn on_ty_gconst_top_down(&mut self, _elem: &mut Gconst, _env: &Env) -> ControlFlow<()> {
        self.reset_tparams();
        ControlFlow::Continue(())
    }

    fn on_ty_fun_def_top_down(&mut self, elem: &mut FunDef, _env: &Env) -> ControlFlow<()> {
        self.set_tparams(&elem.fun.tparams);
        ControlFlow::Continue(())
    }

    fn on_ty_module_def_top_down(&mut self, _elem: &mut ModuleDef, _env: &Env) -> ControlFlow<()> {
        self.reset_tparams();
        ControlFlow::Continue(())
    }

    fn on_ty_class__top_down(&mut self, elem: &mut Class_, _env: &Env) -> ControlFlow<()> {
        self.set_tparams(&elem.tparams);
        ControlFlow::Continue(())
    }

    fn on_ty_method__top_down(&mut self, elem: &mut Method_, _env: &Env) -> ControlFlow<()> {
        self.extend_tparams(&elem.tparams);
        ControlFlow::Continue(())
    }

    fn on_ty_tparam_top_down(&mut self, elem: &mut Tparam, _env: &Env) -> ControlFlow<()> {
        self.extend_tparams(&elem.parameters);
        ControlFlow::Continue(())
    }

    fn on_ty_hint_top_down(&mut self, elem: &mut Hint, env: &Env) -> ControlFlow<()> {
        match &mut *elem.1 {
            Hint_::Happly(id, hints) => match canonical_happly(id, hints, self.tparams()) {
                ControlFlow::Continue((hint_opt, err_opt)) => {
                    if let Some(hint_) = hint_opt {
                        *elem.1 = hint_
                    }
                    if let Some(err) = err_opt {
                        env.emit_error(err)
                    }
                    ControlFlow::Continue(())
                }
                ControlFlow::Break((hint_opt, err)) => {
                    if let Some(hint_) = hint_opt {
                        *elem.1 = hint_
                    }
                    env.emit_error(err);
                    ControlFlow::Break(())
                }
            },
            _ => ControlFlow::Continue(()),
        }
    }
}

enum CanonResult {
    Concrete(Hint_),
    This,
    Classname,
    Wildcard,
    Tycon,
    Typaram,
    Varray,
    Darray,
    VecOrDict,
    ErrPrimTopLevel,
}

// Find the canonical representation of an `Happly` hint; if only some _part_
// of the hint needs modifying, we mutate here. If the whole hint should be
// replaced, we return a `Hint_` and replace in the calling function
fn canonical_happly(
    id: &mut Id,
    hints: &mut Vec<Hint>,
    tparams: &HashSet<String>,
) -> ControlFlow<(Option<Hint_>, NamingError), (Option<Hint_>, Option<NamingError>)> {
    match canonical_tycon(id, tparams) {
        // The type constructors canonical representation _is_ `Happly`
        CanonResult::Tycon => ControlFlow::Continue((None, None)),
        // The type constructors canonical representation is a concrete type
        CanonResult::Concrete(hint_canon) => {
            // We can't represent a concrete type applied to other types
            let err_opt = if hints.is_empty() {
                None
            } else {
                Some(NamingError::UnexpectedTypeArguments(id.0.clone()))
            };
            ControlFlow::Continue((Some(hint_canon), err_opt))
        }
        // The type constructors corresponds to an in-scope type parameter
        CanonResult::Typaram => {
            // TODO: is this clone necessary? We are replacing the hint which
            // contains them
            let mut nm_canon = String::default();
            let mut hints_canon = vec![];
            std::mem::swap(&mut id.1, &mut nm_canon);
            std::mem::swap(hints, &mut hints_canon);
            let hint_canon = Hint_::Habstr(nm_canon.to_string(), hints_canon);
            ControlFlow::Continue((Some(hint_canon), None))
        }
        // The type constructors canonical representation is `Happly` but
        // additional elaboration / validation is required
        CanonResult::This => {
            let hint_ = Hint_::Hthis;
            let err_opt = if hints.is_empty() {
                None
            } else {
                Some(NamingError::ThisNoArgument(id.0.clone()))
            };
            ControlFlow::Continue((Some(hint_), err_opt))
        }
        CanonResult::Wildcard => {
            if hints.is_empty() {
                ControlFlow::Continue((None, None))
            } else {
                let err = NamingError::ThisNoArgument(id.0.clone());
                let hint_ = Hint_::Herr;
                ControlFlow::Break((Some(hint_), err))
            }
        }
        CanonResult::Classname => {
            // TODO[mjt] currently if `classname` is not applied to exactly
            // one type parameter, it canonicalizes to `Hprim Tstring`.
            // Investigate why this happens and if we can delay treatment to
            // typing
            if hints.len() == 1 {
                ControlFlow::Continue((None, None))
            } else {
                let err = NamingError::ClassnameParam(id.0.clone());
                let hint_ = Hint_::Hprim(Tprim::Tstring);
                ControlFlow::Break((Some(hint_), err))
            }
        }
        CanonResult::ErrPrimTopLevel => {
            let hint_ = Hint_::Herr;
            let err = NamingError::PrimitiveTopLevel(id.0.clone());
            ControlFlow::Break((Some(hint_), err))
        }
        // TODO[mjt] we should not be assuming knowledge about the arity of
        // type constructors during elaboration
        CanonResult::Darray if hints.len() == 2 => {
            id.1 = sn::collections::DICT.to_string();
            ControlFlow::Continue((None, None))
        }
        CanonResult::Darray if hints.len() < 2 => {
            id.1 = sn::collections::DICT.to_string();
            hints.clear();
            hints.push(Hint(id.0.clone(), Box::new(Hint_::Hany)));
            hints.push(Hint(id.0.clone(), Box::new(Hint_::Hany)));
            let err = NamingError::TooFewTypeArguments(id.0.clone());
            ControlFlow::Continue((None, Some(err)))
        }
        CanonResult::Darray => {
            let hint_ = Hint_::Hany;
            let err = NamingError::TooManyTypeArguments(id.0.clone());
            ControlFlow::Break((Some(hint_), err))
        }
        CanonResult::Varray if hints.len() == 1 => {
            id.1 = sn::collections::VEC.to_string();
            ControlFlow::Continue((None, None))
        }
        CanonResult::Varray if hints.is_empty() => {
            id.1 = sn::collections::VEC.to_string();
            hints.clear();
            hints.push(Hint(id.0.clone(), Box::new(Hint_::Hany)));
            let err = NamingError::TooFewTypeArguments(id.0.clone());
            ControlFlow::Continue((None, Some(err)))
        }
        CanonResult::Varray => {
            let hint_ = Hint_::Hany;
            let err = NamingError::TooManyTypeArguments(id.0.clone());
            ControlFlow::Break((Some(hint_), err))
        }
        CanonResult::VecOrDict if hints.len() > 2 => {
            let hint_ = Hint_::Hany;
            let err = NamingError::TooManyTypeArguments(id.0.clone());
            ControlFlow::Break((Some(hint_), err))
        }
        CanonResult::VecOrDict => match hints.pop() {
            None => {
                let mut pos_canon = Pos::NONE;
                std::mem::swap(&mut id.0, &mut pos_canon);
                let hint_ = Hint_::HvecOrDict(None, Hint(pos_canon, Box::new(Hint_::Hany)));
                let err = NamingError::TooFewTypeArguments(id.0.clone());
                ControlFlow::Continue((Some(hint_), Some(err)))
            }
            Some(hint2) => {
                if let Some(hint1) = hints.pop() {
                    let hint_ = Hint_::HvecOrDict(Some(hint1), hint2);
                    ControlFlow::Continue((Some(hint_), None))
                } else {
                    let hint_ = Hint_::HvecOrDict(None, hint2);
                    ControlFlow::Continue((Some(hint_), None))
                }
            }
        },
    }
}

fn canonical_tycon(id: &Id, tparams: &HashSet<String>) -> CanonResult {
    if id.1 == sn::typehints::INT {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tint))
    } else if id.1 == sn::typehints::BOOL {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tbool))
    } else if id.1 == sn::typehints::FLOAT {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tfloat))
    } else if id.1 == sn::typehints::STRING {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tstring))
    } else if id.1 == sn::typehints::VOID {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tvoid))
    } else if id.1 == sn::typehints::NORETURN {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tnoreturn))
    } else if id.1 == sn::typehints::NULL {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tnull))
    } else if id.1 == sn::typehints::RESOURCE {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tresource))
    } else if id.1 == sn::typehints::ARRAYKEY {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tarraykey))
    } else if id.1 == sn::typehints::NUM {
        CanonResult::Concrete(Hint_::Hprim(Tprim::Tnum))
    } else if id.1 == sn::typehints::DARRAY {
        CanonResult::Darray
    } else if id.1 == sn::typehints::VARRAY {
        CanonResult::Varray
    } else if id.1 == sn::typehints::VARRAY_OR_DARRAY || id.1 == sn::typehints::VEC_OR_DICT {
        // TODO[mjt] `vec_or_dict` is currently special cased since the canonical representation
        // requires us to have no arity mismatches or throw away info. We do not use that repr here
        // to avoid having to do so. Ultimately, we should remove that special case
        CanonResult::VecOrDict
    } else if id.1 == sn::typehints::MIXED {
        CanonResult::Concrete(Hint_::Hmixed)
    } else if id.1 == sn::typehints::NONNULL {
        CanonResult::Concrete(Hint_::Hnonnull)
    } else if id.1 == sn::typehints::NOTHING {
        CanonResult::Concrete(Hint_::Hnothing)
    } else if id.1 == sn::typehints::DYNAMIC {
        CanonResult::Concrete(Hint_::Hdynamic)
    } else if id.1 == sn::typehints::THIS {
        CanonResult::This
    } else if id.1 == sn::typehints::WILDCARD {
        CanonResult::Wildcard
    } else if id.1 == sn::classes::CLASS_NAME {
        CanonResult::Classname
    } else if tparams.contains(&id.1) {
        CanonResult::Typaram
    } else if is_toplevel_prim(&id.1) {
        // TODO: this should be a separate validation pass
        CanonResult::ErrPrimTopLevel
    } else {
        CanonResult::Tycon
    }
}

fn is_toplevel_prim(str: &str) -> bool {
    if let Some(substr) = str.strip_prefix('\\') {
        return is_prim(substr);
    }
    false
}

fn is_prim(str: &str) -> bool {
    str == sn::typehints::NULL
        || str == sn::typehints::NORETURN
        || str == sn::typehints::INT
        || str == sn::typehints::BOOL
        || str == sn::typehints::FLOAT
        || str == sn::typehints::NUM
        || str == sn::typehints::STRING
        || str == sn::typehints::RESOURCE
        || str == sn::typehints::MIXED
        || str == sn::typehints::NONNULL
        || str == sn::typehints::ARRAYKEY
        || str == sn::typehints::NOTHING
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Transform;

    #[test]
    fn test_vec_or_dict_two_tyargs() {
        let env = Env::default();

        let mut pass = ElabHintHapplyPass::default();

        let mut elem = Hint(
            Pos::NONE,
            Box::new(Hint_::Happly(
                Id(Pos::NONE, sn::typehints::VEC_OR_DICT.to_string()),
                vec![
                    Hint(Pos::NONE, Box::new(Hint_::Hmixed)),
                    Hint(Pos::NONE, Box::new(Hint_::Hnothing)),
                ],
            )),
        );

        elem.transform(&env, &mut pass);
        let Hint(_, hint_) = elem;
        assert!(match &*hint_ {
            Hint_::HvecOrDict(Some(h1), h2) => {
                let Hint(_, h1_) = h1 as &Hint;
                let Hint(_, h2_) = h2 as &Hint;
                matches!(**h1_, Hint_::Hmixed) && matches!(**h2_, Hint_::Hnothing)
            }
            _ => false,
        })
    }

    #[test]
    fn test_vec_or_dict_one_tyargs() {
        let env = Env::default();

        let mut pass = ElabHintHapplyPass::default();

        let mut elem = Hint(
            Pos::NONE,
            Box::new(Hint_::Happly(
                Id(Pos::NONE, sn::typehints::VEC_OR_DICT.to_string()),
                vec![Hint(Pos::NONE, Box::new(Hint_::Hnothing))],
            )),
        );

        elem.transform(&env, &mut pass);
        let Hint(_, hint_) = elem;
        assert!(match &*hint_ {
            Hint_::HvecOrDict(None, h) => {
                let Hint(_, h_) = h as &Hint;
                matches!(**h_, Hint_::Hnothing)
            }
            _ => false,
        })
    }

    #[test]
    fn test_vec_or_dict_zero_tyargs() {
        let env = Env::default();

        let mut pass = ElabHintHapplyPass::default();

        let mut elem = Hint(
            Pos::NONE,
            Box::new(Hint_::Happly(
                Id(Pos::NONE, sn::typehints::VEC_OR_DICT.to_string()),
                vec![],
            )),
        );

        elem.transform(&env, &mut pass);
        let Hint(_, hint_) = elem;
        assert!(match &*hint_ {
            Hint_::HvecOrDict(None, h) => {
                let Hint(_, h_) = h as &Hint;
                matches!(**h_, Hint_::Hany)
            }
            _ => false,
        })
    }
}
