use instructions::{extract_opcode, OpCode, Operand, OPCODE_OFFSET};


#[test]
fn test_opcode_from_and_to_u8() {
    assert_eq!(OpCode::from_u8(1), Some(OpCode::LDC));
    assert_eq!(OpCode::from_u8(8), Some(OpCode::ADL));
    assert_eq!(OpCode::from_u8(99), None);

    assert_eq!(OpCode::LDC.to_u8(), 1);
    assert_eq!(OpCode::ADL.to_u8(), 8);
}

#[test]
fn test_opcode_to_u32() {
    assert_eq!(OpCode::RET.to_u32(), 6);
}

#[test]
fn test_extract_opcode() {
    let encoded = (1u32 << OPCODE_OFFSET) | 0x00FFFFFF;
    assert_eq!(extract_opcode(encoded), Some(OpCode::LDC));
}

#[test]
fn test_ldc_encode_decode() {
    let op = OpCode::LDC;
    // layout = [RegAddr, ConstIdx]
    // operand order matches the layout order (RegAddr, ConstIdx)
    let operands = vec![3, 1234];

    let bytes = op.encode(operands.clone());
    let encoded = u32::from_be_bytes(bytes);
    let decoded = op.decode(encoded);

    assert_eq!(decoded, operands);
}

#[test]
fn test_sat_encode_decode() {
    let op = OpCode::SAT;
    let operands = vec![1, 2, 3];
    let bytes = op.encode(operands.clone());
    let encoded = u32::from_be_bytes(bytes);
    let decoded = op.decode(encoded);
    assert_eq!(decoded, operands);
}

#[test]
fn test_mtk_encode_decode() {
    let op = OpCode::MTK;
    let operands = vec![2, 55];
    let bytes = op.encode(operands.clone());
    let encoded = u32::from_be_bytes(bytes);
    let decoded = op.decode(encoded);
    assert_eq!(decoded, operands);
}

#[test]
fn test_roundtrip_all_opcodes() {
    for code in 1..=8 {
        let op = OpCode::from_u8(code).unwrap();
        let operands = op.layout().iter().map(|operand| {
            match operand {
                Operand::RegAddr => 0xF,   // max reg value (4 bits)
                Operand::ConstIdx => 0xFFFFF, // max const value (20 bits)
            }
        }).collect::<Vec<_>>();

        let bytes = op.encode(operands.clone());
        let encoded = u32::from_be_bytes(bytes);
        let decoded = op.decode(encoded);
        assert_eq!(decoded, operands, "failed for {:?}", op);
    }
}


