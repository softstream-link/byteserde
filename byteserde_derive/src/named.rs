use quote::{__private::TokenStream, quote};
use syn::Ident;

use crate::struct_shared::Endian;


// ****************** ARRAYS ******************
// serialize
pub fn ser_arr_num_named(endian: &Endian, fld_name: &Ident) -> TokenStream {
    match endian {
        Endian::Big => quote!(for n in self.#fld_name { ser.serialize_be(n)?; }),
        Endian::Lit => quote!(for n in self.#fld_name { ser.serialize_le(n)?; }),
        _ => quote!(for n in self.#fld_name { ser.serialize_ne(n)?; }),
    }
}
