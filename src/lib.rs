use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote_spanned;
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    Attribute, Expr, Ident, Result, Token, Type, Visibility,
};

/// Declare a new const array without specify length.
/// It helps when apply conditional compilation to part of a const array.
///
/// # Syntax
/// The macro wraps any number of const array declarations(with length `_`).
///
/// ```rust
/// use auto_const_array::auto_const_array;
/// auto_const_array! {
///    // Additional attributes and docs are supported.
///    /// Common array with public visibility.
///    #[allow(unused)]
///    pub const ARRAY_COMMON: [u8; _] = [1, 2, 4];
///    /// Special array with cfg conditional compiling.
///    const ARRAY_WITH_ATTR: [u8; _] = [1, #[cfg(unix)] 2]
/// }
/// ```
#[proc_macro]
pub fn auto_const_array(input: TokenStream) -> TokenStream {
    const_array(input).unwrap_or_else(|e| TokenStream::from(e.to_compile_error()))
}

fn const_array(input: TokenStream) -> Result<TokenStream> {
    let parser = Punctuated::<ConstArray, Token![;]>::parse_terminated;
    let args = parser.parse(input)?;

    let mut output = proc_macro2::TokenStream::new();
    for array in args {
        let len = array.len()?;
        let ConstArray {
            attrs,
            visibility,
            name,
            ty,
            val,
            span,
        } = array;
        output.extend(quote_spanned! {
            span => #(#attrs)* #visibility const #name: [#ty; #len] = #val;
        });
    }

    Ok(TokenStream::from(output))
}

struct ConstArray {
    attrs: Vec<Attribute>,
    visibility: Visibility,
    name: Ident,
    ty: Type,
    val: Expr,
    span: Span,
}

impl Parse for ConstArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![const]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let left_inner;
        bracketed!(left_inner in input);
        let ty = left_inner.parse()?;
        left_inner.parse::<Token![;]>()?;
        left_inner.parse::<Token![_]>()?;
        input.parse::<Token![=]>()?;

        let val: Expr = input.parse()?;
        let span = input.span();
        Ok(Self {
            attrs,
            visibility,
            name,
            ty,
            val,
            span,
        })
    }
}

impl ConstArray {
    fn len(&self) -> Result<proc_macro2::TokenStream> {
        let array = match self.val.clone() {
            Expr::Array(array) => array,
            _ => return Err(syn::Error::new(self.span, "value is not array")),
        };
        let mut output = quote_spanned! {
            self.span =>
                #[allow(unused_mut)]
                let mut length = 0;
        };
        for expr in array.elems {
            let attrs = match expr {
                Expr::Array(inner) => inner.attrs,
                Expr::Assign(inner) => inner.attrs,
                // Expr::AssignOp(inner) => inner.attrs,
                Expr::Async(inner) => inner.attrs,
                Expr::Await(inner) => inner.attrs,
                Expr::Binary(inner) => inner.attrs,
                Expr::Block(inner) => inner.attrs,
                // Expr::Box(inner) => inner.attrs,
                Expr::Break(inner) => inner.attrs,
                Expr::Call(inner) => inner.attrs,
                Expr::Cast(inner) => inner.attrs,
                Expr::Closure(inner) => inner.attrs,
                Expr::Continue(inner) => inner.attrs,
                Expr::Field(inner) => inner.attrs,
                Expr::ForLoop(inner) => inner.attrs,
                Expr::Group(inner) => inner.attrs,
                Expr::If(inner) => inner.attrs,
                Expr::Index(inner) => inner.attrs,
                Expr::Let(inner) => inner.attrs,
                Expr::Lit(inner) => inner.attrs,
                Expr::Loop(inner) => inner.attrs,
                Expr::Macro(inner) => inner.attrs,
                Expr::Match(inner) => inner.attrs,
                Expr::MethodCall(inner) => inner.attrs,
                Expr::Paren(inner) => inner.attrs,
                Expr::Path(inner) => inner.attrs,
                Expr::Range(inner) => inner.attrs,
                Expr::Reference(inner) => inner.attrs,
                Expr::Repeat(inner) => inner.attrs,
                Expr::Return(inner) => inner.attrs,
                Expr::Struct(inner) => inner.attrs,
                Expr::Try(inner) => inner.attrs,
                Expr::TryBlock(inner) => inner.attrs,
                Expr::Tuple(inner) => inner.attrs,
                // Expr::Type(inner) => inner.attrs,
                Expr::Unary(inner) => inner.attrs,
                Expr::Unsafe(inner) => inner.attrs,
                Expr::While(inner) => inner.attrs,
                Expr::Yield(inner) => inner.attrs,
                _ => return Err(syn::Error::new(self.span, "unsupported expr type")),
            };
            output.extend(quote_spanned! {
                self.span =>
                    #(#attrs)*
                    {length += 1;}
            })
        }
        Ok(quote_spanned! {
            self.span => {
                #output
                length
            }
        })
    }
}
