// api-extract: extract public API from crates/library/src/ into stripped Rust reference files.
//
// For each .rs file with pub items, writes api-reference/library/src/<path>.rs.
// Also writes api-reference/library/src/lib.rs as an index of all modules.
//
// Doc comment extraction: syn represents `/// foo` as `#[doc = " foo"]`; we strip the
// leading space and join all doc lines into a single text block.
//
// Signature rendering: use `quote::ToTokens` on `syn::Signature` to produce a token
// stream, then pretty-print it.  For struct/enum we render manually.
//
// Path mapping:
//   crates/library/src/<x>.rs          -> api-reference/library/src/<x>.rs
//   crates/library/src/foo/mod.rs      -> api-reference/library/src/foo/mod.rs
//   crates/library/src/lib.rs          -> api-reference/library/src/lib.rs  (also index)

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;
use std::path::{Path, PathBuf};
use syn::visit::Visit;
use syn::{Attribute, Item, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait, Visibility};
use walkdir::WalkDir;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn is_pub(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

/// Extract doc comments from a list of attributes.
/// `/// foo` → `#[doc = " foo"]` in syn; we trim the leading space.
fn extract_docs(attrs: &[Attribute]) -> String {
    let mut lines = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &nv.value
                {
                    // Each doc comment line has a leading space: " foo" → "foo"
                    // Strip exactly one leading space (rustdoc convention), preserve rest.
                    let val = s.value();
                    lines.push(val.strip_prefix(' ').unwrap_or(&val).to_string());
                }
            }
        }
    }
    lines.join("\n")
}

/// Extract `#[derive(...)]` attributes, rendered as `#[derive(Trait1, Trait2)]`.
fn extract_derives(attrs: &[Attribute]) -> String {
    let mut derives = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("derive") {
            // Parse the token list inside derive(...) and render cleanly
            if let syn::Meta::List(list) = &attr.meta {
                let inner = normalise_tokens(&list.tokens.to_string());
                derives.push(format!("#[derive({inner})]"));
            }
        }
    }
    derives.join("\n")
}

/// Render a syn::Signature to string via its ToTokens impl, then normalise whitespace.
fn render_sig(sig: &syn::Signature) -> String {
    let ts: TokenStream = sig.to_token_stream();
    normalise_tokens(&ts.to_string())
}

/// Normalise any TokenStream-to-string conversion.
fn norm_ts(ts: TokenStream) -> String {
    normalise_tokens(&ts.to_string())
}

/// Best-effort token-stream normalisation: remove excess spaces around `(){}[]<>:,&`.
///
/// syn's TokenStream::to_string() separates every token with spaces, which gives
/// "Vec < [BigRational ; 4] >" and "& self".  We collapse these back to idiomatic Rust.
fn normalise_tokens(s: &str) -> String {
    // Step 1: collapse multiple spaces to one.
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");

    // Step 2: remove spaces inside grouping delimiters and around angle brackets/brackets.
    // The token stream puts spaces between every token: "Vec < T >" → need "Vec<T>".
    // We need to remove space BEFORE `<` (generic context), AFTER `<`, and BEFORE `>`.
    let s = s
        .replace("( ", "(")
        .replace(" )", ")")
        .replace("[ ", "[")
        .replace(" ]", "]")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace(" <", "<")   // "Vec <T>" → "Vec<T>"
        .replace(" ,", ",")
        .replace(" ;", ";");

    // Step 3: fix reference & lifetime spacing.
    // Token stream gives "& mut T", "& self", "& 'a T" — collapse all "& " → "&".
    // We do it with a simple replace loop since the context is always a ref token.
    let mut s = s
        .replace("& mut ", "&mut ")
        .replace("& '", "&'")
        .replace("& self", "&self")
        .replace("& Self", "&Self");
    // Handle remaining "& <word>" → "&<word>" by scanning for "& " followed by identifier/type chars
    // We do this with the char-level pass below (flag ref_amp).

    // Step 4: fix "::" spacing — ":: " → "::" and " ::" → "::"
    s = s.replace(":: ", "::").replace(" ::", "::");

    // Step 5: remove space before `:` in parameter bindings: "x : T" → "x: T"
    s = s.replace(" :", ":");
    // Remove trailing commas before closing parens (syn adds trailing commas in param lists)
    s = s.replace(", )", ")").replace(",)", ")");
    // Remove space between function/method name and its parameter list: "name (" → "name("
    // We can't do a blanket replace of " (" since it appears in e.g. "where (".
    // Instead, handle it in the char-level pass or accept the minor cosmetic issue.
    // For now: handle the most common case by collapsing " (" after word-chars.
    // Now re-add space after lone `:` (not `::`) and after `,` using char-level pass below.

    // Step 6: re-add spaces after `,` and after lone `:`, collapse "& X" → "&X",
    // and collapse "word (" → "word(" (fn/method name before param list).
    let mut out = String::with_capacity(s.len() + 8);
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut i = 0;
    while i < n {
        let c = chars[i];
        match c {
            // Collapse "& " → "&" (remaining reference spacing)
            '&' => {
                out.push('&');
                // Skip one space after '&', unless followed by another '&'
                if chars.get(i + 1).copied() == Some(' ')
                    && chars.get(i + 2).copied().map(|x| x != '&').unwrap_or(false)
                {
                    i += 2;
                    continue;
                }
            }
            // Collapse "ident (" → "ident("
            ' ' => {
                if chars.get(i + 1).copied() == Some('(') {
                    // Drop this space — the '(' will be written in the next iteration.
                    // But only if the previous char was a word character (identifier).
                    let prev_is_word = out.chars().last().map(|c| c.is_alphanumeric() || c == '_' || c == '>').unwrap_or(false);
                    if prev_is_word {
                        i += 1;
                        continue; // skip the space
                    } else {
                        out.push(' ');
                    }
                } else {
                    out.push(' ');
                }
            }
            ',' => {
                out.push(',');
                if chars.get(i + 1).copied().unwrap_or(' ') != ' ' {
                    out.push(' ');
                }
            }
            ':' => {
                out.push(':');
                let next = chars.get(i + 1).copied().unwrap_or(' ');
                // Add space after ':' only if this is a lone ':' (not part of '::').
                // Check: next char is not ':', AND previous char in output is not ':'.
                let prev_is_colon = out.len() >= 2
                    && out.as_bytes().get(out.len() - 2).copied() == Some(b':');
                if next != ':' && next != ' ' && !prev_is_colon {
                    out.push(' ');
                }
            }
            _ => {
                out.push(c);
            }
        }
        i += 1;
    }
    out
}

/// Render enum variants from a syn::ItemEnum.
fn render_enum_variants(item: &ItemEnum) -> String {
    let mut s = String::new();
    for variant in &item.variants {
        let name = &variant.ident;
        match &variant.fields {
            syn::Fields::Unit => {
                let _ = writeln!(s, "    {name},");
            }
            syn::Fields::Unnamed(fields) => {
                let types: Vec<String> = fields
                    .unnamed
                    .iter()
                    .map(|f| norm_ts(f.ty.to_token_stream()))
                    .collect();
                let _ = writeln!(s, "    {name}({}),", types.join(", "));
            }
            syn::Fields::Named(fields) => {
                let _ = write!(s, "    {name} {{");
                for field in &fields.named {
                    let fname = field.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
                    let ftype = norm_ts(field.ty.to_token_stream());
                    let _ = write!(s, " {fname}: {ftype},");
                }
                let _ = writeln!(s, " }},");
            }
        }
    }
    s
}

/// Render struct fields: if all fields are private, write `/* private fields */`;
/// if any field is pub, list name and type.
fn render_struct_fields(item: &ItemStruct) -> String {
    match &item.fields {
        syn::Fields::Unit => String::new(),
        syn::Fields::Named(fields) => {
            let pub_fields: Vec<_> = fields
                .named
                .iter()
                .filter(|f| is_pub(&f.vis))
                .collect();
            if pub_fields.is_empty() {
                "    /* private fields */\n".to_string()
            } else {
                let mut s = String::new();
                for f in &pub_fields {
                    let fname = f.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
                    let ftype = norm_ts(f.ty.to_token_stream());
                    let _ = writeln!(s, "    pub {fname}: {ftype},");
                }
                s
            }
        }
        syn::Fields::Unnamed(fields) => {
            let pub_fields: Vec<_> = fields
                .unnamed
                .iter()
                .filter(|f| is_pub(&f.vis))
                .collect();
            if pub_fields.is_empty() {
                "    /* private fields */\n".to_string()
            } else {
                let types: Vec<String> = pub_fields
                    .iter()
                    .map(|f| {
                        let t = norm_ts(f.ty.to_token_stream());
                        format!("    pub {t},\n")
                    })
                    .collect();
                types.join("")
            }
        }
    }
}

/// Check if an item has a `#[cfg(test)]` attribute.
/// Matches `#[cfg(test)]` exactly, not `#[cfg(not(test))]` or feature names containing "test".
fn has_cfg_test(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("cfg") {
            // Parse the cfg(...) content. We check for the token sequence `test`
            // appearing as a direct argument, not nested inside `not(...)`.
            if let Ok(nested) = attr.parse_args::<syn::Ident>() {
                if nested == "test" {
                    return true;
                }
            }
        }
    }
    false
}

// ── Extracted items ───────────────────────────────────────────────────────────

#[derive(Default)]
struct ModuleItems {
    /// Module-level doc comment (//! lines)
    module_doc: String,
    /// Top-level pub items: structs, enums, traits, free functions, consts
    items: Vec<ExtractedItem>,
    /// Methods grouped by type name: type_name → list of methods
    impl_methods: BTreeMap<String, Vec<ExtractedMethod>>,
    /// Trait impls noted: "impl TraitName for TypeName"
    trait_impls: Vec<String>,
}

#[derive(Debug)]
enum ExtractedItem {
    Struct {
        name: String,
        rendered: String, // "pub struct Name { ... }" block
        doc: String,
    },
    Enum {
        name: String,
        rendered: String,
        doc: String,
    },
    Trait {
        name: String,
        rendered: String,
        doc: String,
    },
    Function {
        name: String,
        sig: String,
        doc: String,
    },
    Const {
        name: String,
        rendered: String,
        doc: String,
    },
}

impl ExtractedItem {
    fn name(&self) -> &str {
        match self {
            Self::Struct { name, .. }
            | Self::Enum { name, .. }
            | Self::Trait { name, .. }
            | Self::Function { name, .. }
            | Self::Const { name, .. } => name,
        }
    }
}

#[derive(Debug)]
struct ExtractedMethod {
    #[allow(dead_code)]
    type_name: String,
    name: String,
    sig: String, // full "pub fn ..." signature string
    doc: String,
}

// ── Extraction visitor ────────────────────────────────────────────────────────

struct Extractor {
    result: ModuleItems,
}

impl Extractor {
    fn new() -> Self {
        Self {
            result: ModuleItems::default(),
        }
    }

    fn handle_const(&mut self, c: &syn::ItemConst) {
        if !is_pub(&c.vis) {
            return;
        }
        if has_cfg_test(&c.attrs) {
            return;
        }
        let name = c.ident.to_string();
        let doc = extract_docs(&c.attrs);
        let ty = norm_ts(c.ty.to_token_stream());
        let val = norm_ts(c.expr.to_token_stream());
        let rendered = format!("pub const {name}: {ty} = {val};");
        self.result
            .items
            .push(ExtractedItem::Const { name, rendered, doc });
    }
}

impl<'ast> Visit<'ast> for Extractor {
    // Extract module-level doc comments from inner attributes on the file's items.
    // syn::File doesn't carry inner attrs directly but they appear on the first item
    // or as file.attrs.  We'll handle file.attrs separately in `extract_file`.

    fn visit_item(&mut self, item: &'ast Item) {
        if has_cfg_test(item_attrs(item)) {
            return; // skip #[cfg(test)] items
        }
        match item {
            Item::Struct(s) => self.visit_item_struct(s),
            Item::Enum(e) => self.visit_item_enum(e),
            Item::Trait(t) => self.visit_item_trait(t),
            Item::Fn(f) => self.visit_item_fn(f),
            Item::Const(c) => self.handle_const(c),
            Item::Impl(i) => self.visit_item_impl(i),
            Item::Mod(m) => {
                // Only recurse into inline modules that aren't #[cfg(test)]
                if !has_cfg_test(&m.attrs) {
                    if let Some((_, items)) = &m.content {
                        for inner in items {
                            self.visit_item(inner);
                        }
                    }
                }
            }
            _ => {} // skip use, type aliases, macro_rules, etc.
        }
    }

    fn visit_item_struct(&mut self, s: &'ast ItemStruct) {
        if !is_pub(&s.vis) {
            return;
        }
        if has_cfg_test(&s.attrs) {
            return;
        }
        let name = s.ident.to_string();
        let doc = extract_docs(&s.attrs);
        let derives = extract_derives(&s.attrs);
        let fields = render_struct_fields(s);
        let derives_prefix = if derives.is_empty() { String::new() } else { format!("{derives}\n") };
        let rendered = format!("{derives_prefix}pub struct {name} {{\n{fields}}}");
        self.result
            .items
            .push(ExtractedItem::Struct { name, rendered, doc });
    }

    fn visit_item_enum(&mut self, e: &'ast ItemEnum) {
        if !is_pub(&e.vis) {
            return;
        }
        if has_cfg_test(&e.attrs) {
            return;
        }
        let name = e.ident.to_string();
        let doc = extract_docs(&e.attrs);
        let derives = extract_derives(&e.attrs);
        let variants = render_enum_variants(e);
        let derives_prefix = if derives.is_empty() { String::new() } else { format!("{derives}\n") };
        let rendered = format!("{derives_prefix}pub enum {name} {{\n{variants}}}");
        self.result
            .items
            .push(ExtractedItem::Enum { name, rendered, doc });
    }

    fn visit_item_trait(&mut self, t: &'ast ItemTrait) {
        if !is_pub(&t.vis) {
            return;
        }
        if has_cfg_test(&t.attrs) {
            return;
        }
        let name = t.ident.to_string();
        let doc = extract_docs(&t.attrs);
        // List method signatures inside the trait
        let mut methods = String::new();
        for item in &t.items {
            if let syn::TraitItem::Fn(m) = item {
                let sig = render_sig(&m.sig);
                let _ = writeln!(methods, "    {sig};");
            }
        }
        let rendered = if methods.is_empty() {
            format!("pub trait {name} {{}}")
        } else {
            format!("pub trait {name} {{\n{methods}}}")
        };
        self.result
            .items
            .push(ExtractedItem::Trait { name, rendered, doc });
    }

    fn visit_item_fn(&mut self, f: &'ast ItemFn) {
        if !is_pub(&f.vis) {
            return;
        }
        if has_cfg_test(&f.attrs) {
            return;
        }
        let name = f.sig.ident.to_string();
        let doc = extract_docs(&f.attrs);
        let sig_str = render_sig(&f.sig);
        let sig = format!("pub {sig_str}");
        self.result
            .items
            .push(ExtractedItem::Function { name, sig, doc });
    }

    fn visit_item_impl(&mut self, imp: &'ast ItemImpl) {
        if has_cfg_test(&imp.attrs) {
            return;
        }

        // Derive the type name from self_ty
        let type_name = type_name_from_syn_type(&imp.self_ty);

        // Note trait impls
        if let Some((_, trait_path, _)) = &imp.trait_ {
            let trait_name = path_to_string(trait_path);
            self.result
                .trait_impls
                .push(format!("impl {trait_name} for {type_name}"));
        }

        // Extract pub methods
        for item in &imp.items {
            if let syn::ImplItem::Fn(method) = item {
                if !is_pub(&method.vis) {
                    continue;
                }
                if has_cfg_test(&method.attrs) {
                    continue;
                }
                let method_name = method.sig.ident.to_string();
                let doc = extract_docs(&method.attrs);
                let sig_str = render_sig(&method.sig);
                let sig = format!("pub {sig_str}");
                self.result
                    .impl_methods
                    .entry(type_name.clone())
                    .or_default()
                    .push(ExtractedMethod {
                        type_name: type_name.clone(),
                        name: method_name,
                        sig,
                        doc,
                    });
            }
        }
    }
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Struct(s) => &s.attrs,
        Item::Enum(e) => &e.attrs,
        Item::Trait(t) => &t.attrs,
        Item::Fn(f) => &f.attrs,
        Item::Const(c) => &c.attrs,
        Item::Impl(i) => &i.attrs,
        Item::Mod(m) => &m.attrs,
        _ => &[],
    }
}

fn type_name_from_syn_type(ty: &syn::Type) -> String {
    if let syn::Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            return seg.ident.to_string();
        }
    }
    ty.to_token_stream().to_string()
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

// ── File-level extraction ─────────────────────────────────────────────────────

fn extract_file(src_path: &Path) -> anyhow_lite::Result<ModuleItems> {
    let src = std::fs::read_to_string(src_path)?;
    let file = syn::parse_file(&src).map_err(|e| {
        anyhow_lite::Error(format!("syn parse error in {}: {e}", src_path.display()))
    })?;

    let mut extractor = Extractor::new();

    // Module-level doc: inner attributes on the file (#![doc = "..."] or //! comments)
    let module_doc = extract_docs(&file.attrs);
    extractor.result.module_doc = module_doc;

    for item in &file.items {
        extractor.visit_item(item);
    }

    Ok(extractor.result)
}

// ── Rust-format rendering ─────────────────────────────────────────────────────

/// Prefix every non-empty line of `doc` with `prefix` and return the result.
/// Empty lines (blank lines within a doc comment block) are emitted as `prefix` trimmed
/// of trailing space, so we get `//!` or `///` on blank doc lines rather than `//! `.
fn prefix_doc_lines(doc: &str, prefix: &str) -> String {
    let mut out = String::new();
    for line in doc.lines() {
        if line.is_empty() {
            // Blank line inside doc block: emit the prefix without trailing space.
            let _ = writeln!(out, "{}", prefix.trim_end());
        } else {
            let _ = writeln!(out, "{prefix}{line}");
        }
    }
    out
}

fn render_module_rs(src_rel_path: &str, items: &ModuleItems) -> String {
    let mut out = String::new();
    out.push_str("// Auto-generated by api-extract — do not edit.\n");
    let _ = writeln!(out, "// Source: crates/library/src/{src_rel_path}");
    out.push('\n');

    if !items.module_doc.is_empty() {
        out.push_str(&prefix_doc_lines(&items.module_doc, "//! "));
        out.push('\n');
    }

    for item in &items.items {
        render_item(&mut out, item, &items.impl_methods);
    }

    // Render trait impls at the end.
    // ti is already "impl TraitName for TypeName" (built in visit_item_impl).
    for ti in &items.trait_impls {
        let _ = writeln!(out, "{ti} {{ ... }}");
    }
    if !items.trait_impls.is_empty() {
        out.push('\n');
    }

    out
}

fn render_item(
    out: &mut String,
    item: &ExtractedItem,
    impl_methods: &BTreeMap<String, Vec<ExtractedMethod>>,
) {
    match item {
        ExtractedItem::Struct { name, rendered, doc } => {
            if !doc.is_empty() {
                out.push_str(&prefix_doc_lines(doc, "/// "));
            }
            let _ = writeln!(out, "{rendered}");
            out.push('\n');
            // Emit impl block with methods for this struct
            if let Some(methods) = impl_methods.get(name) {
                let _ = writeln!(out, "impl {name} {{");
                for m in methods {
                    render_method(out, m);
                }
                out.push_str("}\n\n");
            }
        }
        ExtractedItem::Enum { name, rendered, doc } => {
            if !doc.is_empty() {
                out.push_str(&prefix_doc_lines(doc, "/// "));
            }
            let _ = writeln!(out, "{rendered}");
            out.push('\n');
            // Emit impl block with methods for this enum
            if let Some(methods) = impl_methods.get(name) {
                let _ = writeln!(out, "impl {name} {{");
                for m in methods {
                    render_method(out, m);
                }
                out.push_str("}\n\n");
            }
        }
        ExtractedItem::Trait { name: _, rendered, doc } => {
            if !doc.is_empty() {
                out.push_str(&prefix_doc_lines(doc, "/// "));
            }
            let _ = writeln!(out, "{rendered}");
            out.push('\n');
        }
        ExtractedItem::Function { name: _, sig, doc } => {
            if !doc.is_empty() {
                out.push_str(&prefix_doc_lines(doc, "/// "));
            }
            let _ = writeln!(out, "{sig} {{ ... }}");
            out.push('\n');
        }
        ExtractedItem::Const { name: _, rendered, doc } => {
            if !doc.is_empty() {
                out.push_str(&prefix_doc_lines(doc, "/// "));
            }
            let _ = writeln!(out, "{rendered}");
            out.push('\n');
        }
    }
}

fn render_method(out: &mut String, m: &ExtractedMethod) {
    if !m.doc.is_empty() {
        // Indent doc lines inside the impl block
        for line in m.doc.lines() {
            if line.is_empty() {
                let _ = writeln!(out, "    ///");
            } else {
                let _ = writeln!(out, "    /// {line}");
            }
        }
    }
    let _ = writeln!(out, "    {} {{ ... }}", m.sig);
    out.push('\n');
}

// ── Path utilities ────────────────────────────────────────────────────────────

/// Derive module path from file path relative to src root.
/// e.g. "geom/polytope.rs" → "geom::polytope"
///      "geom/mod.rs"      → "geom::mod"  (kept as-is; caller can decide)
///      "lib.rs"           → "lib"
#[allow(dead_code)]
fn file_to_module_path(rel: &Path) -> String {
    let without_ext = rel.with_extension("");
    without_ext
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("::")
}

/// Derive a human-readable module path: `mod.rs` → parent dir name.
fn file_to_display_module_path(rel: &Path) -> String {
    let without_ext = rel.with_extension("");
    let components: Vec<_> = without_ext
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();
    // If last component is "mod", drop it and use parent; if only one component, keep it.
    if components.last().map(|s| s.as_str()) == Some("mod") && components.len() > 1 {
        components[..components.len() - 1].join("::")
    } else {
        components.join("::")
    }
}

// ── anyhow_lite: minimal error type so we avoid adding anyhow dep ─────────────

mod anyhow_lite {
    use std::fmt;

    pub struct Error(pub String);

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<E: std::error::Error> From<E> for Error {
        fn from(e: E) -> Self {
            Error(e.to_string())
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    // Locate workspace root: the directory containing crates/
    // We're run from the workspace (crates/) directory, so go up one level.
    let workspace_root = {
        let cwd = std::env::current_dir().expect("current_dir");
        // cwd is typically .../crates/ when run via `cargo run -p api-extract`
        // but might be the workspace root if run differently.
        // Check for crates/library presence to determine root.
        if cwd.join("library").exists() {
            // cwd is crates/
            cwd.parent().expect("crates/ has a parent directory").to_path_buf()
        } else if cwd.join("crates/library").exists() {
            cwd.clone()
        } else {
            // Fallback: assume cwd is workspace root
            cwd.clone()
        }
    };

    let library_src = workspace_root.join("crates/library/src");
    let api_ref_root = workspace_root.join("api-reference/library/src");

    println!("Library source: {}", library_src.display());
    println!("Output root:    {}", api_ref_root.display());

    // Collect all .rs files (sorted for deterministic output)
    let mut rs_files: Vec<PathBuf> = WalkDir::new(&library_src)
        .into_iter()
        .filter_map(|e: Result<walkdir::DirEntry, _>| e.ok())
        .filter(|e: &walkdir::DirEntry| {
            e.path().extension().map(|x| x == "rs").unwrap_or(false)
        })
        .map(|e: walkdir::DirEntry| e.path().to_path_buf())
        .collect();
    rs_files.sort();

    println!("Found {} .rs files", rs_files.len());

    // index_entries: module_display_path → list of key item names (structs, enums, fns)
    let mut index_entries: Vec<(String, Vec<String>)> = Vec::new();

    for rs_path in &rs_files {
        let rel: &Path = rs_path
            .strip_prefix(&library_src)
            .expect("strip library_src prefix");

        let module_display = file_to_display_module_path(rel);

        match extract_file(rs_path) {
            Err(e) => {
                eprintln!("WARN: skipping {} — {e}", rs_path.display());
                continue;
            }
            Ok(items) => {
                let has_pub = !items.items.is_empty() || !items.impl_methods.is_empty();

                if !has_pub && items.module_doc.is_empty() {
                    // Nothing to document
                    continue;
                }

                // Build output path: same .rs extension, mirrored under api-reference/
                let out_path = api_ref_root.join(rel);
                std::fs::create_dir_all(out_path.parent().unwrap())
                    .expect("create output dir");

                // rel is already a path ending in .rs; convert to a display string for header
                let src_rel_str = rel.to_string_lossy();
                let rs = render_module_rs(&src_rel_str, &items);
                std::fs::write(&out_path, &rs).expect("write rs");
                println!("  wrote {}", out_path.display());

                // Collect key names for index
                let key_names: Vec<String> = items
                    .items
                    .iter()
                    .filter(|i| matches!(i, ExtractedItem::Struct { .. } | ExtractedItem::Enum { .. }))
                    .map(|i| i.name().to_string())
                    .chain(
                        items
                            .items
                            .iter()
                            .filter(|i| matches!(i, ExtractedItem::Function { .. }))
                            .map(|i| i.name().to_string()),
                    )
                    .take(6) // keep index concise
                    .collect();

                if !key_names.is_empty() || has_pub {
                    index_entries.push((module_display, key_names));
                }
            }
        }
    }

    // Write index
    let index_path = api_ref_root.join("lib.rs");
    let index_rs = render_index(&index_entries);
    std::fs::create_dir_all(api_ref_root).expect("create api_ref_root");
    std::fs::write(&index_path, &index_rs).expect("write index");
    println!("  wrote {}", index_path.display());

    println!("Done. {} modules documented.", index_entries.len());
}

fn render_index(entries: &[(String, Vec<String>)]) -> String {
    let mut out = String::new();
    out.push_str("// Auto-generated by api-extract — do not edit.\n");
    out.push_str("// Path convention: crates/<path>.rs → api-reference/<path>.rs\n");
    out.push('\n');

    for (module_path, key_names) in entries {
        if key_names.is_empty() {
            let _ = writeln!(out, "// {module_path}");
        } else {
            let names = key_names.join(", ");
            let _ = writeln!(out, "// {module_path} — {names}");
        }
    }

    out
}
