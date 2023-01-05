use yaterbilang as terbilang;

#[test]
fn test_zero() {
    assert_eq!(terbilang::from(0), String::from("nol"));
}

#[test]
fn test_basic_number_8() {
    assert_eq!(terbilang::from(8), String::from("delapan"));
}

#[test]
fn test_basic_number_10() {
    assert_eq!(terbilang::from(10), String::from("sepuluh"));
}

#[test]
fn test_basic_number_11() {
    assert_eq!(terbilang::from(11), String::from("sebelas"));
}

#[test]
fn test_teens_12() {
    assert_eq!(terbilang::from(12), String::from("dua belas"));
}

#[test]
fn test_teens_13() {
    assert_eq!(terbilang::from(13), String::from("tiga belas"));
}

#[test]
fn test_teens_19() {
    assert_eq!(terbilang::from(19), String::from("sembilan belas"));
}

#[test]
fn test_tens_20() {
    assert_eq!(terbilang::from(20), String::from("dua puluh"));
}

#[test]
fn test_tens_21() {
    assert_eq!(terbilang::from(21), String::from("dua puluh satu"));
}

#[test]
fn test_tens_99() {
    assert_eq!(terbilang::from(99), String::from("sembilan puluh sembilan"));
}

#[test]
fn test_hundred_100() {
    assert_eq!(terbilang::from(100), String::from("seratus"));
}

#[test]
fn test_hundred_101() {
    assert_eq!(terbilang::from(101), String::from("seratus satu"));
}

#[test]
fn test_hundred_110() {
    assert_eq!(terbilang::from(110), String::from("seratus sepuluh"));
}

#[test]
fn test_hundred_111() {
    assert_eq!(terbilang::from(111), String::from("seratus sebelas"));
}

#[test]
fn test_hundred_112() {
    assert_eq!(terbilang::from(112), String::from("seratus dua belas"));
}

#[test]
fn test_hundred_117() {
    assert_eq!(terbilang::from(117), String::from("seratus tujuh belas"));
}

#[test]
fn test_hundred_159() {
    assert_eq!(terbilang::from(159), String::from("seratus lima puluh sembilan"));
}

#[test]
fn test_hundred_603() {
    assert_eq!(terbilang::from(603), String::from("enam ratus tiga"));
}

#[test]
fn test_hundred_999() {
    assert_eq!(terbilang::from(999), String::from("sembilan ratus sembilan puluh sembilan"));
}

#[test]
fn test_thousand_1000() {
    assert_eq!(terbilang::from(1000), String::from("seribu"));
}

#[test]
fn test_thousand_1001() {
    assert_eq!(terbilang::from(1001), String::from("seribu satu"));
}

#[test]
fn test_thousand_1010() {
    assert_eq!(terbilang::from(1010), String::from("seribu sepuluh"));
}

#[test]
fn test_thousand_1013() {
    assert_eq!(terbilang::from(1013), String::from("seribu tiga belas"));
}

#[test]
fn test_thousand_1100() {
    assert_eq!(terbilang::from(1100), String::from("seribu seratus"));
}

#[test]
fn test_thousand_1101() {
    assert_eq!(terbilang::from(1101), String::from("seribu seratus satu"));
}

#[test]
fn test_thousand_1199() {
    assert_eq!(terbilang::from(1199), String::from("seribu seratus sembilan puluh sembilan"));
}
