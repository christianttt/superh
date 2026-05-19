mod generate;
mod isa;
mod util;

use anyhow::{Context, Result};
use isa::Isa;
use std::{fs, path::Path};

fn main() -> Result<()> {
    let yaml_src = fs::read_to_string("generator/assets/isa.yaml").context("reading isa.yaml")?;
    let isa: Isa = serde_yaml::from_str(&yaml_src).context("parsing isa.yaml")?;
    isa.validate().context("validating isa.yaml")?;

    let out_dir = Path::new("disasm/src/generated");
    fs::create_dir_all(out_dir).context("creating generated dir")?;

    write_generated(out_dir.join("types.rs"), isa.generate_types())?;
    write_generated(out_dir.join("parse.rs"), isa.generate_parse())?;
    write_generated(out_dir.join("display.rs"), isa.generate_display())?;
    write_generated(out_dir.join("defs_uses.rs"), isa.generate_defs_uses())?;

    println!("Generated 4 files in {}", out_dir.display());
    Ok(())
}

fn write_generated(path: impl AsRef<Path>, tokens: proc_macro2::TokenStream) -> Result<()> {
    let path = path.as_ref();
    let file: syn::File =
        syn::parse2(tokens).with_context(|| format!("parse2 for {}", path.display()))?;
    let code = prettyplease::unparse(&file);
    let header = concat!(
        "// @generated — do not edit by hand. Run `cargo run -p superh-generator` to regenerate.\n",
        "#![cfg_attr(rustfmt, rustfmt_skip)]\n",
        "#![allow(\n",
        "    clippy::too_many_lines,\n",
        "    clippy::missing_errors_doc,\n",
        "    clippy::must_use_candidate,\n",
        "    clippy::derivable_impls,\n",
        "    clippy::inline_always,\n",
        "    clippy::cast_lossless,\n",
        "    clippy::cast_possible_truncation,\n",
        "    clippy::cast_possible_wrap,\n",
        "    clippy::cast_sign_loss,\n",
        "    clippy::identity_op,\n",
        "    clippy::match_same_arms,\n",
        "    clippy::uninlined_format_args,\n",
        "    clippy::doc_markdown,\n",
        ")]\n\n"
    );
    fs::write(path, format!("{header}{code}"))
        .with_context(|| format!("writing {}", path.display()))?;
    println!("  wrote {}", path.display());
    Ok(())
}
