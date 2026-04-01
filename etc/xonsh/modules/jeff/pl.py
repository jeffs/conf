import polars as pl


def configure():
    pl.Config.set_tbl_rows(100)  # max rows shown (default 10)
    pl.Config.set_tbl_cols(20)  # max columns shown
    pl.Config.set_fmt_str_lengths(200)  # max characters per string cell
    pl.Config.set_tbl_width_chars(1000)  # total table width in characters


def configure_huge():
    pl.Config.set_tbl_rows(-1)  # show ALL rows
    pl.Config.set_tbl_cols(-1)  # show ALL columns
    pl.Config.set_fmt_str_lengths(1000)  # effectively no truncation
