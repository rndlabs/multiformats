use parse_int;
use reqwest::blocking::get;

use super::*;

#[test]
fn test_multicodec_spec() {
    let spec = {
        let uri = "https://raw.githubusercontent.com/multiformats/multicodec/master/table.csv";
        get(uri).unwrap().text().unwrap()
    };

    let spec_lines: Vec<Vec<String>> = {
        let mut total_lines: Vec<String> = {
            let iter = spec.lines().map(|s| s.to_string());
            iter.collect()
        };
        total_lines.remove(0); // remove the column header.
        let mut total_lines: Vec<Vec<String>> = total_lines
            .into_iter()
            .map(|s| {
                let mut row = s
                    .split(",")
                    .map(|col| col.trim().to_string())
                    .collect::<Vec<String>>();
                row.pop(); // remove the description;
                if row[2] == "0x00" {
                    row[2] = "0".to_string();
                }
                let code: u32 = parse_int::parse(&row[2]).expect(&format!("{}", row[2]));
                vec![row.remove(0), format!("0x{:x}", code), row.remove(0)] // re-order colums
            })
            .collect();
        // remove (ipfs, multiaddr, 0x01a5, libp2p (deprecated))
        assert_eq!(total_lines[110][0], "ipfs", "{:?}", total_lines[99]);
        assert_eq!(total_lines[110][1], "0x1a5", "{:?}", total_lines[99]);
        assert_eq!(total_lines[110][2], "multiaddr", "{:?}", total_lines[99]);
        total_lines.remove(110);

        for _ in 0..224 {
            total_lines.remove(253);
        }
        total_lines
    };

    let pkg_lines: Vec<Vec<String>> = (&TABLE)
        .to_vec()
        .into_iter()
        .map(|cp| vec![cp.name, format!("0x{:x}", cp.code), cp.tag])
        .collect();

    println!("{:?}", pkg_lines);
    assert_eq!(
        spec_lines.len(),
        pkg_lines.len(),
        "{} {}",
        spec_lines.len(),
        pkg_lines.len()
    );

    for (x, y) in spec_lines.into_iter().zip(pkg_lines.into_iter()) {
        println!("{:?} {:?}", x, y);
        assert_eq!(x[0], y[0], "{:?}, {:?}", x, y);
        assert_eq!(x[1], y[1], "{:?}, {:?}", x, y);
        assert_eq!(x[2], y[2], "{:?}, {:?}", x, y);
    }
}

#[test]
fn test_codec() {
    for entry in TABLE.iter() {
        let code: Multicodec = entry.into();

        let buf = code.encode().unwrap();

        let (res_code, res_buf) = Multicodec::decode(&buf).unwrap();
        assert_eq!(res_code, code, "{:?}", code);
        assert_eq!(res_buf, vec![].as_slice(), "{:?}", code);
    }
}
