pub use clap::Parser;
use clap::ValueEnum;
use std::{ffi::OsString, fmt::Display};

use crate::output::time::TimeFormat;

#[derive(Parser)]
#[command(author, version, about, long_about)] // Read from `Cargo.toml`
#[clap(disable_help_flag = true)]
pub struct Opts {
    pub paths: Vec<OsString>,
    /// Show hidden files.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub all: u8,
    /// display extended file metadata as a table.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub long: u8,
    /// list each file's Git status, if tracked or ignored.
    #[arg(long, action = clap::ArgAction::Count)]
    pub git: u8,
    /// Display one entry per line.
    #[arg(short = '1', long, action = clap::ArgAction::Count)]
    pub oneline: u8,
    ///recurse into directories as a tree.
    #[arg(short = 'T', long, action = clap::ArgAction::Count)]
    pub tree: u8,
    /// display entries as a grid (default).
    #[arg(short = 'G', long, action = clap::ArgAction::Count)]
    pub grid: u8,
    /// sort the grid across, rather than downwards.
    #[arg(short = 'x', long, action = clap::ArgAction::Count)]
    pub across: u8,
    /// recurse into directories.
    #[arg(short = 'R', long, action = clap::ArgAction::Count)]
    pub recurse: u8,
    /// display type indicator by file names.
    #[arg(short = 'F', long, action = clap::ArgAction::Count)]
    pub classify: u8,
    #[arg(short = 'X', long, action = clap::ArgAction::Count)]
    pub dereference: u8,
    /// set screen width in columns.
    #[arg(short = 'w', long)]
    pub width: Option<usize>,
    /// when to use terminal colours (always, auto, never).
    #[arg(long, alias = "colour", value_enum, default_value = ShowWhen::Auto, default_missing_value = ShowWhen::Auto, require_equals = false, num_args=0..=1)]
    pub color: ShowWhen,
    /// highlight levels of 'field' distinctly(all, age, size).
    #[arg(long, alias = "colour-scale", value_enum, default_value = None, default_missing_value = None, num_args = 0..=1, require_equals = false)]
    pub color_scale: Option<ColorScaleArgs>,
    /// use gradient or fixed colors in --color-scale (fixed, gradient)
    #[arg(long, alias = "colour-scale-mode", value_enum, default_value_t = ColorScaleModeArgs::Gradient, default_missing_value = "gradient", num_args = 0..=1, require_equals = false)]
    pub color_scale_mode: ColorScaleModeArgs,
    #[arg(short = 'A', long, action = clap::ArgAction::Count)]
    pub almost_all: u8,
    /// list directories as files; don't list their contents.
    #[arg(short = 'd', long, action = clap::ArgAction::Count)]
    pub list_dirs: u8,
    /// limit the depth of recursion.
    #[arg(short = 'L', long)]
    pub level: Option<usize>,
    /// reverse the sort order.
    #[arg(short = 'r', long, action = clap::ArgAction::Count)]
    pub reverse: u8,
    /// which field to sort by.
    #[arg(short = 's', long, num_args = 0..=1, require_equals = false)]
    pub sort: Option<OsString>, // ValueEnum here means we lose the sort field deducing :/
    /// glob patterns (pipe-separated) of files to ignore.
    #[arg(short = 'I', long)]
    pub ignore_glob: Option<OsString>,
    /// ignore files mentioned in '.gitignore'.
    #[arg(long = "git-ignore", action = clap::ArgAction::Count)]
    pub git_ignore: u8,
    /// list directories before other files.
    #[arg(long = "group-directories-first", action = clap::ArgAction::Count)]
    pub dirs_first: u8,
    /// list only directories.
    #[arg(short = 'D', long = "only-dirs", action = clap::ArgAction::Count)]
    pub only_dirs: u8,
    /// list file sizes with binary prefixes.
    #[arg(short = 'b', long, action = clap::ArgAction::Count)]
    pub binary: u8,
    /// list file sizes in bytes, without any prefixes.
    #[arg(short = 'B', long, action = clap::ArgAction::Count)]
    pub bytes: u8,
    /// list each file's group.
    #[arg(short = 'g', long, action = clap::ArgAction::Count)]
    pub group: u8,
    /// list numeric user and group IDs.
    #[arg(short = 'n', long, action = clap::ArgAction::Count)]
    pub numeric: u8,
    /// add a header row to each column.
    #[arg(short = 'h', long, action = clap::ArgAction::Count)]
    pub header: u8,
    /// display icons
    #[arg(long, default_value = None, default_missing_value = ShowWhen::Auto, num_args = 0..=1, require_equals = false)]
    pub icons: Option<ShowWhen>,
    /// list each file's inode number.
    #[arg(short = 'i', long, action = clap::ArgAction::Count)]
    pub inode: u8,
    /// list each file's number of hard links.
    #[arg(short = 'H', long, action = clap::ArgAction::Count)]
    pub links: u8,
    /// use the modified timestamp field.
    #[arg(short = 'm', long, action = clap::ArgAction::Count)]
    pub modified: u8,
    /// use the changed timestamp field.
    #[arg(long, action = clap::ArgAction::Count)]
    pub changed: u8,
    /// show size of allocated file system blocks.
    #[arg(short = 'S', long, action = clap::ArgAction::Count)]
    pub blocksize: u8,
    /// which timestamp field to list (modified, accessed, created).
    #[arg(short = 't', long)]
    pub time: Option<OsString>,
    /// use the accessed timestamp field.
    #[arg(short = 'u', long, action = clap::ArgAction::Count)]
    pub accessed: u8,
    /// use the created timestamp field.
    #[arg(short = 'U', long, action = clap::ArgAction::Count)]
    pub created: u8,
    /// how to format timestamps (default, iso, long-iso, full-iso, relative).
    #[arg(long = "time-style", value_enum, default_value = TimeFormat::DefaultFormat, default_missing_value = "default", num_args = 0..=1, require_equals = false)]
    pub time_style: Option<TimeFormat>,
    /// display entries as hyperlinks.
    #[arg(long, action = clap::ArgAction::Count)]
    pub hyperlink: u8,
    /// suppress the permissions field.
    #[arg(long = "no-permissions", action = clap::ArgAction::Count)]
    pub no_permissions: u8,
    /// suppress the filesize field.
    #[arg(long = "no-filesize", action = clap::ArgAction::Count)]
    pub no_filesize: u8,
    /// suppress the user field.
    #[arg(long = "no-user", action = clap::ArgAction::Count)]
    pub no_user: u8,
    /// suppress the time field.
    #[arg(long = "no-time", action = clap::ArgAction::Count)]
    pub no_time: u8,
    /// don't display icons (always override --icons).
    #[arg(long = "no-icons", action = clap::ArgAction::Count)]
    pub no_icons: u8,
    /// suppress git.
    #[arg(long = "no-git", action = clap::ArgAction::Count)]
    pub no_git: u8,
    /// list root of git-tree status.
    #[arg(long = "git-repos", action = clap::ArgAction::Count)]
    pub git_repos: u8,
    ///List each git-repos branch name (much faster)
    #[arg(long = "git-repos-no-status", action = clap::ArgAction::Count)]
    pub git_repos_no_status: u8,
    /// list each file's permission in octal format.
    #[arg(short = 'o', long, alias = "octal-permission", alias = "octal-permissions", action = clap::ArgAction::Count)]
    pub octal: u8,
    /// Display the number of hard links to file.
    #[arg(short = 'Z', long = "context", action = clap::ArgAction::Count)]
    pub security_context: u8,
    /// Show extended attributes.
    #[arg(short = '@', long, action = clap::ArgAction::Count)]
    pub extended: u8,
    /// Show list of command-line options.
    #[arg(short ='?', long, action = clap::ArgAction::Help)]
    pub help: Option<bool>,
    /// Show mount details (Linux only)
    #[arg(short = 'M', long, action = clap::ArgAction::Count)]
    pub mounts: u8,
    /// Show only files
    #[arg(short = 'f', long = "only-files", action = clap::ArgAction::Count)]
    pub only_files: u8,
    /// Don't Show quotes
    #[arg(long = "no-quotes", action = clap::ArgAction::Count)]
    pub no_quotes: u8,
    /// only show group if it has a different name from owner
    #[arg(long = "smart-group", action = clap::ArgAction::Count)]
    pub smart_group: u8,
    /// show the size of a directory as the size of all files and directories inside
    #[arg(long = "total-size", action = clap::ArgAction::Count)]
    pub total_size: u8,
    /// use stdin as the sole input
    #[arg(long = "stdin", action = clap::ArgAction::Count)]
    pub stdin: u8,

    #[arg(short = 'O', long = "flags", action = clap::ArgAction::Count)]
    pub file_flags: u8,

    #[arg(long = "no-symlinks", action = clap::ArgAction::Count)]
    pub no_symlinks: u8,

    #[arg(long = "show-symlinks", action = clap::ArgAction::Count)]
    pub show_symlinks: u8,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ShowWhen {
    // icons, colors, quotes, headers ? eventually
    Always,
    Auto,
    Never,
}

impl Display for ShowWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShowWhen::Always => write!(f, "always"),
            ShowWhen::Auto => write!(f, "auto"),
            ShowWhen::Never => write!(f, "never"),
        }
    }
}

impl From<ShowWhen> for clap::builder::OsStr {
    fn from(sw: ShowWhen) -> clap::builder::OsStr {
        match sw {
            ShowWhen::Always => clap::builder::OsStr::from("always"),
            ShowWhen::Auto => clap::builder::OsStr::from("auto"),
            ShowWhen::Never => clap::builder::OsStr::from("never"),
        }
    }
}

impl From<clap::builder::OsStr> for ShowWhen {
    fn from(s: clap::builder::OsStr) -> ShowWhen {
        match s.to_str() {
            Some("always") => ShowWhen::Always,
            Some("auto") => ShowWhen::Auto,
            Some("never") => ShowWhen::Never,
            _ => ShowWhen::Auto,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColorScaleArgs {
    All,
    Age,
    Size,
}

impl ValueEnum for ColorScaleArgs {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ColorScaleArgs::All,
            ColorScaleArgs::Age,
            ColorScaleArgs::Size,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            ColorScaleArgs::All => Some(clap::builder::PossibleValue::new("all")),
            ColorScaleArgs::Age => Some(clap::builder::PossibleValue::new("age")),
            ColorScaleArgs::Size => Some(clap::builder::PossibleValue::new("size")),
        }
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        if ignore_case {
            match s.to_ascii_lowercase().as_str() {
                "all" | "age,size" => Ok(ColorScaleArgs::All),
                "age" => Ok(ColorScaleArgs::Age),
                "size" => Ok(ColorScaleArgs::Size),
                _ => Err(format!("Unknown color-scale value: {s}")),
            }
        } else {
            match s {
                "all" | "age,size" => Ok(ColorScaleArgs::All),
                "age" => Ok(ColorScaleArgs::Age),
                "size" => Ok(ColorScaleArgs::Size),
                _ => Err(format!("Unknown color-scale value: {s}")),
            }
        }
    }
}

impl Display for ColorScaleArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorScaleArgs::All => write!(f, "all"),
            ColorScaleArgs::Age => write!(f, "age"),
            ColorScaleArgs::Size => write!(f, "size"),
        }
    }
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ColorScaleModeArgs {
    Fixed,
    Gradient,
}
impl Display for ColorScaleModeArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorScaleModeArgs::Fixed => write!(f, "fixed"),
            ColorScaleModeArgs::Gradient => write!(f, "gradient"),
        }
    }
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum SortArgs {
    Name,
    Size,
    Time,
    Extension,
    Inode,
    Version,
    Created,
    Accessed,
    Modified,
    Changed,
}

impl From<clap::builder::OsStr> for SortArgs {
    fn from(value: clap::builder::OsStr) -> Self {
        match value.to_ascii_lowercase().to_str() {
            Some("name") => SortArgs::Name,
            Some("size") => SortArgs::Size,
            Some("time" | "age" | "date" | "") => SortArgs::Time,
            Some("extension") => SortArgs::Extension,
            Some("inode") => SortArgs::Inode,
            Some("version") => SortArgs::Version,
            Some("created") => SortArgs::Created,
            Some("accessed") => SortArgs::Accessed,
            Some("modified") => SortArgs::Modified,
            Some("changed") => SortArgs::Changed,
            _ => SortArgs::Name,
        }
    }
}

impl SortArgs {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortArgs::Name => "name",
            SortArgs::Size => "size",
            SortArgs::Time => "time",
            SortArgs::Extension => "extension",
            SortArgs::Inode => "inode",
            SortArgs::Version => "version",
            SortArgs::Created => "created",
            SortArgs::Accessed => "accessed",
            SortArgs::Modified => "modified",
            SortArgs::Changed => "changed",
        }
    }
}

impl From<SortArgs> for clap::builder::OsStr {
    fn from(value: SortArgs) -> Self {
        match value {
            SortArgs::Name => clap::builder::OsStr::from("name"),
            SortArgs::Size => clap::builder::OsStr::from("size"),
            SortArgs::Time => clap::builder::OsStr::from("time"),
            SortArgs::Extension => clap::builder::OsStr::from("extension"),
            SortArgs::Inode => clap::builder::OsStr::from("inode"),
            SortArgs::Version => clap::builder::OsStr::from("version"),
            SortArgs::Created => clap::builder::OsStr::from("created"),
            SortArgs::Accessed => clap::builder::OsStr::from("accessed"),
            SortArgs::Modified => clap::builder::OsStr::from("modified"),
            SortArgs::Changed => clap::builder::OsStr::from("changed"),
        }
    }
}
impl Display for SortArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortArgs::Name => write!(f, "name"),
            SortArgs::Size => write!(f, "size"),
            SortArgs::Time => write!(f, "time"),
            SortArgs::Extension => write!(f, "extension"),
            SortArgs::Inode => write!(f, "inode"),
            SortArgs::Version => write!(f, "version"),
            SortArgs::Created => write!(f, "created"),
            SortArgs::Accessed => write!(f, "accessed"),
            SortArgs::Modified => write!(f, "modified"),
            SortArgs::Changed => write!(f, "changed"),
        }
    }
}
#[derive(Clone, Debug, ValueEnum)]
pub enum TimeArgs {
    Modified,
    Accessed,
    Created,
}
impl Display for TimeArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeArgs::Modified => write!(f, "modified"),
            TimeArgs::Accessed => write!(f, "accessed"),
            TimeArgs::Created => write!(f, "created"),
        }
    }
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            paths: vec![],
            all: 0,
            long: 0,
            git: 0,
            oneline: 0,
            recurse: 0,
            list_dirs: 0,
            tree: 0,
            level: None,
            reverse: 0,
            sort: None,
            ignore_glob: None,
            git_ignore: 0,
            dirs_first: 0,
            only_dirs: 0,
            binary: 0,
            bytes: 0,
            group: 0,
            numeric: 0,
            grid: 0,
            across: 0,
            classify: 0,
            dereference: 0,
            width: None,
            color: ShowWhen::Auto,
            color_scale: None,
            color_scale_mode: ColorScaleModeArgs::Gradient,
            almost_all: 0,
            header: 0,
            icons: None,
            inode: 0,
            git_repos: 0,
            git_repos_no_status: 0,
            links: 0,
            modified: 0,
            created: 0,
            accessed: 0,
            changed: 0,
            blocksize: 0,
            time: None,
            time_style: None,
            no_filesize: 0,
            no_icons: 0,
            no_permissions: 0,
            no_time: 0,
            no_user: 0,
            extended: 0,
            hyperlink: 0,
            octal: 0,
            security_context: 0,
            help: Some(false),
            no_git: 0,
            mounts: 0,
            only_files: 0,
            no_quotes: 0,
            smart_group: 0,
            total_size: 0,
            stdin: 0,
            file_flags: 0,
            no_symlinks: 0,
            show_symlinks: 0,
        }
    }
}
