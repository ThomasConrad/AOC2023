defmodule Aoc.Day01 do
  @spec part1(String.t()) :: any()
  def part1(raw) do
    raw
    |> parse()
    |> part1_solve({50, 0})
  end

  @spec part2(String.t()) :: any()
  def part2(raw) do
    raw
    |> parse()
    |> part2_solve({50, 0})
  end

  defp parse(raw) do
    raw
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_line/1)
  end

  defp parse_line(<<dir::binary-size(1), rest::binary>>) do
    {dir == "L", String.to_integer(rest)}
  end

  defp modulo(a, b), do: rem(rem(a, b) + b, b)

  defp part1_solve(numbers, {val, count}) do
    Enum.reduce(numbers, {val, count}, fn
      {is_left, n}, {val, count} ->
        new_val =
          case is_left do
            true -> modulo(val - n, 100)
            false -> modulo(val + n, 100)
          end

        {new_val, count + if(new_val == 0, do: 1, else: 0)}
    end)
  end

  defp part2_solve(numbers, {val, count}) do
    Enum.reduce(numbers, {val, count}, fn
      {is_left, n}, {val, count} ->
        {new_val, diff} =
          case is_left do
            true -> {modulo(val - n, 100), -n}
            false -> {modulo(val + n, 100), n}
          end

        required =
          case {val, diff} do
            {0, diff} -> 100
            {val, diff} when diff < 0 -> val
            _ -> 100 - val
          end

        count = count + max(0, floor((n - required) / 100) + 1)

        {new_val, count}
    end)
  end
end
