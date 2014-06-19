use std::str::from_utf8_lossy;
use {handle};
use super::server;

#[test]
pub fn test_get_requests() {
  let srv = server!(
    recv!(
      "GET / HTTP/1.1\r\n\
       Host: localhost:8482\r\n\
       Accept: */*\r\n\r\n"),
    send!(
      "HTTP/1.1 200 OK\r\n\
       Content-Length: 5\r\n\r\n\
       Hello\r\n\r\n"),
    recv!(
      "GET /next HTTP/1.1\r\n\
       Host: localhost:8482\r\n\
       Accept: */*\r\n\r\n"),
    send!(
      "HTTP/1.1 200 OK\r\n\
       Content-Length: 5\r\n\r\n\
       World\r\n\r\n"));

  let mut handle = handle();
  let res1 = handle.get("http://localhost:8482").exec().unwrap();
  let res2 = handle.get("http://localhost:8482/next").exec().unwrap();

  srv.assert();

  assert!(res1.get_code() == 200);
  assert!(res1.get_body() == "Hello".as_bytes());

  assert!(res2.get_code() == 200);
  assert!(res2.get_body() == "World".as_bytes());
}

#[test]
pub fn test_post_get_requests() {
  let srv = server!(
    recv!(
      "POST / HTTP/1.1\r\n\
       Host: localhost:8482\r\n\
       Accept: */*\r\n\
       Content-Length: 5\r\n\
       Content-Type: application/octet-stream\r\n\
       \r\n\
       Hello"),
    send!(
      "HTTP/1.1 200 OK\r\n\
       Content-Length: 5\r\n\r\n\
       World\r\n\r\n"),
    recv!(
      "GET /next HTTP/1.1\r\n\
       Host: localhost:8482\r\n\
       Accept: */*\r\n\r\n"),
    send!(
      "HTTP/1.1 200 OK\r\n\
       Content-Length: 4\r\n\r\n\
       NEXT\r\n\r\n"));

  let mut handle = handle();
  let res1 = handle.post("http://localhost:8482", "Hello").exec().unwrap();
  let res2 = handle.get("http://localhost:8482/next").exec().unwrap();

  srv.assert();

  assert!(res1.get_code() == 200);
  assert!(res1.get_body() == "World".as_bytes(), "actual={}", from_utf8_lossy(res1.get_body()));

  assert!(res2.get_code() == 200);
  assert!(res2.get_body() == "NEXT".as_bytes(), "actual={}", from_utf8_lossy(res2.get_body()));
}
