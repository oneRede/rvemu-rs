#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 + B }".parse().unwrap()
}

#[proc_macro]
pub fn make_answer2(_item: TokenStream) -> TokenStream {
    let add = quote! {fn answer2() -> u32 {422 + B}};
    add.into()
}

#[proc_macro]
pub fn p_func1(typ: TokenStream) -> TokenStream {
    let ty: syn::Type = syn::parse(typ).unwrap();
    let tt = quote! {
        let addr: u64 = state.gp_regs[insn.rs1 as usize] + (insn.imm as u64);
        let h_addr: u64 = to_host!(addr);
        let ptr: *mut #ty = ptr::null_mut();
        let ptr: *mut #ty = unsafe { ptr.add(h_addr as usize) };
        let n: #ty = unsafe{ (*(ptr.as_ref().unwrap()))};
        state.gp_regs[insn.rd as usize] = n as u64;
    };
    println!("{}", tt.to_string());
    tt.into()
}
