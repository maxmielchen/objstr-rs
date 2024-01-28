
use std::{io::SeekFrom, path::Path};

use objstr::{api::ObjStr as _, file::FileObjStr};

fn setup(name: &str) -> FileObjStr {
    FileObjStr::new(Path::new(
        format!("target/tmp/example_{}.bin", name).as_str()
    )).unwrap()
}

fn teardown(name: &str) {
    std::fs::remove_file(
        format!("target/tmp/example_{}.bin", name).as_str()
    ).unwrap();
}

#[test] 
fn test_example(){
    let mut str = setup("test_example");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();
    str.append(b"Hello, world3".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world1".to_vec());
    assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());
    assert_eq!(str.read().unwrap(), b"Hello, world3".to_vec());
    assert!(str.read().is_err());

    str.seek(SeekFrom::Start(0)).unwrap();
    str.seek(SeekFrom::Current(1)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());
    assert_eq!(str.read().unwrap(), b"Hello, world3".to_vec());

    str.seek(SeekFrom::Current(-2)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());
    assert_eq!(str.read().unwrap(), b"Hello, world3".to_vec());

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world1".to_vec());

    str.seek(SeekFrom::End(-1)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world3".to_vec());

    str.seek(SeekFrom::End(0)).unwrap();

    str.cut().unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.len(1, 1).unwrap(), 13);

    assert_eq!(str.len(1, 2).unwrap(), 26 + 8);

    str.seek(SeekFrom::Current(1)).unwrap();

    assert_eq!(str.len(1, 1).unwrap(), 13);

    teardown("test_example");
}