use assert_cmd::Command;

#[test]
fn works() {
    let mut cmd = Command::cargo_bin("hello").unwrap(); // Cargoによってビルドされたバイナリのパスを取得する
    cmd.assert().success();
}
