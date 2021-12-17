#[test]
fn day16p01_sample1() {
    let data = "8A004A801A8002F478".to_string();
    assert_eq!(day16::part1(data), 16)
}

#[test]
fn day16p01_sample2() {
    let data = "620080001611562C8802118E34".to_string();
    assert_eq!(day16::part1(data), 12)
}

#[test]
fn day16p01_sample3() {
    let data = "C0015000016115A2E0802F182340".to_string();
    assert_eq!(day16::part1(data), 23)
}

#[test]
fn day16p01_sample4() {
    let data = "A0016C880162017C3686B18A3D4780".to_string();
    assert_eq!(day16::part1(data), 31)
}

#[test]
fn day16p01() {
    assert_eq!(day16::part1(util::read_input("../..", 16)), 979)
}

#[test]
fn day16p02_sample1() {
    let data = "C200B40A82".to_string();
    assert_eq!(day16::part2(data), 3)
}

#[test]
fn day16p02_sample2() {
    let data = "04005AC33890".to_string();
    assert_eq!(day16::part2(data), 54)
}

#[test]
fn day16p02_sample3() {
    let data = "880086C3E88112".to_string();
    assert_eq!(day16::part2(data), 7)
}

#[test]
fn day16p02_sample4() {
    let data = "CE00C43D881120".to_string();
    assert_eq!(day16::part2(data), 9)
}

#[test]
fn day16p02_sample5() {
    let data = "D8005AC2A8F0".to_string();
    assert_eq!(day16::part2(data), 1)
}

#[test]
fn day16p02_sample6() {
    let data = "F600BC2D8F".to_string();
    assert_eq!(day16::part2(data), 0)
}

#[test]
fn day16p02_sample7() {
    let data = "9C005AC2F8F0".to_string();
    assert_eq!(day16::part2(data), 0)
}

#[test]
fn day16p02_sample8() {
    let data = "9C0141080250320F1802104A08".to_string();
    assert_eq!(day16::part2(data), 1)
}

#[test]
fn day16p02() {
    assert_eq!(day16::part2(util::read_input("../..", 16)), 277110354175)
}
