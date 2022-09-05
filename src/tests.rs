use chrono::{Utc, TimeZone};

use crate::sign;

#[test]
fn test_sign() {
    let expire_at = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);
    let key = std::fs::read_to_string("dummy.pem").unwrap();

    let signed_url = sign::sign("https://example.com/hello.txt", expire_at, "ABC123", &key).unwrap();
    assert_eq!(signed_url, "https://example.com/hello.txt?Expires=1640995200&Signature=PiCEpLIwrJ%7EOcbGevw5G50sy1dLCUeU8nXiBciIKLSRYyuvapw5-TIksRYuiib8TZ844Atj7lQ7NJwS3MEFIhUTxQOawgZ0q8sphmA9ZRBTt-4Sti0prAHvrlskAG9nlnJ3YuoJJuBmDQ1bc5hlFZd-L2mmmnLlDysrTspkx9arZsxwqO7t0m8KEKH5z4Nd5MB0DJnRn5Hm7TfJXjx-3f%7EH3b7MDRTGv70Ycp14qslV3KfPj8T1x4ETfVlit2L%7Er0CN8OktIS5PjdDL9pehJtRp-m4AghWsZSz8E-a0-QcyO1bRvvMefKC6n3tB7niVBUrelbLRf0udvFbGDBcLmhg__&Key-Pair-Id=ABC123")
}

