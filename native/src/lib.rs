use napi::Env;
use napi_derive::napi;

#[napi]
pub fn no_output() {
  println!("Output not appearing in console");
}

#[napi]
pub fn has_output(env: Env) {
  let _ = env.run_script::<_, ()>("console.log('This output will appear in the console')");
}
