use quote::__private::TokenStream;
use syn::Ident;

use crate::attr_struct::Peek;

pub enum StructType {
    Regular(String, Ident),
    Tuple(String, Ident),
    Enum(String, Ident),
}
#[derive(Debug)]
pub struct FldSerDesTokens {
    // only used to create serailizer
    pub ser_vars: TokenStream,
    pub ser_repl: TokenStream,
    pub ser_uses_stck: TokenStream,
    pub ser_uses_heap: TokenStream,

    // only used to create deserailizer
    pub des_vars: TokenStream,
    pub des_option: TokenStream,
    pub des_uses: TokenStream,
    pub des_errors: Vec<String>,

    // only used to create size
    pub size_of: TokenStream,
    pub size_errors: Vec<String>,
    pub len_of: TokenStream,
}

pub struct SerDesTokens {
    pub struct_type: StructType,
    pub flds: Vec<FldSerDesTokens>,
}
impl SerDesTokens {
    pub fn struct_name(&self) -> String {
        match self.struct_type {
            StructType::Regular(ref name, _)
            | StructType::Tuple(ref name, _)
            | StructType::Enum(ref name, _) => name.clone(),
        }
    }
    pub fn struct_ident(&self) -> &Ident{
        match self.struct_type {
            StructType::Regular(_, ref ident)
            | StructType::Tuple(_, ref ident)
            | StructType::Enum(_, ref ident) => ident,
        }
    }
    // SERIALIAZER

    pub fn ser_vars(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .map(|f| f.ser_vars.clone())
            .collect::<Vec<_>>()
    }
    pub fn ser_repl(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .map(|f| f.ser_repl.clone())
            .collect::<Vec<_>>()
    }
    pub fn ser_uses_stck(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .map(|f| f.ser_uses_stck.clone())
            .collect::<Vec<_>>()
    }
    pub fn ser_uses_heap(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .map(|f| f.ser_uses_heap.clone())
            .collect::<Vec<_>>()
    }

    // DESERIALIZER
    pub fn des_collated_errs(&self) -> Option<String> {
        let des_errors = self
            .flds
            .iter()
            .map(|f| f.des_errors.clone())
            .collect::<Vec<_>>();
        collate_errors(des_errors)
    }
    pub fn des_vars(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .map(|f| f.des_vars.clone())
            .collect::<Vec<_>>()
    }
    pub fn des_option(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .filter(|f| !f.des_option.is_empty())
            .map(|f| f.des_option.clone())
            .collect::<Vec<_>>()
    }
    pub fn has_option_flds(&self) -> bool {
        self.des_option().len() > 0
    }
    pub fn des_uses(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .filter(|f| !f.des_uses.is_empty())
            .map(|f| f.des_uses.clone())
            .collect::<Vec<_>>()
    }

    // SIZE
    pub fn size_of(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .filter(|f| !f.size_of.is_empty())
            .map(|f| f.size_of.clone())
            .collect::<Vec<_>>()
    }
    pub fn size_errors(&self) -> Option<String> {
        let size_errors = self
            .flds
            .iter()
            .map(|f| f.size_errors.clone())
            .collect::<Vec<_>>();
        collate_errors(size_errors)
    }

    pub fn len_of(&self) -> Vec<TokenStream> {
        self.flds
            .iter()
            .map(|f| f.len_of.clone())
            .collect::<Vec<_>>()
    }

    pub fn des_validate(&self, peek: &Peek) {
        // validate struct with Option<T> fields
        if self.has_option_flds() || matches!(peek, Peek::Set(..)) {
            if !self.has_option_flds() || !matches!(peek, Peek::Set(..))  {
                panic!("struct `{}` `#[byteserde(peek( start, len ))]` and `#[byteserde(eq( .. ))]` have to be used together.",
                    self.struct_name());
            }
            if self.des_vars().len() != self.des_option().len() {
                let val_err = format!(
                    "struct `{}` has a mix of Option<T> other types, which can't be mixed, please move all Option<T> types into a seperate struct", 
                    self.struct_name() );
                let fld_errors = match self.des_collated_errs() {
                    Some(msg) => format!("\n{}", msg),
                    None => format!(""),
                };
                panic!(
                    "struct `{}` ByteDeserialize error:\n{}{}",
                    self.struct_name(),
                    val_err,
                    fld_errors
                );
            }
        // Validate struct with out Option<T> fields
        } else {
            match self.des_collated_errs() {
                Some(msg) => panic!(
                    "struct `{}` ByteDeserialize error:\n{}",
                    self.struct_name(),
                    msg
                ),
                None => (),
            }
        }
    }

    pub fn size_validate(&self) {
        let errors = self.size_errors();
        match errors {
            Some(msg) => panic!(
                "struct `{}` ByteSerializedSizeOf error:\n{}",
                self.struct_name(),
                msg,
            ),
            None => (),
        }
    }
}

pub fn collate_errors(field_errors: Vec<Vec<String>>) -> Option<String> {
    let result = field_errors
        .iter()
        .map(|f| {
            f.iter()
                .filter(|v| !v.is_empty())
                .map(|v| v.clone())
                .collect::<Vec<String>>()
                .join("\n")
        })
        .collect::<Vec<String>>()
        .iter()
        .filter(|v| !v.is_empty())
        .map(|v| v.clone())
        .collect::<Vec<String>>()
        .join("\n");
    if result.len() > 1 {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unittest::setup;
    use log::info;
    #[test]
    fn test_collate_errors() {
        setup::log::configure();
        let field_errors = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
            vec![],
        ];
        info!("field_errors: {:?}", field_errors);
        let result = collate_errors(field_errors);
        assert!(result.is_some());
        let msg = result.unwrap();
        info!("result: \n{}", msg);
        assert_eq!(msg, "a\nb\nc\nd");

        let field_errors: Vec<Vec<String>> = vec![vec![], vec![]];
        info!("field_errors: {:?}", field_errors);
        let result = collate_errors(field_errors);
        info!("result: \n{:?}", result);
        assert_eq!(result, None);
    }
}
