use ::colored::Colorize;
use ::std::ffi::OsStr;
use ::std::fmt;
use ::std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Language {
    Assembly,
    Astro,
    Bash,
    C,
    Carbon,
    Catrina,
    Clojure,
    CMake,
    Co,
    Cobol,
    Css,
    Cxx,
    CoffeeScript,
    Cognate,
    Crystal,
    CSharp,
    D,
    Dart,
    Elm,
    Elixir,
    Erlang,
    Fortran,
    FSharp,
    Gleam,
    Gn,
    Go,
    Grain,
    GraphQl,
    Gren,
    Hare,
    Haskell,
    Html,
    Idris,
    Jai,
    Jakt,
    Java,
    JavaScript,
    Json,
    Julia,
    Koka,
    Kotlin,
    Llvm,
    Lua,
    Make,
    Markdown,
    Nim,
    NuShell,
    Oak,
    ObjectiveC,
    ObjectiveCxx,
    OCaml,
    Odin,
    Pascal,
    Php,
    Perl,
    Porth,
    PowerShell,
    Prolog,
    Python,
    Terraform,
    Turquoise,
    Racket,
    Ren,
    Ruby,
    Roc,
    Rust,
    Sass,
    Scala,
    Sql,
    Svelte,
    Swift,
    Tcl,
    Toml,
    TypeScript,
    Unison,
    V,
    Vale,
    VisualBasic,
    Vue,
    WebAssembly,
    Xml,
    Yaml,
    Zig,
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = LanguageInfo::from(self);

        let name = f
            .width()
            .map(|width| format!("{:width$}", &info.name, width = width))
            .unwrap_or(info.name.clone());

        write!(
            f,
            "{}",
            info.color
                .map(|color| color.color(&*name))
                .unwrap_or(name.to_string())
        )
    }
}

#[derive(Clone, Debug)]
pub struct LanguageInfo {
    pub name: String,
    pub color: Option<Color>,
}

macro_rules! info {
    ( $n:expr $(,)? ) => {{
        LanguageInfo {
            name: $n.into(),
            color: None,
        }
    }};

    ( $n:expr , color: $c:expr $(,)? ) => {{
        LanguageInfo {
            name: $n.into(),
            color: Some($c.into()),
        }
    }};
}

impl Language {
    pub fn from_file_name(file_name: &OsStr) -> Option<Self> {
        use Language::*;

        match file_name.to_str()? {
            "CMakeLists.txt" => Some(CMake),
            "Makefile" => Some(Make),
            _ => None,
        }
    }

    pub fn from_extension(ext: &OsStr) -> Option<Self> {
        use Language::*;

        match ext.to_str()? {
            "asm" => Some(Assembly),
            "astro" => Some(Astro),
            "c" => Some(C),
            "carbon" => Some(Carbon),
            "cbl" => Some(Cobol),
            "cc" => Some(Cxx),
            "cjs" => Some(JavaScript),
            "clj" => Some(Clojure),
            "co" => Some(Co),
            "coffee" => Some(CoffeeScript),
            "cog" => Some(Cognate),
            "cpp" => Some(Cxx),
            "cr" => Some(Crystal),
            "cs" => Some(CSharp),
            "css" => Some(Css),
            "cts" => Some(TypeScript),
            "cxx" => Some(Cxx),
            "d" => Some(D),
            "dart" => Some(Dart),
            "elm" => Some(Elm),
            "erl" => Some(Erlang),
            "ex" => Some(Elixir),
            "f" => Some(Fortran),
            "for" => Some(Fortran),
            "fs" => Some(FSharp),
            "f90" => Some(Fortran),
            "f95" => Some(Fortran),
            "f03" => Some(Fortran),
            "gleam" => Some(Gleam),
            "gn" => Some(Gn),
            "go" => Some(Go),
            "gql" => Some(GraphQl),
            "gr" => Some(Grain),
            "gren" => Some(Gren),
            "h" => Some(C),
            "ha" => Some(Hare),
            "hh" => Some(Cxx),
            "hpp" => Some(Cxx),
            "hs" => Some(Haskell),
            "htm" => Some(Html),
            "html" => Some(Html),
            "hxx" => Some(Cxx),
            "idr" => Some(Idris),
            "jai" => Some(Jai),
            "jakt" => Some(Jakt),
            "java" => Some(Java),
            "jl" => Some(Julia),
            "js" => Some(JavaScript),
            "json" => Some(Json),
            "jsonc" => Some(Json),
            "jsx" => Some(JavaScript),
            "kk" => Some(Koka),
            "kt" => Some(Kotlin),
            "kts" => Some(Kotlin),
            "ll" => Some(Llvm),
            "lua" => Some(Lua),
            "m" => Some(ObjectiveC),
            "md" => Some(Markdown),
            "mjs" => Some(JavaScript),
            "mk" => Some(Make),
            "ml" => Some(OCaml),
            "mm" => Some(ObjectiveCxx),
            "mts" => Some(TypeScript),
            "nim" => Some(Nim),
            "nu" => Some(NuShell),
            "oak" => Some(Oak),
            "odin" => Some(Odin),
            "pas" => Some(Pascal),
            "php" => Some(Php),
            "pl" => Some(Perl),
            "pm" => Some(Perl),
            "porth" => Some(Porth),
            "pro" => Some(Prolog),
            "ps1" => Some(PowerShell),
            "py" => Some(Python),
            "q" => Some(Turquoise),
            "rb" => Some(Ruby),
            "ren" => Some(Ren),
            "rina" => Some(Catrina),
            "rkt" => Some(Racket),
            "roc" => Some(Roc),
            "rs" => Some(Rust),
            "s" => Some(Assembly),
            "sass" => Some(Sass),
            "scala" => Some(Scala),
            "sh" => Some(Bash),
            "sql" => Some(Sql),
            "svelte" => Some(Svelte),
            "swift" => Some(Swift),
            "tcl" => Some(Tcl),
            "tf" => Some(Terraform),
            "toml" => Some(Toml),
            "ts" => Some(TypeScript),
            "tsx" => Some(TypeScript),
            "u" => Some(Unison),
            "v" => Some(V),
            "vale" => Some(Vale),
            "vb" => Some(VisualBasic),
            "vue" => Some(Vue),
            "wat" => Some(WebAssembly),
            "xml" => Some(Xml),
            "yaml" => Some(Yaml),
            "yml" => Some(Yaml),
            "zig" => Some(Zig),
            _ => None,
        }
    }
}

impl LanguageInfo {
    pub fn from(lang: &Language) -> Self {
        use Language::*;

        match lang {
            Assembly => info!("Assembly"),
            Astro => info!("Astro"),
            Bash => info!("Bash", color: [50, 50, 50]),
            // C => info!("C", color: [163, 176, 240]),
            C => info!("C", color: [40, 48, 126]),
            Carbon => info!("Carbon"),
            Catrina => info!("Catrina", color: [255, 105, 180]),
            Clojure => info!("Clojure", color: [0, 112, 255]),
            CMake => info!("CMake"),
            Co => info!("Co"),
            Cobol => info!("Cobol", color: [0, 112, 255]),
            CoffeeScript => info!("CoffeeScript", color: 0x3e2723),
            Cognate => info!("Cognate"),
            Crystal => info!("Crystal"),
            CSharp => info!("C#", color: [5, 142, 12]),
            Css => info!("CSS"),
            Cxx => info!("C++", color: [25, 65, 122]),
            D => info!("D"),
            Dart => info!("Dart", color: 0x40c4ff),
            Elm => info!("Elm", color: 0x60b5cc),
            Elixir => info!("Elixir", color: 0x4e2a8e),
            Erlang => info!("Erlang", color: 0xa2003e),
            FSharp => info!("F#", color: 0xb845fc),
            Fortran => info!("Fortran"),
            Gleam => info!("Gleam", color: 0xffaff3),
            Gn => info!("gn"),
            Go => info!("Go", color: 0x00add8),
            Grain => info!("Grain", color: [255, 133, 14]),
            GraphQl => info!("GraphQL"),
            Gren => info!("Gren", color: 0xff6600),
            Hare => info!("Hare", color: 0x121415),
            Haskell => info!("Haskell", color: 0x6144b3),
            Html => info!("HTML"),
            Idris => info!("Idris"),
            Jai => info!("Jai"),
            Jakt => info!("Jakt", color: [255, 0, 0]), // TODO: bad
            // Java => info!("Java", color: [205, 112, 42]),
            Java => info!("Java", color: [205, 55, 47]),
            JavaScript => info!("JavaScript", color: 0xf1e05a),
            Json => info!("JSON"),
            Julia => info!("Julia", color: 0xa270ba),
            Koka => info!("Koka"),
            // Kotlin => info!("Kotlin", color: 0x7f52ff),
            Kotlin => info!("Kotlin", color: 0xa97bff),
            Llvm => info!("LLVM IR"),
            Lua => info!("Lua"),
            Make => info!("Make"),
            Markdown => info!("Markdown"),
            Nim => info!("Nim", color: 0xffc200),
            NuShell => info!("NuShell", color: 0x3aa675),
            Oak => info!("Oak"),
            ObjectiveC => info!("Objective-C"),
            ObjectiveCxx => info!("Objective-C++"),
            OCaml => info!("OCaml"),
            Odin => info!("Odin"),
            Pascal => info!("Pascal"),
            Perl => info!("Perl"),
            Php => info!("PHP", color: 0x4f5d95),
            Porth => info!("Porth"),
            PowerShell => info!("PowerShell"),
            Prolog => info!("Prolog"),
            Python => info!("Python", color: 0x3776ab),
            Turquoise => info!("Turquoise", color: 0x90eada),
            Racket => info!("Racket"),
            Ren => info!("Ren", color: 0xdd5e36),
            Ruby => info!("Ruby", color: 0xcc342d),
            Roc => info!("Roc"),
            Rust => info!("Rust", color: 0xa72145),
            Sass => info!("Sass"),
            Scala => info!("Scala"),
            Sql => info!("SQL", color: 0x336790),
            Svelte => info!("Svelte"),
            Swift => info!("Swift", color: 0xf05138),
            Tcl => info!("Tcl"),
            Terraform => info!("Terraform", color: 0x844fba),
            Toml => info!("TOML"),
            TypeScript => info!("TypeScript", color: 0x3178c6),
            Unison => info!("Unison", color: [118, 207, 143]),
            V => info!("V"),
            Vale => info!("Vale"),
            VisualBasic => info!("Visual Basic"),
            Vue => info!("Vue", color: 0x41b883),
            WebAssembly => info!("WebAssembly"),
            Xml => info!("XML"),
            Yaml => info!("YAML"),
            Zig => info!("Zig", color: [235, 168, 66]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LanguageSummary {
    pub language: Language,
    pub lines: usize,
}

impl Display for LanguageSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:89}  {:>9}", self.language, self.lines)
    }
}

impl LanguageSummary {
    pub fn from(language: Language) -> Self {
        Self { language, lines: 0 }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn color<T>(&self, t: T) -> String
    where
        T: Colorize,
    {
        t.truecolor(self.r, self.g, self.b).to_string()
    }

    pub fn on_color<T>(&self, t: T) -> String
    where
        T: Colorize,
    {
        t.on_truecolor(self.r, self.g, self.b).to_string()
    }
}

impl From<[u8; 3]> for Color {
    fn from(color: [u8; 3]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
        }
    }
}

impl From<usize> for Color {
    fn from(color: usize) -> Self {
        Self {
            r: ((color >> 16) & 0xff) as u8,
            g: ((color >> 8) & 0xff) as u8,
            b: (color & 0xff) as u8,
        }
    }
}
