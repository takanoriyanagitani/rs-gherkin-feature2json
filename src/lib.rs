use std::io;

use io::BufWriter;
use io::Write;

use io::Read;

use gherkin::Feature;
use gherkin::GherkinEnv;

pub fn str2feature<P>(s: P, env: GherkinEnv) -> Result<Feature, io::Error>
where
    P: AsRef<str>,
{
    Feature::parse(s, env).map_err(io::Error::other)
}

pub fn str2feature_default<P>(s: P) -> Result<Feature, io::Error>
where
    P: AsRef<str>,
{
    str2feature(s, GherkinEnv::default())
}

pub fn string2feature_default(s: String) -> Result<Feature, io::Error> {
    str2feature_default(s)
}

pub fn feature2json2wtr<W>(f: &Feature, mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    serde_json::to_writer(&mut wtr, f)?;
    wtr.flush()
}

pub fn feature2json(f: Feature) -> Result<Vec<u8>, io::Error> {
    let mut buf: Vec<u8> = vec![];
    feature2json2wtr(&f, &mut buf)?;
    Ok(buf)
}

pub fn compose<F, G, T, U, V, E>(f: F, g: G) -> impl Fn(T) -> Result<V, E>
where
    F: Fn(T) -> Result<U, E>,
    G: Fn(U) -> Result<V, E>,
{
    move |t: T| {
        let u: U = f(t)?;
        g(u)
    }
}

pub fn string2feature2json_default() -> impl Fn(String) -> Result<Vec<u8>, io::Error> {
    compose(string2feature_default, feature2json)
}

pub fn bind<I, P, T, U, E>(mut iofn: I, pure: P) -> impl FnMut() -> Result<U, E>
where
    I: FnMut() -> Result<T, E>,
    P: Fn(T) -> Result<U, E>,
{
    move || {
        let t: T = iofn()?;
        pure(t)
    }
}

pub const FEATURE_FILE_SIZE_LIMIT_DEFAULT: u64 = 1048576;

#[derive(Debug, Clone, Copy)]
pub struct FeatureFileSizeLimit(pub u64);

impl Default for FeatureFileSizeLimit {
    fn default() -> Self {
        Self(FEATURE_FILE_SIZE_LIMIT_DEFAULT)
    }
}

impl FeatureFileSizeLimit {
    pub fn rdr2string_source<R>(self, mut rdr: R) -> impl FnMut() -> Result<String, io::Error>
    where
        R: Read,
    {
        move || {
            let mr = &mut rdr;
            let mut taken = mr.take(self.0);
            let mut buf: String = String::default();
            taken.read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

impl FeatureFileSizeLimit {
    pub fn rdr2fjson_source<R>(self, rdr: R) -> impl FnMut() -> Result<Vec<u8>, io::Error>
    where
        R: Read,
    {
        bind(self.rdr2string_source(rdr), string2feature2json_default())
    }
}

impl FeatureFileSizeLimit {
    pub fn stdin2fjson_source(self) -> impl FnMut() -> Result<Vec<u8>, io::Error> {
        self.rdr2fjson_source(io::stdin().lock())
    }
}

pub fn wtr2jsink<W>(mut wtr: W) -> impl FnMut(Vec<u8>) -> Result<(), io::Error>
where
    W: Write,
{
    move |fjson: Vec<u8>| {
        wtr.write_all(&fjson)?;
        wtr.flush()
    }
}

impl FeatureFileSizeLimit {
    pub fn stdin2feature2json2stdout(self) -> impl FnMut() -> Result<(), io::Error> {
        move || {
            let o = io::stdout();
            let mut ol = o.lock();

            let mut fsrc = self.stdin2fjson_source();
            let mut sink = wtr2jsink(BufWriter::new(&mut ol));

            let fjson: Vec<u8> = fsrc()?;
            sink(fjson)?;

            drop(sink);

            ol.flush()
        }
    }
}

pub fn stdin2feature2json2stdout_default() -> impl FnMut() -> Result<(), io::Error> {
    FeatureFileSizeLimit::default().stdin2feature2json2stdout()
}
