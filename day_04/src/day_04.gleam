import gleam/dict
import gleam/list
import gleam/set
import gleam/string
import simplifile

type Coord =
  #(Int, Int)

type Space {
  Roll
  Empty
}

pub fn main() {
  let assert Ok(input) = simplifile.read("input.txt")
  let grid = parse(input)

  let roll_set =
    grid
    |> dict.filter(fn(_, v) { v == Roll })
    |> dict.to_list
    |> list.map(fn(pair) { pair.0 })
    |> set.from_list

  let nbr_coords = collect_nbr_coords(grid)

  let roll_counts = initial_roll_counts(roll_set, nbr_coords)

  // Part 1:
  let accessible_count =
    roll_set
    |> set.to_list
    |> list.filter(fn(coord) {
      case dict.get(roll_counts, coord) {
        Ok(n) -> n < 4
        _ -> False
      }
    })
    |> list.length

  echo accessible_count

  // Part 2
  let initial_count = set.size(roll_set)
  let final_rolls = process_mover(roll_set, roll_counts, nbr_coords)
  let final_count = set.size(final_rolls)

  echo initial_count - final_count
}

fn process_mover(
  rolls: set.Set(Coord),
  counts: dict.Dict(Coord, Int),
  nbrs: dict.Dict(Coord, List(Coord)),
) -> set.Set(Coord) {
  // Compute accessible rolls (count < 4)
  let accessible =
    rolls
    |> set.to_list
    |> list.filter(fn(coord) {
      case dict.get(counts, coord) {
        Ok(n) -> n < 4
        _ -> False
      }
    })

  case list.is_empty(accessible) {
    True -> rolls
    False -> {
      let to_remove = set.from_list(accessible)

      // Remove them from rolls
      let new_rolls = set.difference(rolls, to_remove)

      // Update neighbour counts efficiently
      let new_counts = update_counts_after_removal(counts, to_remove, nbrs)

      process_mover(new_rolls, new_counts, nbrs)
    }
  }
}

fn update_counts_after_removal(
  counts: dict.Dict(Coord, Int),
  removed: set.Set(Coord),
  nbrs: dict.Dict(Coord, List(Coord)),
) -> dict.Dict(Coord, Int) {
  removed
  |> set.to_list
  |> list.fold(counts, fn(acc, coord) {
    let neighbours = case dict.get(nbrs, coord) {
      Ok(n) -> n
      _ -> []
    }

    neighbours
    |> list.fold(acc, fn(acc2, nbr) {
      case dict.get(acc2, nbr) {
        Ok(n) -> dict.insert(acc2, nbr, n - 1)
        _ -> acc2
      }
    })
  })
}

fn initial_roll_counts(
  rolls: set.Set(Coord),
  nbrs: dict.Dict(Coord, List(Coord)),
) -> dict.Dict(Coord, Int) {
  rolls
  |> set.to_list
  |> list.map(fn(coord) {
    let neighbours = case dict.get(nbrs, coord) {
      Ok(n) -> n
      _ -> []
    }

    let count =
      neighbours
      |> list.filter(fn(nc) { set.contains(rolls, nc) })
      |> list.length

    #(coord, count)
  })
  |> dict.from_list
}

fn parse(input: String) -> dict.Dict(Coord, Space) {
  input
  |> string.split("\n")
  |> list.index_map(fn(line, y) {
    line
    |> string.to_graphemes
    |> list.index_map(fn(ch, x) {
      let v = case ch {
        "." -> Empty
        "@" -> Roll
        _ -> Empty
      }
      #(#(x, y), v)
    })
  })
  |> list.flatten
  |> dict.from_list
}

fn collect_nbr_coords(
  grid: dict.Dict(Coord, Space),
) -> dict.Dict(Coord, List(Coord)) {
  grid
  |> dict.to_list
  |> list.map(fn(pair) {
    let coord = pair.0
    let #(x, y) = coord
    #(coord, get_nbr_coords(x, y))
  })
  |> dict.from_list
}

fn get_nbr_coords(x: Int, y: Int) -> List(Coord) {
  [
    #(x - 1, y - 1),
    #(x - 1, y),
    #(x - 1, y + 1),
    #(x, y - 1),
    #(x, y + 1),
    #(x + 1, y - 1),
    #(x + 1, y),
    #(x + 1, y + 1),
  ]
}
