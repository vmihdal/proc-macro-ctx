use std::{
    fs::File,
    path::{Path, PathBuf},
};

use syn::{
    Ident, Type,
    visit::{self, Visit},
};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    pub structs: Vec<String>,
}

pub fn load() -> anyhow::Result<Metadata> {
    let path = current_crate_data_path()?;
    let reader = File::open(&path)?;
    let crate_data: Metadata = serde_json::from_reader(reader)?;
    println!("ctx::load() loaded data from {:?}", path);
    Ok(crate_data)
}

fn out_dir_path() -> &'static Path {
    Path::new(env!("OUT_DIR"))
}

fn current_crate_data_path() -> anyhow::Result<PathBuf> {
    let crate_name = std::env::var("CARGO_PKG_NAME")?;
    let out_path = out_dir_path().join(format!("{}.json", crate_name));
    Ok(out_path)
}

pub fn generate() -> anyhow::Result<()> {
    println!("cargo:warning=ctx::generate() start");

    let crate_manifest_dir_var = std::env::var("CARGO_MANIFEST_DIR")?;
    let crate_manifest_dir = std::path::PathBuf::from(&crate_manifest_dir_var);

    let entry_path = crate_manifest_dir.join("src/main.rs");

    let inline_result =
        syn_inline_mod::InlinerBuilder::default().parse_and_inline_modules(entry_path.as_path())?;

    if inline_result.has_errors() {
        for err in inline_result.errors() {
            Err(anyhow::anyhow!(
                "kind = {}, path = {:?}",
                err.kind(),
                err.path()
            ))?;
        }
    }

    let (file, _) = inline_result.into_output_and_errors();

    let mut traverser = Traverser::default();

    traverser.visit_file(&file);

    let out_path = current_crate_data_path()?;
    let writer = File::create(out_path)?;
    serde_json::to_writer(writer, &traverser.meta)?;

    //Required to rerun analyzer when crate is modified.
    println!("cargo:rerun-if-changed={}", &crate_manifest_dir_var);

    println!("cargo:warning=ctx::generate() saved metadata to {:?}", current_crate_data_path()?);

    println!("cargo:warning=ctx::generate() end");

    Ok(())
}

#[derive(Default)]
pub struct Traverser {
    pub meta: Metadata,
}

impl<'ast> Visit<'ast> for Traverser {
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.meta.structs.push(node.ident.to_string());
    }
}
