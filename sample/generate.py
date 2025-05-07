import polars as pl
import random

df = pl.read_csv("a.csv")
df_len = len(df)
new_df = df.with_columns(
    pl.Series(name="random_values", values=[
        random.randrange(0,9999) for _ in range(df_len)
    ])
)
# new_df.write_csv("b.csv")
new_df.write_parquet("b.parquet")