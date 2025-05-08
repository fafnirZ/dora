# dora
like dora...the explorer, exploring data files like csv/parquet/xlsx in the terminal

![](./assets/screenshot.png)

## what does the app do?
It is an terminal based, interactive dataframe explorer built ontop of:
- polars
- ratatui

## Supported File types: 
- [x] csv (local|gcs)
- [x] parquet (local|gcs)
- [ ] xlsx
- [ ] multisheet excel

## App Modes:
- [x] normal mode
- [x] search mode
- [x] command mode
- [ ] help mode
- [ ] filter mode


## Supported Operations (TODO)
- [ ] select
- [ ] filter
- [ ] order by
- [ ] display the current query expression

## Commands
[link](./docs/commands.md)

## References
Code structure and functionality heavily inspired from https://github.com/YS-L/csvlens