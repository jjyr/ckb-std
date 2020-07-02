use crate::ckb_constants::*;
use crate::debug;
use crate::error::SysError;
use crate::syscalls;
use alloc::vec::Vec;
use ckb_types::{packed::*, prelude::*};

/// Default buffer size
pub const BUF_SIZE: usize = 1024;

/// Load tx hash
///
/// Return the tx hash or a syscall error
///
/// # Example
///
/// ```
/// let tx_hash = load_tx_hash().unwrap();
/// ```
pub fn load_tx_hash() -> Result<[u8; 32], SysError> {
    let mut hash = [0u8; 32];
    let len = syscalls::load_tx_hash(&mut hash, 0)?;
    debug_assert_eq!(hash.len(), len);
    Ok(hash)
}

/// Load script hash
///
/// Return the script hash or a syscall error
///
/// # Example
///
/// ```
/// let script_hash = load_script_hash().unwrap();
/// ```
pub fn load_script_hash() -> Result<[u8; 32], SysError> {
    let mut hash = [0u8; 32];
    let len = syscalls::load_script_hash(&mut hash, 0)?;
    debug_assert_eq!(hash.len(), len);
    Ok(hash)
}

/// Common method to fully load data from syscall
fn load_data<F: Fn(&mut [u8], usize) -> Result<usize, SysError>>(
    syscall: F,
) -> Result<Vec<u8>, SysError> {
    let mut buf = [0u8; BUF_SIZE];
    match syscall(&mut buf, 0) {
        Ok(len) => Ok(buf[..len].to_vec()),
        Err(SysError::LengthNotEnough(actual_size)) => {
            let mut data = Vec::with_capacity(actual_size);
            let loaded_len = buf.len();
            data[..loaded_len].copy_from_slice(&buf);
            let len = syscall(&mut data[loaded_len..], loaded_len)?;
            debug_assert_eq!(len + loaded_len, actual_size);
            Ok(data)
        }
        Err(err) => Err(err),
    }
}

/// Load cell
///
/// Return the cell or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let cell_output = load_cell(0, Source::Input).unwrap();
/// ```
pub fn load_cell(index: usize, source: Source) -> Result<CellOutput, SysError> {
    let data = load_data(|buf, offset| syscalls::load_cell(buf, offset, index, source))?;

    match CellOutputReader::verify(&data, false) {
        Ok(()) => Ok(CellOutput::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load input
///
/// Return the input or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let input = load_input(0, Source::Input).unwrap();
/// ```
pub fn load_input(index: usize, source: Source) -> Result<CellInput, SysError> {
    let data = load_data(|buf, offset| syscalls::load_input(buf, offset, index, source))?;

    match CellInputReader::verify(&data, false) {
        Ok(()) => Ok(CellInput::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load header
///
/// Return the header or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let header = load_header(0, Source::HeaderDep).unwrap();
/// ```
pub fn load_header(index: usize, source: Source) -> Result<Header, SysError> {
    let data = load_data(|buf, offset| syscalls::load_header(buf, offset, index, source))?;

    match HeaderReader::verify(&data, false) {
        Ok(()) => Ok(Header::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load witness args
///
/// Return the witness args or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let witness_args = load_witness_args(0, Source::Input).unwrap();
/// ```
pub fn load_witness_args(index: usize, source: Source) -> Result<WitnessArgs, SysError> {
    let data = load_data(|buf, offset| syscalls::load_witness(buf, offset, index, source))?;

    match WitnessArgsReader::verify(&data, false) {
        Ok(()) => Ok(WitnessArgs::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load transaction
///
/// Return the transaction or a syscall error
///
/// # Example
///
/// ```
/// let tx = load_transaction().unwrap();
/// ```
pub fn load_transaction() -> Result<Transaction, SysError> {
    let data = load_data(|buf, offset| syscalls::load_transaction(buf, offset))?;

    match TransactionReader::verify(&data, false) {
        Ok(()) => Ok(Transaction::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load cell capacity
///
/// Return the loaded data length or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let capacity = syscalls::load_cell_capacity(index, source).unwrap();
/// ```
pub fn load_cell_capacity(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::Capacity)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load cell occupied capacity
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let occupied_capacity = load_cell_occupied_capacity(index, source).unwrap();
/// ```
pub fn load_cell_occupied_capacity(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len =
        syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::OccupiedCapacity)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load cell data hash
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let data_hash = load_cell_data_hash(index, source).unwrap();
/// ```
pub fn load_cell_data_hash(index: usize, source: Source) -> Result<[u8; 32], SysError> {
    let mut buf = [0u8; 32];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::DataHash)?;
    debug_assert_eq!(len, buf.len());
    Ok(buf)
}

/// Load cell lock hash
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let lock_hash = load_cell_lock_hash(index, source).unwrap();
/// ```
pub fn load_cell_lock_hash(index: usize, source: Source) -> Result<[u8; 32], SysError> {
    let mut buf = [0u8; 32];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::LockHash)?;
    debug_assert_eq!(len, buf.len());
    Ok(buf)
}

/// Load cell type hash
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let type_hash = load_cell_type_hash(index, source).unwrap();
/// ```
pub fn load_cell_type_hash(index: usize, source: Source) -> Result<[u8; 32], SysError> {
    let mut buf = [0u8; 32];
    let len = syscalls::load_cell_by_field(&mut buf, 0, index, source, CellField::TypeHash)?;
    debug_assert_eq!(len, buf.len());
    Ok(buf)
}

/// Load cell lock
///
/// Return the lock script or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let lock = load_cell_lock(index, source).unwrap();
/// ```
pub fn load_cell_lock(index: usize, source: Source) -> Result<Script, SysError> {
    let data = load_data(|buf, offset| {
        syscalls::load_cell_by_field(buf, offset, index, source, CellField::Lock)
    })?;

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Script::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load cell type
///
/// Return the type script or a syscall error
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let type_script = load_cell_type(index, source).unwrap();
/// ```
pub fn load_cell_type(index: usize, source: Source) -> Result<Script, SysError> {
    let data = load_data(|buf, offset| {
        syscalls::load_cell_by_field(buf, offset, index, source, CellField::Type)
    })?;

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Script::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

// Load header epoch number
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let epoch_number = load_header_epoch_number(index, source).unwrap();
/// ```
pub fn load_header_epoch_number(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_header_by_field(&mut buf, 0, index, source, HeaderField::EpochNumber)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load header epoch start block number
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let epoch_start_block_number = load_header_epoch_start_block_number(index, source).unwrap();
/// ```
pub fn load_header_epoch_start_block_number(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_header_by_field(
        &mut buf,
        0,
        index,
        source,
        HeaderField::EpochStartBlockNumber,
    )?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load header epoch length
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let epoch_length = load_header_epoch_length(index, source).unwrap();
/// ```
pub fn load_header_epoch_length(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_header_by_field(&mut buf, 0, index, source, HeaderField::EpochLength)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load input since
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let since = load_input_since(index, source).unwrap();
/// ```
pub fn load_input_since(index: usize, source: Source) -> Result<u64, SysError> {
    let mut buf = [0u8; 8];
    let len = syscalls::load_input_by_field(&mut buf, 0, index, source, InputField::Since)?;
    debug_assert_eq!(len, buf.len());
    Ok(u64::from_le_bytes(buf))
}

/// Load input out point
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let out_point = load_input_out_point(index, source).unwrap();
/// ```
pub fn load_input_out_point(index: usize, source: Source) -> Result<OutPoint, SysError> {
    let mut buf = [0u8; 36];
    let len = syscalls::load_input_by_field(&mut buf, 0, index, source, InputField::OutPoint)?;
    debug_assert_eq!(len, buf.len());
    match OutPointReader::verify(&buf, false) {
        Ok(()) => Ok(OutPoint::new_unchecked(buf.to_vec().into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// Load cell data
///
/// # Arguments
///
/// * `index` - index
/// * `source` - source
///
/// # Example
///
/// ```
/// let data = load_cell_data(index, source).unwrap();
/// ```
pub fn load_cell_data(index: usize, source: Source) -> Result<Vec<u8>, SysError> {
    load_data(|buf, offset| syscalls::load_cell_data(buf, offset, index, source))
}

/// Load script
///
/// # Example
///
/// ```
/// let script = load_script().unwrap();
/// ```
pub fn load_script() -> Result<Script, SysError> {
    let data = load_data(|buf, offset| syscalls::load_script(buf, offset))?;

    match ScriptReader::verify(&data, false) {
        Ok(()) => Ok(Script::new_unchecked(data.into())),
        Err(_err) => Err(SysError::Encoding),
    }
}

/// QueryIter
///
/// A advanced iterator to manipulate cells/inputs/headers/witnesses
///
/// # Example
///
/// ```
/// // calculate all inputs capacity
/// let inputs_capacity = QueryIter::new(load_cell, Source::Input)
/// .map(|cell| cell.capacity().unpack())
/// .sum::<u64>();
///
/// // calculate all outputs capacity
/// let outputs_capacity = QueryIter::new(load_cell, Source::Output)
/// .map(|cell| cell.capacity().unpack())
/// .sum::<u64>();
///
/// assert_eq!(inputs_capacity, outputs_capacity);
/// ```
pub struct QueryIter<F> {
    query_fn: F,
    index: usize,
    source: Source,
}

impl<F> QueryIter<F> {
    pub fn new(query_fn: F, source: Source) -> Self {
        QueryIter {
            query_fn,
            index: 0,
            source,
        }
    }
}

impl<T, F: Fn(usize, Source) -> Result<T, SysError>> Iterator for QueryIter<F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.query_fn)(self.index, self.source) {
            Ok(item) => {
                self.index += 1;
                Some(item)
            }
            Err(SysError::IndexOutOfBound) => None,
            Err(err) => {
                debug!("QueryIter error {:?}", err);
                panic!("QueryIter query_fn return an error")
            }
        }
    }
}
