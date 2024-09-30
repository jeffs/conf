# CL(1)

# NAME

    cl

## SYNOPSIS

    cl [-y | --yesterday]

## DESCRIPTION

Defines a `cl` command to open a date-specific target directory in VS Code.  The directory path is `~/log/YYYY/MM/DD`, where `YYYY/MM/DD` is the current date, or yesterday's date if the `-y` or `--yesterday` flag is specified.  The directory is created automatically if it does not exist.  It is initialized as a git repository, regardless of whether it previously existed.

## EXIT STATUS

The `cl` command returns 0 on success, 1 if an I/O error occurs (e.g., if the target directory does not exist and cannot be automatically created), and 2 if any command-line argument is not recognized.
