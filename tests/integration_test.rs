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

#[test]
fn test_thousand_7898() {
    assert_eq!(terbilang::from(7898), String::from("tujuh ribu delapan ratus sembilan puluh delapan"));
}

#[test]
fn test_tens_of_thousands_10000() {
    assert_eq!(terbilang::from(10_000), String::from("sepuluh ribu"));
}

#[test]
fn test_tens_of_thousands_10001() {
    assert_eq!(terbilang::from(10_001), String::from("sepuluh ribu satu"));
}

#[test]
fn test_tens_of_thousands_10010() {
    assert_eq!(terbilang::from(10_010), String::from("sepuluh ribu sepuluh"));
}

#[test]
fn test_tens_of_thousands_10101() {
    assert_eq!(terbilang::from(10_101), String::from("sepuluh ribu seratus satu"));
}

#[test]
fn test_tens_of_thousands_12345() {
    assert_eq!(terbilang::from(12_345), String::from("dua belas ribu tiga ratus empat puluh lima"));
}


#[test]
fn test_tens_of_thousands_99999() {
    assert_eq!(terbilang::from(99_999), String::from("sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan"));
}

#[test]
fn test_hundreds_of_thousands_100000() {
    assert_eq!(terbilang::from(100_000), String::from("seratus ribu"));
}

#[test]
fn test_hundreds_of_thousands_100001() {
    assert_eq!(terbilang::from(100_001), String::from("seratus ribu satu"));
}

#[test]
fn test_hundreds_of_thousands_123456() {
    assert_eq!(terbilang::from(123_456), String::from("seratus dua puluh tiga ribu empat ratus lima puluh enam"));
}

#[test]
fn test_hundreds_of_thousands_109123() {
    assert_eq!(terbilang::from(109_123), String::from("seratus sembilan ribu seratus dua puluh tiga"));
}

#[test]
fn test_hundreds_of_thousands_999999() {
    assert_eq!(terbilang::from(999_999), String::from("sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan"));
}

#[test]
fn test_millions_1000000() {
    assert_eq!(terbilang::from(1_000_000), String::from("satu juta"));
}

#[test]
fn test_millions_1234567() {
    assert_eq!(terbilang::from(1_234_567), String::from("satu juta dua ratus tiga puluh empat ribu lima ratus enam puluh tujuh"));
}

#[test]
fn test_millions_9999999() {
    assert_eq!(terbilang::from(9_999_999), String::from("sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan"));
}

#[test]
fn test_tens_of_millions_10000000() {
    assert_eq!(terbilang::from(10_000_000), String::from("sepuluh juta"));
}

#[test]
fn test_tens_of_millions_99999999() {
    assert_eq!(terbilang::from(99_999_999), String::from("sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan"));
}

#[test]
fn test_hundreds_of_millions_100000000() {
    assert_eq!(terbilang::from(100_000_000), String::from("seratus juta"));
}

#[test]
fn test_hundreds_of_millions_101001000() {
    assert_eq!(terbilang::from(101_001_000), String::from("seratus satu juta seribu"));
}

#[test]
fn test_hundreds_of_millions_109001234() {
    assert_eq!(terbilang::from(109_001_234), String::from("seratus sembilan juta seribu dua ratus tiga puluh empat"));
}

#[test]
fn test_hundreds_of_millions_999999999() {
    assert_eq!(terbilang::from(999_999_999), String::from("sembilan ratus sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan"));
}

#[test]
fn test_billions_1000000000() {
    assert_eq!(terbilang::from(1_000_000_000), String::from("satu triliun"));
}

#[test]
fn test_billions_1234567890() {
    assert_eq!(terbilang::from(1_234_567_890), String::from("satu triliun dua ratus tiga puluh empat juta lima ratus enam puluh tujuh ribu delapan ratus sembilan puluh"));
}

#[test]
fn test_billions_9999999999() {
    assert_eq!(terbilang::from(9_999_999_999), String::from("sembilan triliun sembilan ratus sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan"));
}

#[test]
fn test_tens_of_billions_10000000000() {
    assert_eq!(terbilang::from(10_000_000_000), String::from("sepuluh triliun"));
}

#[test]
fn test_tens_of_billions_12345678901() {
    assert_eq!(terbilang::from(12_345_678_901), String::from("dua belas triliun tiga ratus empat puluh lima juta enam ratus tujuh puluh delapan ribu sembilan ratus satu"));
}

#[test]
fn test_tens_of_billions_99999999999() {
    assert_eq!(terbilang::from(99_999_999_999), String::from("sembilan puluh sembilan triliun sembilan ratus sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan"));
}

#[test]
fn test_hundreds_of_billions_100000000000() {
    assert_eq!(terbilang::from(100_000_000_000), String::from("seratus triliun"));
}

#[test]
fn test_hundreds_of_billions_123456789012() {
    assert_eq!(terbilang::from(123_456_789_012), String::from("seratus dua puluh tiga triliun empat ratus lima puluh enam juta tujuh ratus delapan puluh sembilan ribu dua belas"));
}

#[test]
fn test_hundreds_of_billions_999999999999() {
    assert_eq!(terbilang::from(999_999_999_999), String::from("sembilan ratus sembilan puluh sembilan triliun sembilan ratus sembilan puluh sembilan juta sembilan ratus sembilan puluh sembilan ribu sembilan ratus sembilan puluh sembilan"));
}
