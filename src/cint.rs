/* automatically generated by rust-bindgen 0.59.1 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PairData {
    pub rij: [f64; 3usize],
    pub eij: f64,
    pub cceij: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CINTOpt {
    pub index_xyz_array: *mut *mut ::std::os::raw::c_int,
    pub non0ctr: *mut *mut ::std::os::raw::c_int,
    pub sortedidx: *mut *mut ::std::os::raw::c_int,
    pub nbas: ::std::os::raw::c_int,
    pub log_max_coeff: *mut *mut f64,
    pub pairdata: *mut *mut PairData,
}
#[link(name="cint")]
extern "C" {
    pub fn CINTgto_norm(n: ::std::os::raw::c_int, a: f64) -> f64;
    pub fn CINTcgto_cart(
        bas_id: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn CINTcgto_spheric(
        bas_id: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn cint2e_cart(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint2e_cart_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint2e_sph(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint2e_sph_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint3c2e_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint3c2e_cart(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint3c2e_sph(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint2c2e_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint2c2e_cart(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint2c2e_sph(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint2e(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint2e_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint1e_ovlp_sph(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_ovlp_cart(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_ovlp_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint1e_ipovlp_sph(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_ipovlp_cart(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_ipovlp_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint1e_nuc_sph(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_nuc_cart(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_nuc_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint1e_kin_sph(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_kin_cart(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn int1e_kin_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint1e_ipkin_sph(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_ipkin_cart(
        opijkl: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn int1e_ipkin_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint1e_r_cart(
        out: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn cint1e_r_sph(
        out: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    ) -> ::std::os::raw::c_int;
    pub fn int1e_r_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn CINTinit_2e_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn CINTinit_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn int2e_ip1_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
    );
    pub fn cint2e_ip1_cart(
        out: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    );
    pub fn cint2e_ip1_sph(
        out: *mut f64,
        shls: *const ::std::os::raw::c_int,
        atm: *const ::std::os::raw::c_int,
        natm: ::std::os::raw::c_int,
        bas: *const ::std::os::raw::c_int,
        nbas: ::std::os::raw::c_int,
        env: *const f64,
        opt: *const CINTOpt,
    );

    pub fn CINTdel_2e_optimizer(opt: *mut *mut CINTOpt);
    pub fn CINTdel_optimizer(opt: *mut *mut CINTOpt);
}