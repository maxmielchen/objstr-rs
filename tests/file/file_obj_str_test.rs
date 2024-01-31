use std::{io::SeekFrom, path::Path};

use objstr::{api::ObjStr as _, file::FileObjStr};

fn setup(name: &str) -> FileObjStr {
    FileObjStr::new(Path::new(
        format!("target/tmp/file_obj_str_test_{}.bin", name).as_str()
    )).unwrap()
}

fn teardown(name: &str) -> Vec<u8> {
    let bytes = std::fs::read(
        format!("target/tmp/file_obj_str_test_{}.bin", name).as_str()
    ).unwrap();
    std::fs::remove_file(
        format!("target/tmp/file_obj_str_test_{}.bin", name).as_str()
    ).unwrap();
    bytes
}

#[test]
fn test_append() {
    let mut str = setup("test_append");

    str.append(b"Hello, world1".to_vec()).unwrap();

    let bytes = teardown("test_append");

    assert_eq!(bytes, b"\x00\x00\x00\x0dHello, world1\x00\x00\x00\x0d\x00".to_vec());
}

#[test]
fn test_read() {
    let mut str = setup("test_read");

    str.append(b"Hello, world1".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world1".to_vec());

    let _ = teardown("test_read");
}

#[test]
fn test_seek_start_forward() {
    let mut str = setup("test_seek_start_forward");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(1)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());

    let _ = teardown("test_seek_start_forward");
}

// #[test]
// fn test_seek_start_backward() {

// }

#[test]
fn test_seek_current_forward() {
    let mut str = setup("test_seek_current_forward");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();
    str.seek(SeekFrom::Current(1)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());

    let _ = teardown("test_seek_current_forward");
}

#[test]
fn test_seek_current_backward() {
    let mut str = setup("test_seek_current_backward");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::End(0)).unwrap();
    str.seek(SeekFrom::Current(-1)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());

    let _ = teardown("test_seek_current_backward");
}

// #[test]
// fn test_seek_end_forward() {

// }

#[test]
fn test_seek_end_backward() {
    let mut str = setup("test_seek_end_backward");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::End(-1)).unwrap();

    assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());

    let _ = teardown("test_seek_end_backward");
}


#[test]
fn test_len_0_0() {
    let mut str = setup("test_len_0_0");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.len(0, 0).unwrap(), 0);

    let _ = teardown("test_len_0_0");
}

#[test]
fn test_len_0_1() {
    let mut str = setup("test_len_0_1");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.len(0, 1).unwrap(), 13 + 8);

    let _ = teardown("test_len_0_1");
}

#[test]
fn test_len_1_0() {
    let mut str = setup("test_len_1_0");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert!(str.len(1, 0).is_err());

    let _ = teardown("test_len_1_0");
}

#[test]
fn test_len_1_1() {
    let mut str = setup("test_len_1_1");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.len(1, 1).unwrap(), 13);

    let _ = teardown("test_len_1_1");
}

#[test]
fn test_len_1_2() {
    let mut str = setup("test_len_1_2");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.len(1, 2).unwrap(), 13*2 + 8);

    let _ = teardown("test_len_1_2");
}

#[test]
fn test_len_2_1() {
    let mut str = setup("test_len_2_1");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(str.len(2, 1).unwrap(), 13+8 - 8*2);

    let _ = teardown("test_len_2_1");
}

#[test]
fn test_cut_random() {
    let mut str = setup("test_cut_random");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(1)).unwrap();

    str.cut().unwrap();

    let bytes = teardown("test_cut_random");

    assert_eq!(bytes, b"\x00\x00\x00\x0dHello, world1\x00\x00\x00\x0d\x00".to_vec());

    let _ = teardown("test_cut_random");
}

#[test]
fn test_cut_start() {
    let mut str = setup("test_cut_start");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    str.cut().unwrap();

    let bytes = teardown("test_cut_start");

    assert_eq!(bytes, b"\x00".to_vec());

    let _ = teardown("test_cut_start");
}

#[test]
fn test_cut_end() {
    let mut str = setup("test_cut_end");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::End(0)).unwrap();

    str.cut().unwrap();

    let bytes = teardown("test_cut_end");

    assert_eq!(bytes, b"\x00\x00\x00\x0dHello, world1\x00\x00\x00\x0d\x00\x00\x00\x0dHello, world2\x00\x00\x00\x0d\x00".to_vec());

    let _ = teardown("test_cut_end");
}

#[test]
fn test_overwrite_0_0() {
    let mut str = setup("test_overwrite_0_0");

    str.append(b"Hello, world1".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    str.overwrite(vec![], 0).unwrap();

    let bytes = teardown("test_overwrite_0_0");

    assert_eq!(bytes, b"\x00\x00\x00\x0dHello, world1\x00\x00\x00\x0d\x00".to_vec());
}

// #[test]
// fn test_overwrite_0_1() {

// }

#[test]
fn test_overwrite_1_1() {
    let mut str = setup("test_overwrite_1_1");

    str.append(b"Hello, world1".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    str.overwrite(vec![b"Hello, world2".to_vec()], 1).unwrap();

    let bytes = teardown("test_overwrite_1_1");

    assert_eq!(bytes, b"\x00\x00\x00\x0dHello, world2\x00\x00\x00\x0d\x00".to_vec());
}

#[test]
fn test_overwrite_1_2() {
    let mut str = setup("test_overwrite_1_2");

    str.append(b"Hello, world1".to_vec()).unwrap();
    str.append(b"Hello, world2".to_vec()).unwrap();

    str.seek(SeekFrom::Start(0)).unwrap();

    str.overwrite(vec![b"Hello, world2 --  -- Hello, world2".to_vec()], 2).unwrap();

    let bytes = teardown("test_overwrite_1_2");

    assert_eq!(bytes, b"\x00\x00\x00\x22Hello, world2 --  -- Hello, world2\x00\x00\x00\x22\x00".to_vec());
}

#[test]
fn test_overwrite_2_1() {
    let mut str = setup("test_overwrite_2_1");

    str.append(b"Hello, world1".to_vec()).unwrap();
    
    str.seek(SeekFrom::Start(0)).unwrap();

    println!("{:?}", str.len(2, 1).unwrap());

    str.overwrite(vec![b"Hey".to_vec(), b"Ey".to_vec()], 1).unwrap();

    let bytes = teardown("test_overwrite_2_1");

    assert_eq!(bytes, b"\x00\x00\x00\x03Hey\x00\x00\x00\x03\x00\x00\x00\x02Ey\x00\x00\x00\x02\x00".to_vec());
}



// MORE ERRORS














// OLD TESTS

// fn setup(name: &str) -> FileObjStr {
//     FileObjStr::new(Path::new(
//         format!("target/tmp/example_{}.bin", name).as_str()
//     )).unwrap()
// }

// fn teardown(name: &str) {
//     std::fs::remove_file(
//         format!("target/tmp/example_{}.bin", name).as_str()
//     ).unwrap();
// }

// #[test] 
// fn test_example(){
//     let mut str = setup("test_example");

//     str.append(b"Hello, world1".to_vec()).unwrap();
//     str.append(b"Hello, world2".to_vec()).unwrap();
//     str.append(b"Hello, world3".to_vec()).unwrap();

//     str.seek(SeekFrom::Start(0)).unwrap();

//     assert_eq!(str.read().unwrap(), b"Hello, world1".to_vec());
//     assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());
//     assert_eq!(str.read().unwrap(), b"Hello, world3".to_vec());
//     assert!(str.read().is_err());

//     str.seek(SeekFrom::Start(0)).unwrap();
//     str.seek(SeekFrom::Current(1)).unwrap();

//     assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());
//     assert_eq!(str.read().unwrap(), b"Hello, world3".to_vec());

//     str.seek(SeekFrom::Current(-2)).unwrap();

//     assert_eq!(str.read().unwrap(), b"Hello, world2".to_vec());
//     assert_eq!(str.read().unwrap(), b"Hello, world3".to_vec());

//     str.seek(SeekFrom::Start(0)).unwrap();

//     assert_eq!(str.read().unwrap(), b"Hello, world1".to_vec());

//     str.seek(SeekFrom::End(-1)).unwrap();

//     assert_eq!(str.read().unwrap(), b"Hello, world3".to_vec());

//     str.seek(SeekFrom::End(-1)).unwrap();

//     str.cut().unwrap();

//     str.seek(SeekFrom::Start(0)).unwrap();

//     assert_eq!(str.len(1, 1).unwrap(), 13);

//     assert_eq!(str.len(1, 2).unwrap(), 26 + 8);

//     str.append(b"h".to_vec()).unwrap();
//     str.seek(SeekFrom::End(-1)).unwrap();
//     assert_eq!(str.read().unwrap(), b"h".to_vec());
//     str.seek(SeekFrom::Current(-1)).unwrap();

//     assert!(str.len(2, 1).is_err());

//     str.overwrite(vec![b"b".to_vec()], 1).unwrap();

//     str.seek(SeekFrom::Start(0)).unwrap();
//     str.overwrite(vec![b"Hello, World!".to_vec()], 1).unwrap();

//     str.seek(SeekFrom::End(-1)).unwrap();
//     assert_eq!(str.read().unwrap(), b"b".to_vec());

//     str.seek(SeekFrom::Start(0)).unwrap();
//     assert_eq!(str.read().unwrap(), b"Hello, World!".to_vec());


//     str.seek(SeekFrom::Start(0)).unwrap();
//     str.overwrite(vec![b"Hello, World! --  -- Hello, World!".to_vec()], 2).unwrap();
//     str.seek(SeekFrom::Start(0)).unwrap();
//     assert_eq!(str.read().unwrap(), b"Hello, World! --  -- Hello, World!".to_vec());
//     assert_eq!(str.read().unwrap(), b"b".to_vec());

//     str.seek(SeekFrom::Start(0)).unwrap();

//     assert_eq!(str.len(1, 1).unwrap(), 26 + 8);
//     assert_eq!(str.len(2, 1).unwrap(), 26);

//     str.overwrite(vec![b"Hello, World!".to_vec(), b"Hello, World!".to_vec()], 1).unwrap();

//     teardown("test_example");
// }