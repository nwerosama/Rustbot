use tokio::{
  select,
  signal::unix::{
    SignalKind,
    signal
  }
};

pub async fn gracefully_shutdown() {
  let [mut s1, mut s2] = [signal(SignalKind::interrupt()).unwrap(), signal(SignalKind::hangup()).unwrap()];

  select!(
    v = s1.recv() => v.unwrap(),
    v = s2.recv() => v.unwrap()
  );

  println!("\nRustbot says goodbye! 👋");
}
