// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use std::path::Path;

use anyhow::bail;
use anyhow::Result;

use crate::decls;
use crate::state::UnitState;
use crate::textual;

const UNIT_START_MARKER: &str = "TEXTUAL UNIT START";
const UNIT_END_MARKER: &str = "TEXTUAL UNIT END";

pub fn textual_writer(
    w: &mut dyn std::io::Write,
    path: &Path,
    mut unit: ir::Unit<'_>,
    no_builtins: bool,
) -> Result<()> {
    // steal the StringInterner so we can mutate it while reading the Unit.
    let strings = std::mem::take(&mut unit.strings);

    let escaped_path = escaper::escape(path.display().to_string());

    writeln!(w, "// {} {}", UNIT_START_MARKER, escaped_path)?;

    textual::write_attribute(w, textual::Attribute::SourceLanguage("hack".to_string()))?;
    writeln!(w)?;

    let mut state = UnitState::new(strings);
    check_fatal(path, unit.fatal.as_ref())?;

    for cls in unit.classes {
        crate::class::write_class(w, &mut state, cls)?;
    }

    for func in unit.functions {
        crate::func::write_function(w, &mut state, func)?;
    }

    writeln!(w, "// ----- EXTERNALS -----")?;
    for name in state.func_declares.external_funcs() {
        writeln!(w, "declare {name}(...): *Mixed")?;
    }

    if !no_builtins {
        decls::write_decls(w)?;
    }

    writeln!(w, "// {} {}", UNIT_END_MARKER, escaped_path)?;
    writeln!(w)?;

    Ok(())
}

fn check_fatal(path: &Path, fatal: Option<&ir::Fatal>) -> Result<()> {
    if let Some(fatal) = fatal {
        let err = match fatal.op {
            ir::FatalOp::Parse => "Parse",
            ir::FatalOp::Runtime => "Runtime",
            ir::FatalOp::RuntimeOmitFrame => "Runtime Omit",
            _ => unreachable!(),
        };

        bail!(
            "{err} error in {}[{}]: {}",
            path.display(),
            fatal.loc.line_begin,
            fatal.message
        );
    }

    Ok(())
}
