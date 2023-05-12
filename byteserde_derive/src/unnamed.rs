use quote::{__private::TokenStream, quote};
use syn::{Member};

use crate::struct_shared::Endian;



// ****************** ARRAYS ******************
// serialize
pub fn ser_arr_num_unnamed(endian: &Endian, fld_index: &Member) -> TokenStream {
    match endian {
        Endian::Big => quote!(for n in self.#fld_index { ser.serialize_be(n)?; }),
        Endian::Lit => quote!(for n in self.#fld_index { ser.serialize_le(n)?; }),
        _ => quote!(for n in self.#fld_index { ser.serialize_ne(n)?; }),
    }
}
