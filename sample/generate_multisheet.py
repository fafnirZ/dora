import polars as pl
import random
import pandas as pd

df = pl.read_csv("a.csv")
df_len = len(df)

num_sheets = 3
polars_dfs_to_write = {}
for i in range(num_sheets):
    random_values = [random.randrange(0, 9999) for _ in range(df_len)]
    temp_df = df.with_columns(
        pl.Series(name=f"random_values_sheet_{i+1}", values=random_values)
    )
    polars_dfs_to_write[f"Sheet{i+1}"] = temp_df

# Convert Polars DataFrames to Pandas DataFrames
pandas_dfs_to_write = {
    sheet_name: pdf.to_pandas()
    for sheet_name, pdf in polars_dfs_to_write.items()
}

# Use pandas ExcelWriter to create a multi-sheeted Excel file
excel_file = "multi_sheet_output.xlsx"
try:
    with pd.ExcelWriter(excel_file) as writer:
        for sheet_name, pandas_df in pandas_dfs_to_write.items():
            pandas_df.to_excel(writer, sheet_name=sheet_name, index=False)
    print(f"Successfully created '{excel_file}' with multiple sheets.")
except ImportError:
    print(
        "Error: pandas requires 'openpyxl' or 'xlsxwriter' to write Excel files."
    )
    print("Please install one of them (e.g., 'pip install openpyxl').")