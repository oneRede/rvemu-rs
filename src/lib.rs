#![crate_type = "proc-macro"]
extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn p_func1(typ: TokenStream) -> TokenStream {
    let ty: syn::Type = syn::parse(typ).unwrap();
    let tt = quote! {
        let addr: u64 = ((state.gp_regs[insn.rs1 as usize] as i64) + (insn.imm as i64)) as u64;
        let ptr: *mut #ty = get_ptr(to_host!(addr)) as *mut #ty;
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
        let imm = insn.imm as i64;
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
        let ptr = get_ptr(to_host!(((rs1 as i64) + (insn.imm as i64)) as u64));

        let d_p = (state.gp_regs).as_ptr() as *const u8;
        let d_p = unsafe{ d_p.add((insn.rs2 as usize) * 8)};
        unsafe {ptr.copy_from(d_p, mem::size_of::<#ty>())};
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
        let target_addr = ((state.pc as i64) + (insn.imm as i64)) as u64;
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

#[proc_macro]
pub fn p_func7(expr: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(expr).unwrap();
    let tt = quote! {
        let rs1 = state.fp_regs[insn.rs1 as usize].f;
        let rs2 = state.fp_regs[insn.rs2 as usize].f;
        let rs3 = state.fp_regs[insn.rs3 as usize].f;
        state.fp_regs[insn.rd as usize].f = (#expr) as f32;

    };

    tt.into()
}

#[proc_macro]
pub fn p_func8(expr: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(expr).unwrap();
    let tt = quote! {
        let rs1 = state.fp_regs[insn.rs1 as usize].d;
        let rs2 = state.fp_regs[insn.rs2 as usize].d;
        let rs3 = state.fp_regs[insn.rs3 as usize].d;
        state.fp_regs[insn.rd as usize].d = (#expr);

    };

    tt.into()
}

#[proc_macro]
pub fn p_func9(expr: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(expr).unwrap();
    let tt = quote! {
        let rs1 = state.fp_regs[insn.rs1 as usize].f;
        let rs2 = state.fp_regs[insn.rs2 as usize].f;
        state.fp_regs[insn.rd as usize].f = (#expr) as f32;

    };

    tt.into()
}

#[proc_macro]
pub fn p_func10(expr: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(expr).unwrap();
    let tt = quote! {
        let rs1 = state.fp_regs[insn.rs1 as usize].d;
        let rs2 = state.fp_regs[insn.rs2 as usize].d;
        state.fp_regs[insn.rd as usize].d = (#expr);

    };

    tt.into()
}

#[proc_macro]
pub fn p_func11(array: TokenStream) -> TokenStream {
    let array: syn::ExprArray = syn::parse(array).unwrap();

    let tt = quote! {
        let arr:[bool;2] = #array;
        let rs1 = state.fp_regs[insn.rs1 as usize].w;
        let rs2 = state.fp_regs[insn.rs2 as usize].w;
        state.fp_regs[insn.rd as usize].v = (fsgnj32(rs1, rs2, arr[0], arr[1]) as u64 | ((-1i64 << 32)) as u64);
    };

    tt.into()
}

#[proc_macro]
pub fn p_func12(array: TokenStream) -> TokenStream {
    let array: syn::ExprArray = syn::parse(array).unwrap();

    let tt = quote! {
        let arr:[bool;2] = #array;
        let rs1 = state.fp_regs[insn.rs1 as usize].v;
        let rs2 = state.fp_regs[insn.rs2 as usize].v;
        state.fp_regs[insn.rd as usize].v = fsgnj64(rs1, rs2, arr[0], arr[1]);
    };

    tt.into()
}

#[proc_macro]
pub fn p_func13(expr: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(expr).unwrap();
    let tt = quote! {
        let rs1 = state.fp_regs[insn.rs1 as usize].f;
        let rs2 = state.fp_regs[insn.rs2 as usize].f;
        state.gp_regs[insn.rd as usize] = (#expr) as u64;

    };

    tt.into()
}

#[proc_macro]
pub fn p_func14(expr: TokenStream) -> TokenStream {
    let expr: syn::Expr = syn::parse(expr).unwrap();
    let tt = quote! {
        let rs1 = state.fp_regs[insn.rs1 as usize].d;
        let rs2 = state.fp_regs[insn.rs2 as usize].d;
        state.gp_regs[insn.rd as usize] = (#expr) as u64;

    };

    tt.into()
}

#[proc_macro]
pub fn rewrite_flag(flag: TokenStream) -> TokenStream {
    let st: &str = &("NEWLIB_".to_string() + &flag.to_string());
    let i_flag: syn::Ident = syn::parse(flag).unwrap();
    let st: TokenStream = TokenStream::from_str(st).unwrap();
    let ident: syn::Ident = syn::parse(st).unwrap();
    let tt = quote! {
        if (flags & #ident) != 0 {
            host_flags |= #i_flag
        }

    };

    tt.into()
}

#[proc_macro]
pub fn guest_mem_offset(_item: TokenStream) -> TokenStream {
    let tt = quote! {
        let guest_mem_offset: u64 = 0x000800000000u64;
    };

    tt.into()
}
