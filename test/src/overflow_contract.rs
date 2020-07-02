use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};
use std::fs::File;
use std::io::Read;

const MAX_CYCLES: u64 = 100_000_000;
const RT_ERROR_STACK_OVERFLOW: i8 = -128;

#[test]
fn it_works() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin = {
        let mut buf = Vec::new();
        File::open("overflow-contract/target/riscv64imac-unknown-none-elf/release/overflow-contract")
            .unwrap()
            .read_to_end(&mut buf)
            .expect("read code");
        Bytes::from(buf)
    };
    let contract_out_point = context.deploy_contract(contract_bin);

    // prepare scripts
    let lock_script = context
        .build_script(&contract_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder().out_point(contract_out_point).build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    assert_error_eq!(err, ScriptError::ValidationFailure(RT_ERROR_STACK_OVERFLOW));
}
