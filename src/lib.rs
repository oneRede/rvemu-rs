#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

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

    tt.into()
}

#[proc_macro]
pub fn p_func2(typ: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(typ).unwrap();
    let tt = quote! {
        let rs1 = state.gp_regs[insn.rs1 as usize];
        let imm = insn.imm as u64;
        state.gp_regs[insn.rd as usize] = (#expr) as u64;
    };

    tt.into()
}

#[proc_macro]
pub fn p_func3(typ: TokenStream) -> TokenStream {
    let ty: syn::Type = syn::parse(typ).unwrap();
    let tt = quote! {
        let rs1 = state.gp_regs[insn.rs1 as usize];
        let rs2 = state.gp_regs[insn.rs2 as usize];
        let ptr: *mut #ty = ptr::null_mut();
        let ptr: *mut #ty = unsafe { ptr.add((rs1 + (insn.imm as u64)) as usize) };
        let ptr_mut: &mut #ty = unsafe{ptr.as_mut().unwrap()};
        *ptr_mut = (rs2 as #ty);

    };

    tt.into()
}

#[proc_macro]
pub fn p_func4(expr: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(expr).unwrap();
    let tt = quote! {
        let rs1 = state.gp_regs[insn.rs1 as usize];
        let rs2 = state.gp_regs[insn.rs2 as usize];
        state.gp_regs[insn.rd as usize] = (#expr) as u64;
    };

    tt.into()
}                                              

#[proc_macro]
pub fn p_func5(expr: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(expr).unwrap();
    let tt = quote! {
        let rs1 = state.gp_regs[insn.rs1 as usize];
        let rs2 = state.gp_regs[insn.rs2 as usize];
        let target_addr = state.pc + (insn.imm as u64);
        if (#expr) {
            state.pc = target_addr;
            state.reenter_pc = target_addr;
            state.exit_reason = ExitReason::DirectBranch;
            insn.cont = true;
        }
    };

    tt.into()
}

#[proc_macro]
pub fn p_func6(_item: TokenStream) -> TokenStream {
    let tt = quote! {
        match (insn.csr) {
            0 | 1 | 2 => {}
            _ => unimplemented!()
        }
        state.gp_regs[insn.rd as usize] = 0;
    };
    tt.into()
}