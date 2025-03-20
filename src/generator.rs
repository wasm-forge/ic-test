use std::{env, fs, path::PathBuf};

use ic_cdk_bindgen::code_generator;

use crate::arguments::IcTestArgs;

/*
struct CandidMacroArgs {
    ident: Ident,
    file: String,
}

impl Parse for CandidMacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Expect an identifier, followed by a comma, followed by a string literal
        let ident: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let file_lit: Lit = input.parse()?;

        // We want to be sure it's actually a string literal:
        let file_string = match file_lit {
            Lit::Str(s) => s.value(),
            _ => {
                return Err(syn::Error::new_spanned(
                    file_lit,
                    "Expected a string literal for the file name",
                ));
            }
        };

        Ok(CandidMacroArgs {
            ident,
            file: file_string,
        })
    }
}

#[proc_macro]
pub fn candid(input: TokenStream) -> TokenStream {
    let CandidMacroArgs { ident, file } = parse_macro_input!(input as CandidMacroArgs);

    // read the file at compile time
    let candid_content =
        std::fs::read_to_string(&file).unwrap_or_else(|_| panic!("Could not read file: {}", file));

    // try parse candid file
    let conf = code_generator::Config::new();
    //conf.set_target(code_generator::Target:: );

    let (env, actor) = candid_parser::typing::check_str(&candid_content, true).unwrap();

    let content = ic_cdk_bindgen::code_generator::compile(&conf, &env, &actor);

    let rust_code = syn::parse_str::<syn::File>(&content).expect("Invalid Rust code produced!");

    let expanded = quote! {
        mod #ident {
            #rust_code
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn candid_inline(input: TokenStream) -> TokenStream {
    let prog = input.to_string();

    // try parse candid file
    let mut conf = code_generator::Config::new();

    let (env, actor) = candid_parser::typing::check_str(&prog, true).unwrap();

    let content = ic_cdk_bindgen::code_generator::compile(&conf, &env, &actor);

    let rust_code = syn::parse_str::<syn::File>(&content).expect("String must be valid Rust code");

    // Convert the parsed syntax tree back into tokens
    let tokens = quote! { #rust_code };

    TokenStream::from(tokens)
}

*/

pub fn generate(args: &IcTestArgs) -> anyhow::Result<()> {
    let canister_name = args.canister_names.clone();
    /*
    let candid_file = args.candid_path.clone();

    let candid_content = std::fs::read_to_string(&candid_file).unwrap_or_else(|_| {
        panic!(
            "Could not read file: {} from path: {:?}",
            &candid_file,
            env::current_dir()
        )
    });

    // current folder
    let mut bindings_path = env::current_dir()?;
    bindings_path.push("tests/bindings");

    fs::create_dir_all(&bindings_path)?;

    // prepare mod file
    {
        let mut mod_file: PathBuf = bindings_path.clone();
        mod_file.push("mod.rs");

        let mod_content = format!("mod {canister_name};\n");

        let rust_code =
            syn::parse_str::<syn::File>(&mod_content).expect("Invalid Rust code produced!");

        let formatted_code = prettyplease::unparse(&rust_code);

        fs::write(mod_file, formatted_code)
            .unwrap_or_else(|_| panic!("Could not create the mod.rs file: {}", canister_name));
    }

    // create canister file(s)
    {
        let mut canister_file = bindings_path.clone();
        canister_file.push(format!("{canister_name}.rs"));

        // try parse candid file
        let conf = code_generator::Config::new();
        //conf.set_target(code_generator::Target:: );

        let (env, actor) = candid_parser::typing::check_str(&candid_content, true).unwrap();

        let content = ic_cdk_bindgen::code_generator::compile(&conf, &env, &actor);

        fs::write(&canister_file, content)
            .unwrap_or_else(|_| panic!("Could not write to file: {}", canister_name));
    }

    */

    Ok(())
}
