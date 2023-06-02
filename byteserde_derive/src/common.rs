use quote::__private::TokenStream;
use syn::Ident;

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
    pub fn struct_name(&self) -> String{
        match self.struct_type {
            StructType::Regular(ref name, ref id)
            | StructType::Tuple(ref name, ref id)
            | StructType::Enum(ref name, ref id) => name.clone(),
        }
    }
    // SERIALIAZER

    pub fn ser_vars(&self) -> Vec<TokenStream> {
        self.flds.iter().map(|f| f.ser_vars.clone()).collect::<Vec<_>>()
    }
    pub fn ser_repl(&self) -> Vec<TokenStream> {
        self.flds.iter().map(|f| f.ser_repl.clone()).collect::<Vec<_>>()
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
    pub fn des_errs(&self) -> Option<String>{
        let des_errors = self
            .flds
            .iter()
            .map(|f| f.des_errors.clone())
            .collect::<Vec<_>>();
        collate_errors(des_errors)
    }
    pub fn des_vars(&self) -> Vec<TokenStream> {
        self.flds.iter().map(|f| f.des_vars.clone()).collect::<Vec<_>>()
    }
    pub fn des_option(&self) -> Vec<TokenStream> {
        self.flds.iter().map(|f| f.des_option.clone()).collect::<Vec<_>>()
    }
    pub fn des_uses(&self) -> Vec<TokenStream> {
        self.flds.iter().map(|f| f.des_uses.clone()).collect::<Vec<_>>()
    }

    // SIZE
    pub fn size_of(&self) -> Vec<TokenStream> {
        self.flds.iter().map(|f| f.size_of.clone()).collect::<Vec<_>>()
    }
    pub fn size_errors(&self) -> Option<String>{
        let size_errors = self
            .flds
            .iter()
            .map(|f| f.size_errors.clone())
            .collect::<Vec<_>>();
        collate_errors(size_errors)
    }

    pub fn len_of(&self) -> Vec<TokenStream> {
        self.flds.iter().map(|f| f.len_of.clone()).collect::<Vec<_>>()
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
