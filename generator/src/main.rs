mod generate;
mod isa;
mod util;

use anyhow::{Context, Result};
use isa::{Isa, OpcodeIdRegistry};
use std::{fs, path::Path};

fn main() -> Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().context("locating workspace root")?;
    let assets_dir = manifest_dir.join("assets");

    let yaml_src = fs::read_to_string(assets_dir.join("isa.yaml")).context("reading isa.yaml")?;
    let isa: Isa = serde_yaml::from_str(&yaml_src).context("parsing isa.yaml")?;
    isa.validate().context("validating isa.yaml")?;

    let registry_src = fs::read_to_string(assets_dir.join("opcode_ids.yaml"))
        .context("reading opcode_ids.yaml")?;
    let registry: OpcodeIdRegistry =
        serde_yaml::from_str(&registry_src).context("parsing opcode_ids.yaml")?;
    isa.validate_ids(&registry).context("validating opcode ids against opcode_ids.yaml")?;

    let out_dir = workspace_dir.join("disasm/src/generated");
    fs::create_dir_all(&out_dir).context("creating generated dir")?;

    write_generated(out_dir.join("types.rs"), isa.generate_types())?;
    write_generated(out_dir.join("parse.rs"), isa.generate_parse())?;
    write_generated(out_dir.join("encode.rs"), isa.generate_encode())?;
    write_generated(out_dir.join("display.rs"), isa.generate_display())?;
    write_generated(out_dir.join("effects.rs"), isa.generate_effects())?;
    let readme = fs::read(workspace_dir.join("README.md")).context("reading workspace README")?;
    write_if_changed(workspace_dir.join("disasm/README.md"), &readme)
        .context("syncing package README")?;

    println!("Generated files and the package README are synchronized in {}", out_dir.display());
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
        "    clippy::collapsible_match,\n",
        ")]\n\n"
    );
    write_if_changed(path, format!("{header}{code}").as_bytes())?;
    Ok(())
}

fn write_if_changed(path: impl AsRef<Path>, contents: &[u8]) -> Result<bool> {
    let path = path.as_ref();
    match fs::read(path) {
        Ok(existing) if existing == contents => {
            println!("  unchanged {}", path.display());
            return Ok(false);
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error).with_context(|| format!("reading {}", path.display())),
    }
    fs::write(path, contents).with_context(|| format!("writing {}", path.display()))?;
    println!("  wrote {}", path.display());
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_only_when_contents_change() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir()
            .join(format!("superh-generator-write-if-changed-{}-{unique}", std::process::id()));

        assert!(write_if_changed(&path, b"first").expect("create file"));
        assert!(!write_if_changed(&path, b"first").expect("leave matching file unchanged"));
        assert!(write_if_changed(&path, b"second").expect("update changed file"));
        assert_eq!(fs::read(&path).expect("read updated file"), b"second");

        fs::remove_file(path).expect("remove test file");
    }
}
