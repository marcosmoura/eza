// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::options::parser::{Arg, Args, TakesValue, Values};

// exa options
pub static VERSION: Arg = Arg { short: Some(b'v'), long: "version",  takes_value: TakesValue::Forbidden };
pub static HELP:    Arg = Arg { short: Some(b'?'), long: "help",     takes_value: TakesValue::Forbidden };

// display options
pub static ONE_LINE:     Arg = Arg { short: Some(b'1'), long: "oneline",         takes_value: TakesValue::Forbidden };
pub static LONG:         Arg = Arg { short: Some(b'l'), long: "long",            takes_value: TakesValue::Forbidden };
pub static GRID:         Arg = Arg { short: Some(b'G'), long: "grid",            takes_value: TakesValue::Forbidden };
pub static ACROSS:       Arg = Arg { short: Some(b'x'), long: "across",          takes_value: TakesValue::Forbidden };
pub static RECURSE:      Arg = Arg { short: Some(b'R'), long: "recurse",         takes_value: TakesValue::Forbidden };
pub static TREE:         Arg = Arg { short: Some(b'T'), long: "tree",            takes_value: TakesValue::Forbidden };
pub static CLASSIFY:     Arg = Arg { short: Some(b'F'), long: "classify",        takes_value: TakesValue::Optional(Some(WHEN), "auto") };
pub static DEREF_LINKS:  Arg = Arg { short: Some(b'X'), long: "dereference",     takes_value: TakesValue::Forbidden };
pub static WIDTH:        Arg = Arg { short: Some(b'w'), long: "width",           takes_value: TakesValue::Necessary(None) };
pub static SPACING:Arg = Arg { short: None,       long: "spacing", takes_value: TakesValue::Necessary(None) };
pub static NO_QUOTES:    Arg = Arg { short: None,       long: "no-quotes",       takes_value: TakesValue::Forbidden };
pub static ABSOLUTE:     Arg = Arg { short: None,       long: "absolute",        takes_value: TakesValue::Optional(Some(ABSOLUTE_MODES), "on") };
pub static FOLLOW_LINKS: Arg = Arg { short: None,       long: "follow-symlinks", takes_value: TakesValue::Forbidden };
const ABSOLUTE_MODES: &[&str] = &["on", "follow", "off"];

pub static COLOR:  Arg = Arg { short: None, long: "color",  takes_value: TakesValue::Optional(Some(WHEN), "auto") };
pub static COLOUR: Arg = Arg { short: None, long: "colour", takes_value: TakesValue::Optional(Some(WHEN), "auto") };
const WHEN: &[&str] = &["always", "auto", "never"];

pub static COLOR_SCALE:  Arg = Arg { short: None, long: "color-scale",  takes_value: TakesValue::Optional(Some(SCALES), "all") };
pub static COLOUR_SCALE: Arg = Arg { short: None, long: "colour-scale", takes_value: TakesValue::Optional(Some(SCALES), "all") };
pub static COLOR_SCALE_MODE:  Arg = Arg { short: None, long: "color-scale-mode",  takes_value: TakesValue::Necessary(Some(COLOR_SCALE_MODES))};
pub static COLOUR_SCALE_MODE: Arg = Arg { short: None, long: "colour-scale-mode", takes_value: TakesValue::Necessary(Some(COLOR_SCALE_MODES))};
const SCALES: Values = &["all", "size", "age"];
const COLOR_SCALE_MODES: Values = &["fixed", "gradient"];

// filtering and sorting options
pub static ALL:                 Arg = Arg { short: Some(b'a'), long: "all",         takes_value: TakesValue::Forbidden };
pub static ALMOST_ALL:          Arg = Arg { short: Some(b'A'), long: "almost-all",  takes_value: TakesValue::Forbidden };
pub static TREAT_DIRS_AS_FILES: Arg = Arg { short: Some(b'd'), long: "treat-dirs-as-files",   takes_value: TakesValue::Forbidden };
pub static LIST_DIRS:           Arg = Arg { short: None, long: "list-dirs",   takes_value: TakesValue::Forbidden };
pub static LEVEL:               Arg = Arg { short: Some(b'L'), long: "level",       takes_value: TakesValue::Necessary(None) };
pub static REVERSE:             Arg = Arg { short: Some(b'r'), long: "reverse",     takes_value: TakesValue::Forbidden };
pub static SORT:                Arg = Arg { short: Some(b's'), long: "sort",        takes_value: TakesValue::Necessary(Some(SORTS)) };
pub static IGNORE_GLOB:         Arg = Arg { short: Some(b'I'), long: "ignore-glob", takes_value: TakesValue::Necessary(None) };
pub static GIT_IGNORE:          Arg = Arg { short: None, long: "git-ignore",           takes_value: TakesValue::Forbidden };
pub static DIRS_FIRST:          Arg = Arg { short: None, long: "group-directories-first",  takes_value: TakesValue::Forbidden };
pub static DIRS_LAST:           Arg = Arg { short: None, long: "group-directories-last",  takes_value: TakesValue::Forbidden };
pub static ONLY_DIRS:           Arg = Arg { short: Some(b'D'), long: "only-dirs", takes_value: TakesValue::Forbidden };
pub static ONLY_FILES:          Arg = Arg { short: Some(b'f'), long: "only-files", takes_value: TakesValue::Forbidden };
pub static NO_SYMLINKS:         Arg = Arg { short: None,       long: "no-symlinks", takes_value: TakesValue::Forbidden };
pub static SHOW_SYMLINKS:       Arg = Arg { short: None,     long: "show-symlinks", takes_value: TakesValue::Forbidden };

const SORTS: Values = &[ "name", "Name", "size", "extension",
                         "Extension", "modified", "changed", "accessed",
                         "created", "inode", "type", "none" ];

// display options
pub static BINARY:      Arg = Arg { short: Some(b'b'), long: "binary",      takes_value: TakesValue::Forbidden };
pub static BYTES:       Arg = Arg { short: Some(b'B'), long: "bytes",       takes_value: TakesValue::Forbidden };
pub static GROUP:       Arg = Arg { short: Some(b'g'), long: "group",       takes_value: TakesValue::Forbidden };
pub static NUMERIC:     Arg = Arg { short: Some(b'n'), long: "numeric",     takes_value: TakesValue::Forbidden };
pub static HEADER:      Arg = Arg { short: Some(b'h'), long: "header",      takes_value: TakesValue::Forbidden };
pub static ICONS:       Arg = Arg { short: None,       long: "icons",       takes_value: TakesValue::Optional(Some(WHEN), "auto")};
pub static INODE:       Arg = Arg { short: Some(b'i'), long: "inode",       takes_value: TakesValue::Forbidden };
pub static LINKS:       Arg = Arg { short: Some(b'H'), long: "links",       takes_value: TakesValue::Forbidden };
pub static MODIFIED:    Arg = Arg { short: Some(b'm'), long: "modified",    takes_value: TakesValue::Forbidden };
pub static CHANGED:     Arg = Arg { short: None,       long: "changed",     takes_value: TakesValue::Forbidden };
pub static BLOCKSIZE:   Arg = Arg { short: Some(b'S'), long: "blocksize",   takes_value: TakesValue::Forbidden };
pub static TOTAL_SIZE:  Arg = Arg { short: None,       long: "total-size",  takes_value: TakesValue::Forbidden };
pub static TIME:        Arg = Arg { short: Some(b't'), long: "time",        takes_value: TakesValue::Necessary(Some(TIMES)) };
pub static ACCESSED:    Arg = Arg { short: Some(b'u'), long: "accessed",    takes_value: TakesValue::Forbidden };
pub static CREATED:     Arg = Arg { short: Some(b'U'), long: "created",     takes_value: TakesValue::Forbidden };
pub static TIME_STYLE:  Arg = Arg { short: None,       long: "time-style",  takes_value: TakesValue::Necessary(Some(TIME_STYLES)) };
pub static HYPERLINK:   Arg = Arg { short: None,       long: "hyperlink",   takes_value: TakesValue::Forbidden };
pub static MOUNTS:      Arg = Arg { short: Some(b'M'), long: "mounts",      takes_value: TakesValue::Forbidden };
pub static SMART_GROUP: Arg = Arg { short: None,       long: "smart-group", takes_value: TakesValue::Forbidden };
const TIMES: Values = &["modified", "changed", "accessed", "created"];
const TIME_STYLES: Values = &["default", "long-iso", "full-iso", "iso", "relative"];

// suppressing columns
pub static NO_PERMISSIONS: Arg = Arg { short: None, long: "no-permissions", takes_value: TakesValue::Forbidden };
pub static NO_FILESIZE: Arg = Arg { short: None, long: "no-filesize", takes_value: TakesValue::Forbidden };
pub static NO_USER: Arg = Arg { short: None, long: "no-user", takes_value: TakesValue::Forbidden };
pub static NO_TIME: Arg = Arg { short: None, long: "no-time", takes_value: TakesValue::Forbidden };

// optional feature options
pub static GIT:               Arg = Arg { short: None,       long: "git",                  takes_value: TakesValue::Forbidden };
pub static NO_GIT:            Arg = Arg { short: None,       long: "no-git",               takes_value: TakesValue::Forbidden };
pub static GIT_REPOS:         Arg = Arg { short: None,       long: "git-repos",            takes_value: TakesValue::Forbidden };
pub static GIT_REPOS_NO_STAT: Arg = Arg { short: None,       long: "git-repos-no-status",  takes_value: TakesValue::Forbidden };
pub static EXTENDED:          Arg = Arg { short: Some(b'@'), long: "extended",             takes_value: TakesValue::Forbidden };
pub static OCTAL:             Arg = Arg { short: Some(b'o'), long: "octal-permissions",    takes_value: TakesValue::Forbidden };
pub static SECURITY_CONTEXT:  Arg = Arg { short: Some(b'Z'), long: "context",              takes_value: TakesValue::Forbidden };
pub static STDIN:             Arg = Arg { short: None,       long: "stdin",                takes_value: TakesValue::Forbidden };
pub static FILE_FLAGS:        Arg = Arg { short: Some(b'O'), long: "flags",                takes_value: TakesValue::Forbidden };

pub static ALL_ARGS: Args = Args(&[
    &VERSION, &HELP,

    &ONE_LINE, &LONG, &GRID, &ACROSS, &RECURSE, &TREE, &CLASSIFY, &DEREF_LINKS, &FOLLOW_LINKS,
    &COLOR, &COLOUR, &COLOR_SCALE, &COLOUR_SCALE, &COLOR_SCALE_MODE, &COLOUR_SCALE_MODE,
    &WIDTH, &NO_QUOTES, &ABSOLUTE, &SPACING,

    &ALL, &ALMOST_ALL, &TREAT_DIRS_AS_FILES, &LIST_DIRS, &LEVEL, &REVERSE, &SORT, &DIRS_FIRST, &DIRS_LAST,
    &IGNORE_GLOB, &GIT_IGNORE, &ONLY_DIRS, &ONLY_FILES,

    &BINARY, &BYTES, &GROUP, &NUMERIC, &HEADER, &ICONS, &INODE, &LINKS, &MODIFIED, &CHANGED,
    &BLOCKSIZE, &TOTAL_SIZE, &TIME, &ACCESSED, &CREATED, &TIME_STYLE, &HYPERLINK, &MOUNTS,
    &NO_PERMISSIONS, &NO_FILESIZE, &NO_USER, &NO_TIME, &SMART_GROUP, &NO_SYMLINKS, &SHOW_SYMLINKS,

    &GIT, &NO_GIT, &GIT_REPOS, &GIT_REPOS_NO_STAT,
    &EXTENDED, &OCTAL, &SECURITY_CONTEXT, &STDIN, &FILE_FLAGS
]);
