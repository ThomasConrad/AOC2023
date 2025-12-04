defmodule Mix.Tasks.Solve do
  use Mix.Task

  @shortdoc "Run a day's solver: mix solve <day> <folder> [part1|part2]"

  @impl true
  def run([day, folder, part]) do
    run_parts(day, folder, [normalize_part!(part)])
  end

  def run([day, folder]) do
    run_parts(day, folder, [:part1, :part2])
  end

  def run(_args) do
    Mix.raise("Usage: mix solve <day> <folder> [part1|part2]")
  end

  defp run_parts(day_str, folder_str, parts) do
    day = String.trim(day_str)
    folder = String.trim(folder_str)
    module = module_for_day(day)
    input = read_input(folder, day)

    Enum.each(parts, fn part ->
      ensure_part_exists!(module, part)
      label = "Answer #{day} (#{folder}, #{part})"
      module |> apply(part, [input]) |> IO.inspect(label: label)
    end)
  end

  defp module_for_day(day_str) do
    # accept "1" or "01"
    padded = day_str |> String.to_integer() |> Integer.to_string() |> String.pad_leading(2, "0")
    Module.concat(Aoc, :"Day#{padded}")
  end

  defp normalize_part!(part_str) do
    part = part_str |> String.trim() |> String.downcase()

    cond do
      part in ["1", "part1"] -> :part1
      part in ["2", "part2"] -> :part2
      true -> Mix.raise("Invalid part #{inspect(part_str)}; use part1 or part2")
    end
  end

  defp ensure_part_exists!(module, part_fun) do
    unless Code.ensure_loaded?(module) and function_exported?(module, part_fun, 1) do
      Mix.raise("Module #{inspect(module)} with #{part_fun}/1 not found")
    end
  end

  defp read_input(folder, day) do
    path = "#{folder}/#{day}.txt"

    case File.read(path) do
      {:ok, content} -> content
      {:error, reason} -> Mix.raise("Failed to read #{path}: #{inspect(reason)}")
    end
  end
end
