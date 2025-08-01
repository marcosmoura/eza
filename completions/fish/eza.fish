# Meta-stuff
complete -c eza -s v -l version -d "Show version of eza"
complete -c eza -l help -d "Show list of command-line options"

# Display options
complete -c eza -s 1 -l oneline -d "Display one entry per line"
complete -c eza -s l -l long -d "Display extended file metadata as a table"
complete -c eza -s G -l grid -d "Display entries in a grid"
complete -c eza -s x -l across -d "Sort the grid across, rather than downwards"
complete -c eza -s R -l recurse -d "Recurse into directories"
complete -c eza -s T -l tree -d "Recurse into directories as a tree"
complete -c eza -s X -l dereference -d "Dereference symbolic links when displaying file information"
complete -c eza -s F -l classify -d "Display type indicator by file names"
complete -c eza -l color \
    -l colour -d "When to use terminal colours" -x -a "
    always\t'Always use colour'
    auto\t'Use colour if standard output is a terminal'
    automatic\t'Use colour if standard output is a terminal'
    never\t'Never use colour'
"
complete -c eza -l color-scale \
    -l colour-scale -d "Highlight levels 'field' distinctly" -x -a "
    all\t''
    age\t''
    size\t''
"
complete -c eza -l color-scale-mode \
    -l colour-scale-mode \
    -d "Use gradient or fixed colors in --color-scale" -x -a "
    fixed\t'Highlight based on fixed colors'
    gradient\t'Highlight based \'field\' in relation to other files'
"
complete -c eza -l icons -d "When to display icons" -x -a "
  always\t'Always display icons'
  auto\t'Display icons if standard output is a terminal'
  automatic\t'Display icons if standard output is a terminal'
  never\t'Never display icons'
"
complete -c eza -l no-quotes -d "Don't quote file names with spaces"
complete -c eza -l hyperlink -d "Display entries as hyperlinks"
complete -c eza -l follow-symlinks -d "Drill down into symbolic links that point to directories"
complete -c eza -l absolute -d "Display entries with their absolute path" -x -a "
  on\t'Show absolute path for listed entries'
  follow\t'Show absolute path with followed symlinks'
  off\t'Do not show the absolute path'
"
complete -c eza -l smart-group -d "Only show group if it has a different name from owner"

# Filtering and sorting options
complete -c eza -l group-directories-first -d "Sort directories before other files"
complete -c eza -l group-directories-last -d "Sort directories after other files"
complete -c eza -l git-ignore -d "Ignore files mentioned in '.gitignore'"
complete -c eza -s a -l all -d "Show hidden and 'dot' files. Use this twice to also show the '.' and '..' directories"
complete -c eza -s A -l almost-all -d "Equivalent to --all; included for compatibility with $(ls -A)"
complete -c eza -s d -l treat-dirs-as-files -d "List directories like regular files"
complete -c eza -s L -l level -d "Limit the depth of recursion" -x -a "1 2 3 4 5 6 7 8 9"
complete -c eza -s w -l width -d "Limits column output of grid, 0 implies auto-width"
complete -c eza -l spacing -d "Set the space between columns"
complete -c eza -s r -l reverse -d "Reverse the sort order"
complete -c eza -s s -l sort -d "Which field to sort by" -x -a "
    accessed\t'Sort by file accessed time'
    age\t'Sort by file modified time (newest first)'
    changed\t'Sort by changed time'
    created\t'Sort by file modified time'
    date\t'Sort by file modified time'
    ext\t'Sort by file extension'
    Ext\t'Sort by file extension (uppercase first)'
    extension\t'Sort by file extension'
    Extension\t'Sort by file extension (uppercase first)'
    filename\t'Sort by filename'
    Filename\t'Sort by filename (uppercase first)'
    inode\t'Sort by file inode'
    modified\t'Sort by file modified time'
    name\t'Sort by filename'
    Name\t'Sort by filename (uppercase first)'
    newest\t'Sort by file modified time (newest first)'
    none\t'Do not sort files at all'
    oldest\t'Sort by file modified time'
    size\t'Sort by file size'
    time\t'Sort by file modified time'
    type\t'Sort by file type'
"

complete -c eza -s I -l ignore-glob -d "Ignore files that match these glob patterns" -r
complete -c eza -s D -l only-dirs -d "List only directories"
complete -c eza -s f -l only-files -d "List only files"
complete -c eza -l show-symlinks -d "Explicitly show symbolic links (For use with --only-dirs | --only-files)"
complete -c eza -l no-symlinks -d "Do not show symbolic links"

# Long view options
complete -c eza -s b -l binary -d "List file sizes with binary prefixes"
complete -c eza -s B -l bytes -d "List file sizes in bytes, without any prefixes"
complete -c eza -s g -l group -d "List each file's group"
complete -c eza -s h -l header -d "Add a header row to each column"
complete -c eza -s H -l links -d "List each file's number of hard links"
complete -c eza -s i -l inode -d "List each file's inode number"
complete -c eza -s S -l blocksize -d "List each file's size of allocated file system blocks"
complete -c eza -s t -l time -d "Which timestamp field to list" -x -a "
    modified\t'Display modified time'
    changed\t'Display changed time'
    accessed\t'Display accessed time'
    created\t'Display created time'
"
complete -c eza -s m -l modified -d "Use the modified timestamp field"
complete -c eza -s n -l numeric -d "List numeric user and group IDs."
complete -c eza -l changed -d "Use the changed timestamp field"
complete -c eza -s u -l accessed -d "Use the accessed timestamp field"
complete -c eza -s U -l created -d "Use the created timestamp field"
complete -c eza -l time-style -d "How to format timestamps" -x -a "
    default\t'Use the default time style'
    iso\t'Display brief ISO timestamps'
    long-iso\t'Display longer ISO timestamps, up to the minute'
    full-iso\t'Display full ISO timestamps, up to the nanosecond'
    relative\t'Display relative timestamps'
    +FORMAT\t'Use custom time style'
"
complete -c eza -l total-size -d "Show recursive directory size (unix only)"
complete -c eza -l no-permissions -d "Suppress the permissions field"
complete -c eza -s o -l octal-permissions -d "List each file's permission in octal format"
complete -c eza -l no-filesize -d "Suppress the filesize field"
complete -c eza -l no-user -d "Suppress the user field"
complete -c eza -l no-time -d "Suppress the time field"
complete -c eza -s M -l mounts -d "Show mount details"
complete -c eza -l stdin -d "When piping to eza. Read file names from stdin"

# Optional extras
complete -c eza -l git -d "List each file's Git status, if tracked"
complete -c eza -l no-git -d "Suppress Git status"
complete -c eza -l git-repos -d "List each git-repos status and branch name"
complete -c eza -l git-repos-no-status -d "List each git-repos branch name (much faster)"
complete -c eza -s '@' -l extended -d "List each file's extended attributes and sizes"
complete -c eza -s Z -l context -d "List each file's security context"
