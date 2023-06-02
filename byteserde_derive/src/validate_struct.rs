use crate::common::{FldSerDesTokens, SerDesTokens};

pub fn has_option_flds(sdt: &SerDesTokens) -> bool {
    sdt.des_option().len() > 0
}
pub fn validate(sdt: &mut SerDesTokens)  {
    if has_option_flds(sdt) {
        if (sdt.des_vars().len() != sdt.des_option().len()) {
            let msg = format!(
                "struct `{}` has a mix of Option<T> other types, which can't be mixed, please", 
                sdt.struct_name() );
            panic!("Error:\n{}", fld_errors(sdt));
        }
    } else {
        match sdt.des_errs() {
            Some(err) => panic!("Error:\n{}", err),
            None => (),
        }
    }
}

fn fld_errors(sdt: &SerDesTokens) -> String {
    match sdt.des_errs() {
        Some(err) => format!("n{}", err),
        None => format!(""),
    }
}