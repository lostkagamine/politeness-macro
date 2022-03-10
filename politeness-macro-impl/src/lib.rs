#![feature(proc_macro_hygiene)]
#![feature(proc_macro_span)]
#![feature(extend_one)]

use proc_macro::*;
use proc_macro::TokenTree::Ident;

enum PolitenessResult {
    Correct,
    TooPolite,
    TooRude
}

fn count_tokens(strm: TokenStream) -> (i32, i32) {
    let mut total = 0;
    let mut polite = 0;
    let mut input = strm.clone().into_iter();
    while let Some(t) = input.next() {
        match t {
            TokenTree::Group(x) => {
                let l = count_tokens(x.stream());
                total += l.0;
                polite += l.1;
            },
            Ident(x) if x.to_string() == "please" => {
                polite += 1;
                total += 1;
            },
            _ => {
                total += 1;
            }
        }
    }
    (total, polite)
}

fn polite_enough(total: i32, polite: i32) -> (PolitenessResult, f32) {
    let ratio = if polite == 0 {
        0.0
    } else {
        polite as f32 / total as f32
    };

    // We can't match on this because floats are dumb
    if ratio >= 0.0 && ratio <= 0.15 {
        (PolitenessResult::TooRude, ratio)
    } else if ratio >= 0.15 && ratio <= 0.4 {
        (PolitenessResult::Correct, ratio)
    } else if ratio >= 0.4 {
        (PolitenessResult::TooPolite, ratio)
    } else {
        panic!("what")
    }
}

fn remove_please_from_group(grp: Group) -> Group {
    let strm = remove_please(grp.stream());
    Group::new(grp.delimiter(), strm)
}

fn remove_please(tk: TokenStream) -> TokenStream {
    let mut k = tk.clone().into_iter();
    let mut new = TokenStream::new();
    while let Some(f) = k.next() {
        match f {
            TokenTree::Group(x) => {
                new.extend_one(TokenTree::Group(remove_please_from_group(x)));
            },
            Ident(x) if x.to_string() == "please" => {
                // Do nothing, remove it
            },
            x @ _ => {
                new.extend_one(x);
            }
        }
    };
    new
}

#[proc_macro]
pub fn polite(tk: TokenStream) -> TokenStream {
    let howpolite = count_tokens(tk.clone());
    match polite_enough(howpolite.0, howpolite.1) {
        (PolitenessResult::Correct, _) => {},
        (PolitenessResult::TooRude, x) => {
            panic!("code was too rude (value {x})")
        },
        (PolitenessResult::TooPolite, x) => {
            panic!("code was too polite (value {x})")
        }
    }

    let new = remove_please(tk);
    new
}