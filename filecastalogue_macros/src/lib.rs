extern crate proc_macro;
use std::{convert::TryFrom, fmt::Display, iter::FromIterator};
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span,
    TokenTree, TokenStream
};

#[derive(Debug)]
enum ErrorKind {
    TokenStreamIsNotPurePuncts
}

#[allow(dead_code)] // for as_str
impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::TokenStreamIsNotPurePuncts
            => "TokenStream doesn't exclusively consist of TokenTree::Punct.",
        }
    }
}

#[derive(Debug)]
struct FcMacroError {
    kind: ErrorKind,
    context: String
}

impl FcMacroError {
    fn new(kind: ErrorKind, context: &str) -> Self {
        Self {
            kind: kind,
            context: String::from(context)
        }
    }
}

impl Display for FcMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FcMacroError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Returns true if the specified TokenStream either contains only
/// Punct TokenTrees or nothing at all (this function considers
/// empty TokenStreams as an empty variety of "pure" Punct TokenStreams).
fn is_pure_punct_token_stream(stream: &TokenStream) -> bool {
    let mut tokens = stream.to_owned().into_iter();
    loop {
        let maybe_token: Option<TokenTree> = tokens.next();
        break match maybe_token {
            Some(tt) => match tt {
                TokenTree::Punct(_) => (),
                _ => return false,
            },
            None => (),
        }
    }
    true
}

/// Takes two chars and a TokenStream and checks whether the Punct
/// consists of those two chars, in the order specified.
fn is_this_double_punct(chars: Vec<char>, stream: TokenStream) -> bool {
    let mut i = 0;
    for maybe_punct in stream {
        
        if i == 2 {
            // Third iteration: It's definitely nothing "double". :p
            return false
        }
        
        match maybe_punct as TokenTree {
            TokenTree::Punct(some_punct)
            => if some_punct.as_char() != chars[i] {
                // Not "this" (double?) punct.
                return false
            },
            // Not any punct.
            _ => return false,
        };
        
        i = i+1;
    }
    // If we get here, the stream must consist of exactly two puncts,
    // of which the first corresponds to the first char in chars, and
    // the second corresponds to the second char in chars.
    return true
}

/// Create a TokenStream consisting of two Punct TokenTrees.
fn create_double_punct(first: char, second: char) -> TokenStream {
    TokenStream::from_iter(vec!(
        TokenTree::Punct(Punct::new(first, Spacing::Joint)),
        TokenTree::Punct(Punct::new(second, Spacing::Alone))
    ))
}


/// Punct based TokenStreams of particular kinds.
/// This makes it easier in particular to handle multi-Punct TokenStreams.
enum Puncts {
    FatArrow(TokenStream),
    Turbo(TokenStream),
    Unspecified(TokenStream)
}

impl Puncts {

    fn new_turbo() -> Self {
        Puncts::Turbo(create_double_punct(':', ':'))
    }

    fn new_fat_arrow() -> Self {
        Puncts::FatArrow(create_double_punct('=', '>'))
    }

    /// Tries to guess the TokenStreams'
    /// proper enum variant and returns it accordingly.
    /// This is primarily intended to enable the pattern of assigning 
    /// a TokenStream to Puncts::Unspecified first, to benefit from
    /// the functions Puncts offers to analyze TokenStreams, and then
    /// get a more specific version like ::Turbo, for example.
    /// Returns None if the TokenStream contains anything but Punct,
    /// and ::Unspecified if categorizing it wasn't possible.
    fn get_specific(&self) -> Option<Puncts> {
        let stream = self.get_token_stream();
        if !is_pure_punct_token_stream(&stream) {
            return None
        };
        if *self == Puncts::new_turbo() {
            return Some(Puncts::Turbo(self.get_token_stream()))
        }
        if *self == Puncts::new_fat_arrow() {
            return Some(Puncts::FatArrow(self.get_token_stream()))
        }
        return Some(Puncts::Unspecified(stream))
    }

    /// Get an owned version of the TokenStream we hold.
    fn get_token_stream(&self) -> TokenStream {
         match self {
            Puncts::FatArrow(stream) => stream.to_owned(),
            Puncts::Turbo(stream) => stream.to_owned(),
            Puncts::Unspecified(stream) => stream.to_owned()
        }
    }

    /// Designed towards extracting a Punct from Option<TokenTree>
    /// as would be returned by .next() on iterators when comparing
    /// two TokenStreams.
    ///
    /// If we get None, we panic. To make the panic message nicer, we take
    /// &other, so we can also report the contents of the other TokenStream.
    fn get_punct_from_token_option(&self, maybe_tt: Option<TokenTree>, other: &Self) -> Punct {
        match maybe_tt {
            Some(tt) => match tt {
                TokenTree::Punct(punct) => punct,
                _ => panic!(
                    "{}. Own TokenStream: {:?}. Other TokenStream: {:?}",
                    concat!(
                        "Tried to get Punct while comparing TokenStreams ",
                        "and got non-Punct. This should absolutely never happen."
                    ),
                    self.get_token_stream(),
                    other.get_token_stream()
                ),
            },
            None => panic!(
                "{}. Own TokenStream: {:?}. Other TokenStream: {:?}",
                concat!(
                    "Getting token while comparing TokenStreams failed. ",
                    "This has passed checks against this and should never happen."
                ),
                self.get_token_stream(),
                other.get_token_stream()
            ),
        }
    }
}

impl TryFrom<TokenStream> for Puncts {
    type Error = FcMacroError;

    fn try_from(stream: TokenStream) -> Result<Self, Self::Error> {
        match Puncts::Unspecified(stream).get_specific() {
            Some(puncts) => Ok(puncts),
            None => Err(Self::Error::new(
                ErrorKind::TokenStreamIsNotPurePuncts,
                "Trying to create a Puncts from TokenStream using try_from."
            )),
        }
    }
}

impl From<Puncts> for TokenStream {
    fn from(puncts: Puncts) -> Self {
        puncts.get_token_stream()
    }
}

impl PartialEq for Puncts {
    fn eq(&self, other: &Self) -> bool {

        let own_token_stream = self.get_token_stream();
        let other_token_stream = other.get_token_stream();

        let mut own_tokens = own_token_stream.into_iter();
        let mut other_tokens = other_token_stream.into_iter();
        loop {

            let maybe_own_token = own_tokens.next();
            let maybe_other_token = other_tokens.next();

            // If one is none and the other isn't,
            // they can't possibly be the same.
            if maybe_own_token.is_none() != maybe_other_token.is_none() {
                break false
            }

            // Since both have the same none state, checking one should
            // be sufficient.
            if maybe_own_token.is_none() {
                // If we arrive here, we either:
                //   - Have two TokenStreams with no tokens, which means
                //     for all intents and purposes they're the same.
                //   - Looped past all the below opportunities to find
                //     inequalities and have finally passed the above
                //     none-related checks and found that we're now at
                //     the end. We must consider the two TokenStreams
                //     equal at this point.
                break true
            }

            let own_punct: Punct = self.get_punct_from_token_option(
                maybe_own_token, other);
            let other_punct: Punct = other.get_punct_from_token_option(
                maybe_other_token, other);

            if own_punct.as_char() != other_punct.as_char() {
                break false
            }
            
        }
    }
}

impl Eq for Puncts {}

#[proc_macro]
pub fn proc_error(input: TokenStream)
-> TokenStream {
    let mut output_tokens: Vec<TokenStream> = Vec::new();

    // The following TokenTrees are all cast .into() TokenStreams,
    // this is so we can keep output_tokens Vec<TokenStream>,
    // enabling us to freely combine TokenStreams with our
    // hand-made tokens here. The reasoning for this is that
    // TokenStream is prefereable, as it's powerful (it allows
    // us to combine arbitrary numbers of tokens) and also
    // the standard format for proc_macro interfaces.

    let error_struct_ident: TokenStream = TokenTree::Ident(Ident::new(
        "Error", // Just for testing.
        Span::call_site()
    )).into();
    let new_fn_ident: TokenStream = TokenTree::Ident(Ident::new(
        "new",
        Span::call_site()
    )).into();
    let params: TokenStream = TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        TokenStream::new()
    )).into();

    let turbo = Puncts::new_turbo();
    //let fat_arrow = Puncts::new_fat_arrow();

    output_tokens.push(error_struct_ident);
    output_tokens.push(turbo.into());
    output_tokens.push(new_fn_ident);
    output_tokens.push(params);

    let output: TokenStream =
        TokenStream::from_iter(output_tokens);

    TokenStream::from(output)
}