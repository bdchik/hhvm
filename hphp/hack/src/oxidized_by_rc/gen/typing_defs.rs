// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
//
// @generated SignedSource<<114e7da3db4c5bce1e66c9caa612cfd8>>
//
// To regenerate this file, run:
//   hphp/hack/src/oxidized_by_rc/regen.sh

use ocamlrep_derive::ToOcamlRep;
use serde::Serialize;

#[allow(unused_imports)]
use crate::*;

pub use typing_defs_flags::*;

pub use typing_defs_core::*;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct PuOrigin {
    pub class: std::rc::Rc<String>,
    pub enum_: std::rc::Rc<String>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct ClassElt {
    pub visibility: Visibility,
    pub type_: oxidized_by_ref::lazy::Lazy<Ty>,
    /// identifies the class from which this elt originates
    pub origin: std::rc::Rc<String>,
    pub deprecated: Option<std::rc::Rc<String>>,
    pub pos: oxidized_by_ref::lazy::Lazy<std::rc::Rc<pos::Pos>>,
    pub flags: isize,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct FunElt {
    pub deprecated: Option<std::rc::Rc<String>>,
    pub type_: Ty,
    pub decl_errors: Option<errors::Errors>,
    pub pos: std::rc::Rc<pos::Pos>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct ClassConst {
    pub synthesized: bool,
    pub abstract_: bool,
    pub pos: std::rc::Rc<pos::Pos>,
    pub type_: Ty,
    pub expr: Option<nast::Expr>,
    /// identifies the class from which this const originates
    pub origin: std::rc::Rc<String>,
}

/// The position is that of the hint in the `use` / `implements` AST node
/// that causes a class to have this requirement applied to it. E.g.
///
/// ```
/// class Foo {}
///
/// interface Bar {
///   require extends Foo; <- position of the decl_phase ty
/// }
///
/// class Baz extends Foo implements Bar { <- position of the `implements`
/// }
/// ```
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct Requirement(pub std::rc::Rc<pos::Pos>, pub Ty);

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct ClassType {
    pub need_init: bool,
    /// Whether the typechecker knows of all (non-interface) ancestors
    /// and thus known all accessible members of this class
    pub members_fully_known: bool,
    pub abstract_: bool,
    pub final_: bool,
    pub const_: bool,
    /// True when the class is annotated with the __PPL attribute.
    pub ppl: bool,
    /// When a class is abstract (or in a trait) the initialization of
    /// a protected member can be delayed
    pub deferred_init_members: s_set::SSet,
    pub kind: oxidized::ast_defs::ClassKind,
    pub is_xhp: bool,
    pub has_xhp_keyword: bool,
    pub is_disposable: bool,
    pub name: std::rc::Rc<String>,
    pub pos: std::rc::Rc<pos::Pos>,
    pub tparams: Vec<Tparam>,
    pub where_constraints: Vec<WhereConstraint>,
    pub consts: s_map::SMap<ClassConst>,
    pub typeconsts: s_map::SMap<TypeconstType>,
    pub pu_enums: s_map::SMap<PuEnumType>,
    pub props: s_map::SMap<ClassElt>,
    pub sprops: s_map::SMap<ClassElt>,
    pub methods: s_map::SMap<ClassElt>,
    pub smethods: s_map::SMap<ClassElt>,
    /// the consistent_kind represents final constructor or __ConsistentConstruct
    pub construct: (
        Option<ClassElt>,
        oxidized_by_ref::typing_defs::ConsistentKind,
    ),
    /// This includes all the classes, interfaces and traits this class is
    /// using.
    pub ancestors: s_map::SMap<Ty>,
    pub req_ancestors: Vec<Requirement>,
    /// the extends of req_ancestors
    pub req_ancestors_extends: s_set::SSet,
    pub extends: s_set::SSet,
    pub enum_type: Option<EnumType>,
    pub sealed_whitelist: Option<s_set::SSet>,
    pub decl_errors: Option<errors::Errors>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub enum TypeconstAbstractKind {
    TCAbstract(Option<Ty>),
    TCPartiallyAbstract,
    TCConcrete,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct TypeconstType {
    pub abstract_: TypeconstAbstractKind,
    pub name: nast::Sid,
    pub constraint: Option<Ty>,
    pub type_: Option<Ty>,
    pub origin: std::rc::Rc<String>,
    pub enforceable: (std::rc::Rc<pos::Pos>, bool),
    pub reifiable: Option<std::rc::Rc<pos::Pos>>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct PuEnumType {
    pub name: nast::Sid,
    pub is_final: bool,
    pub case_types: s_map::SMap<(PuOrigin, Tparam)>,
    pub case_values: s_map::SMap<(PuOrigin, nast::Sid, Ty)>,
    pub members: s_map::SMap<PuMemberType>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct PuMemberType {
    pub atom: nast::Sid,
    pub origin: PuOrigin,
    pub types: s_map::SMap<(PuOrigin, nast::Sid, Ty)>,
    pub exprs: s_map::SMap<(PuOrigin, nast::Sid)>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct EnumType {
    pub base: Ty,
    pub constraint: Option<Ty>,
}

pub use oxidized_by_ref::typing_defs::RecordFieldReq;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct RecordDefType {
    pub name: nast::Sid,
    pub extends: Option<nast::Sid>,
    pub fields: Vec<(nast::Sid, oxidized_by_ref::typing_defs::RecordFieldReq)>,
    pub abstract_: bool,
    pub pos: std::rc::Rc<pos::Pos>,
    pub errors: Option<errors::Errors>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct TypedefType {
    pub pos: std::rc::Rc<pos::Pos>,
    pub vis: oxidized::aast::TypedefVisibility,
    pub tparams: Vec<Tparam>,
    pub constraint: Option<Ty>,
    pub type_: Ty,
    pub decl_errors: Option<errors::Errors>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub enum DeserializationError {
    /// The type was valid, but some component thereof was a decl_ty when we
    /// expected a locl_phase ty, or vice versa.
    WrongPhase(std::rc::Rc<String>),
    /// The specific type or some component thereof is not one that we support
    /// deserializing, usually because not enough information was serialized to be
    /// able to deserialize it again.
    NotSupported(std::rc::Rc<String>),
    /// The input JSON was invalid for some reason.
    DeserializationError(std::rc::Rc<String>),
}
