# First Steps

## Introduction

The goal of this chapter is to show you how to produce useful graphics with `gongbi`.

## Fuel economy data

In this chapter, we will use the `mpg` dataset from the `ggplot2` package. The dataset includes information about the fuel economy of popular car models in 1999 and 2008, collected by the US Environmental Protection Agency.

```rust
// Read the dataset to a polars DataFrame
let mpg = ...
println!("{mpg:?}");
```

```text
shape: (234, 11)
┌──────────────┬────────┬───────┬──────┬───┬─────┬─────┬─────┬─────────┐
│ manufacturer ┆ model  ┆ displ ┆ year ┆ … ┆ cty ┆ hwy ┆ fl  ┆ class   │
│ ---          ┆ ---    ┆ ---   ┆ ---  ┆   ┆ --- ┆ --- ┆ --- ┆ ---     │
│ str          ┆ str    ┆ f64   ┆ i64  ┆   ┆ i64 ┆ i64 ┆ str ┆ str     │
╞══════════════╪════════╪═══════╪══════╪═══╪═════╪═════╪═════╪═════════╡
│ audi         ┆ a4     ┆ 1.8   ┆ 1999 ┆ … ┆ 18  ┆ 29  ┆ p   ┆ compact │
│ audi         ┆ a4     ┆ 1.8   ┆ 1999 ┆ … ┆ 21  ┆ 29  ┆ p   ┆ compact │
│ audi         ┆ a4     ┆ 2.0   ┆ 2008 ┆ … ┆ 20  ┆ 31  ┆ p   ┆ compact │
│ audi         ┆ a4     ┆ 2.0   ┆ 2008 ┆ … ┆ 21  ┆ 30  ┆ p   ┆ compact │
│ audi         ┆ a4     ┆ 2.8   ┆ 1999 ┆ … ┆ 16  ┆ 26  ┆ p   ┆ compact │
│ …            ┆ …      ┆ …     ┆ …    ┆ … ┆ …   ┆ …   ┆ …   ┆ …       │
│ volkswagen   ┆ passat ┆ 2.0   ┆ 2008 ┆ … ┆ 19  ┆ 28  ┆ p   ┆ midsize │
│ volkswagen   ┆ passat ┆ 2.0   ┆ 2008 ┆ … ┆ 21  ┆ 29  ┆ p   ┆ midsize │
│ volkswagen   ┆ passat ┆ 2.8   ┆ 1999 ┆ … ┆ 16  ┆ 26  ┆ p   ┆ midsize │
│ volkswagen   ┆ passat ┆ 2.8   ┆ 1999 ┆ … ┆ 18  ┆ 26  ┆ p   ┆ midsize │
│ volkswagen   ┆ passat ┆ 3.6   ┆ 2008 ┆ … ┆ 17  ┆ 26  ┆ p   ┆ midsize │
└──────────────┴────────┴───────┴──────┴───┴─────┴─────┴─────┴─────────┘
```

## Key components

Every `gongbi` plot is composed of three main components:

1. **Data** that you want to visualize,
2. A set of **aesthetic mappings** that describe how variables in the data are mapped to visual properties, and
3. At least one **layer** that describes how to display the data. Layers are usually created with a `geom_*` macro.

Here is a simple example:

```rust
{{#rustdoc_include examples/mpg_point.rs:10:14}}
```

![draw mpg with geom_point](examples/mpg_point.svg)

This produces a scatterplot defined by:

1. Data: `mpg` dataframe,
2. Aesthetic mappings: `x = "displ", y = "hwy"`,
3. Layer: `geom_point()`.

In the above example, data and aesthetic mappings are supplied in `plot!()`, then layers are added on with `+` operator.

Almost every plot needs data and aesthetic mappings, and most aesthetic mappings are about which columns in the data are mapped to `x` and `y` axes. Thus, `plot!()` and `aes!()` macros accept unnamed arguments to simplify the usage. That is, the following lines are equivalent:

```rust
plot!(data = mpg, mapping = aes!(x = "displ", y = "hwy"))
plot!(mpg, mapping = aes!(x = "displ", y = "hwy"))
plot!(mpg, aes!(x = "displ", y = "hwy"))
plot!(mpg, aes!("displ", y = "hwy"))
```

For simplicity, we will use the last form in this book.

## Color, size, shape and other aesthetic attributes

In addition to `x` and `y` axes, you can also provide other aesthetic mappings to control the appearance of the data points.

### Color

```rust
{{#rustdoc_include examples/mpg_point.rs:16:21}}
```

![draw mpg with geom_point and color](examples/mpg_point_color.svg)
