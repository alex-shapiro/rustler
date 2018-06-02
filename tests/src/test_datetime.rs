use rustler::{Env, Term, NifResult, Encoder};
use chrono::{DateTime, Utc};

pub fn datetime_echo<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let datetime: DateTime<Utc> = args[0].decode()?;
    Ok(datetime.encode(env))
}
