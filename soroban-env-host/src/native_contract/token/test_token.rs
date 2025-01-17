use crate::{
    host_vec,
    native_contract::{
        base_types::Address,
        testutils::{authorize_single_invocation, HostVec, TestSigner},
    },
    Host, HostError,
};
use soroban_env_common::{
    xdr::{Asset, ContractId, CreateContractArgs, HostFunction, ScContractCode},
    Env, RawVal,
};
use soroban_env_common::{Symbol, TryFromVal, TryIntoVal};

use crate::native_contract::base_types::{Bytes, BytesN};

pub(crate) struct TestToken<'a> {
    pub(crate) id: BytesN<32>,
    host: &'a Host,
}

impl<'a> TestToken<'a> {
    pub(crate) fn new_from_asset(host: &'a Host, asset: Asset) -> Self {
        let id_obj: RawVal = host
            .invoke_function(HostFunction::CreateContract(CreateContractArgs {
                contract_id: ContractId::Asset(asset),
                source: ScContractCode::Token,
            }))
            .unwrap()
            .try_into_val(host)
            .unwrap();
        Self {
            id: BytesN::<32>::try_from_val(host, &id_obj).unwrap(),
            host,
        }
    }

    pub(crate) fn allowance(&self, from: Address, spender: Address) -> Result<i128, HostError> {
        Ok(self
            .host
            .call(
                self.id.clone().into(),
                Symbol::from_str("allowance"),
                host_vec![self.host, from, spender].into(),
            )?
            .try_into_val(self.host)?)
    }

    fn call_with_single_signer(
        &self,
        signer: &TestSigner,
        function_name: &str,
        args: HostVec,
    ) -> Result<(), HostError> {
        authorize_single_invocation(self.host, signer, &self.id, function_name, args.clone());
        Ok(self
            .host
            .call(
                self.id.clone().into(),
                Symbol::from_str(function_name),
                args.into(),
            )?
            .try_into()?)
    }

    pub(crate) fn incr_allow(
        &self,
        from: &TestSigner,
        spender: Address,
        amount: i128,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            from,
            "incr_allow",
            host_vec![self.host, from.address(self.host), spender, amount],
        )
    }

    pub(crate) fn decr_allow(
        &self,
        from: &TestSigner,
        spender: Address,
        amount: i128,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            from,
            "decr_allow",
            host_vec![self.host, from.address(self.host), spender, amount],
        )
    }

    pub(crate) fn balance(&self, addr: Address) -> Result<i128, HostError> {
        Ok(self
            .host
            .call(
                self.id.clone().into(),
                Symbol::from_str("balance"),
                host_vec![self.host, addr].into(),
            )?
            .try_into_val(self.host)?)
    }

    pub(crate) fn spendable(&self, addr: Address) -> Result<i128, HostError> {
        Ok(self
            .host
            .call(
                self.id.clone().into(),
                Symbol::from_str("spendable"),
                host_vec![self.host, addr].into(),
            )?
            .try_into_val(self.host)?)
    }

    pub(crate) fn authorized(&self, addr: Address) -> Result<bool, HostError> {
        Ok(self
            .host
            .call(
                self.id.clone().into(),
                Symbol::from_str("authorized"),
                host_vec![self.host, addr].into(),
            )?
            .try_into_val(self.host)?)
    }

    pub(crate) fn xfer(
        &self,
        from: &TestSigner,
        to: Address,
        amount: i128,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            from,
            "xfer",
            host_vec![self.host, from.address(self.host), to, amount],
        )
    }

    pub(crate) fn xfer_from(
        &self,
        spender: &TestSigner,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            spender,
            "xfer_from",
            host_vec![self.host, spender.address(self.host), from, to, amount],
        )
    }

    pub(crate) fn burn(&self, from: &TestSigner, amount: i128) -> Result<(), HostError> {
        self.call_with_single_signer(
            from,
            "burn",
            host_vec![self.host, from.address(self.host), amount],
        )
    }

    pub(crate) fn burn_from(
        &self,
        spender: &TestSigner,
        from: Address,
        amount: i128,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            spender,
            "burn_from",
            host_vec![self.host, spender.address(self.host), from, amount],
        )
    }

    pub(crate) fn set_auth(
        &self,
        admin: &TestSigner,
        addr: Address,
        authorize: bool,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            admin,
            "set_auth",
            host_vec![self.host, admin.address(self.host), addr, authorize],
        )
    }

    pub(crate) fn mint(
        &self,
        admin: &TestSigner,
        to: Address,
        amount: i128,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            admin,
            "mint",
            host_vec![self.host, admin.address(self.host), to, amount],
        )
    }

    pub(crate) fn clawback(
        &self,
        admin: &TestSigner,
        from: Address,
        amount: i128,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            admin,
            "clawback",
            host_vec![self.host, admin.address(self.host), from, amount],
        )
    }

    pub(crate) fn set_admin(
        &self,
        admin: &TestSigner,
        new_admin: Address,
    ) -> Result<(), HostError> {
        self.call_with_single_signer(
            admin,
            "set_admin",
            host_vec![self.host, admin.address(self.host), new_admin],
        )
    }

    pub(crate) fn decimals(&self) -> Result<u32, HostError> {
        Ok(self
            .host
            .call(
                self.id.clone().into(),
                Symbol::from_str("decimals"),
                host_vec![self.host].into(),
            )?
            .try_into()?)
    }

    pub(crate) fn name(&self) -> Result<Bytes, HostError> {
        Ok(self
            .host
            .call(
                self.id.clone().into(),
                Symbol::from_str("name"),
                host_vec![self.host].into(),
            )?
            .try_into_val(self.host)?)
    }

    pub(crate) fn symbol(&self) -> Result<Bytes, HostError> {
        Ok(self
            .host
            .call(
                self.id.clone().into(),
                Symbol::from_str("symbol"),
                host_vec![self.host].into(),
            )?
            .try_into_val(self.host)?)
    }
}
