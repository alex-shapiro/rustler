//! Automatically converts between chrono DateTime<Utc>
//! and Elixir.DateTime. As of Elixir 1.6, DateTime
//! does not support timezones beyond UTC.

use chrono::{self, Utc, Datelike, Timelike, TimeZone};
use Encoder;
use Env;
use Term;
use NifResult;
use Decoder;
use types;

impl Encoder for chrono::DateTime<Utc> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let mut map = types::elixir_struct::make_ex_struct(env, "Elixir.DateTime").ok().unwrap();
        map = map.map_put(atoms::calendar().to_term(env), atoms::calendar_iso().to_term(env)).ok().unwrap();
        map = map.map_put(atoms::day().to_term(env), self.day().encode(env)).ok().unwrap();
        map = map.map_put(atoms::hour().to_term(env), self.hour().encode(env)).ok().unwrap();
        map = map.map_put(atoms::microsecond().to_term(env), (self.nanosecond() / 1000, 6).encode(env)).ok().unwrap();
        map = map.map_put(atoms::minute().to_term(env), self.minute().encode(env)).ok().unwrap();
        map = map.map_put(atoms::month().to_term(env), self.month().encode(env)).ok().unwrap();
        map = map.map_put(atoms::second().to_term(env), self.second().encode(env)).ok().unwrap();
        map = map.map_put(atoms::std_offset().to_term(env), 0.encode(env)).ok().unwrap();
        map = map.map_put(atoms::time_zone().to_term(env), "Etc/UTC".encode(env)).ok().unwrap();
        map = map.map_put(atoms::utc_offset().to_term(env), 0.encode(env)).ok().unwrap();
        map = map.map_put(atoms::year().to_term(env), self.year().encode(env)).ok().unwrap();
        map = map.map_put(atoms::zone_abbr().to_term(env), "UTC".encode(env)).ok().unwrap();
        map
    }
}

impl<'a> Decoder<'a> for chrono::DateTime<Utc> {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let env = term.get_env();
        let year: i32 = Decoder::decode(term.map_get(atoms::year().to_term(env))?)?;
        let month: u32 = Decoder::decode(term.map_get(atoms::month().to_term(env))?)?;
        let day: u32 = Decoder::decode(term.map_get(atoms::day().to_term(env))?)?;
        let hour: u32 = Decoder::decode(term.map_get(atoms::hour().to_term(env))?)?;
        let minute: u32 = Decoder::decode(term.map_get(atoms::minute().to_term(env))?)?;
        let second: u32 = Decoder::decode(term.map_get(atoms::second().to_term(env))?)?;
        let microsecond: (u32, u32) = Decoder::decode(term.map_get(atoms::microsecond().to_term(env))?)?;
        Ok(Utc
            .ymd(year, month, day)
            .and_hms_micro(hour, minute, second, microsecond.0))
    }
}

mod atoms {
    rustler_atoms! {
        atom calendar_iso = "Elixir.Calendar.ISO";
        atom calendar = "calendar";
        atom day = "day";
        atom hour = "hour";
        atom microsecond = "microsecond";
        atom minute = "minute";
        atom month = "month";
        atom second = "second";
        atom std_offset = "std_offset";
        atom time_zone = "time_zone";
        atom utc_offset = "utc_offset";
        atom year = "year";
        atom zone_abbr = "zone_abbr";
    }
}
