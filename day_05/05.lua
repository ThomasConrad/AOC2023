local function readAll(file)
    local f = assert(io.open(file, "rb"))
    local content = f:read("*all")
    f:close()
    return content
end

-- interval list: {{start, end}, ...}
local function expand_intervals(intervals, new_interval)
    local new_start, new_end = new_interval[1], new_interval[2]
    local merged = {}
    local placed = false

    for _, interval in ipairs(intervals) do
        local start, finish = interval[1], interval[2]
        if finish < new_start then
            table.insert(merged, interval)
        elseif new_end < start then
            if not placed then
                table.insert(merged, {new_start, new_end})
                placed = true
            end
            table.insert(merged, interval)
        else
            new_start = math.min(new_start, start)
            new_end = math.max(new_end, finish)
        end
    end

    if not placed then
        table.insert(merged, {new_start, new_end})
    end

    return merged
end

local function interval_length(interval)
    return interval[2] - interval[1] + 1
end

local function parseInterval(line)
    local a, b = line:match("^(%d+)%s*-%s*(%d+)$")
    if a and b then
        return tonumber(a), tonumber(b)
    else
        error("Invalid interval format: " .. line)
    end
end

local input = readAll("input.txt")

local part1, part2 = input:match("^(.-)\n\n(.*)$")
assert(part1 and part2, "Input missing blank line separator")

local intervals = part1:gmatch("[^\n]+")

local ingredients = part2:gmatch("[^\n]+")

local parsedIntervals = {}
for interval in intervals do
    table.insert(parsedIntervals, {parseInterval(interval)})
end

local freshCount = 0
for ingredient in ingredients do
    for _, interval in ipairs(parsedIntervals) do
        local start, finish = interval[1], interval[2]
        if tonumber(ingredient) >= start and tonumber(ingredient) <= finish then
            freshCount = freshCount + 1
            break
        end
    end
end

print("Total fresh ingredients:", freshCount)

-- Part 2: Merging intervals

local interval_list = {}
for _, interval in ipairs(parsedIntervals) do
    interval_list = expand_intervals(interval_list, interval)
end

local total_covered = 0
for _, interval in ipairs(interval_list) do
    total_covered = total_covered + interval_length(interval)
end
print("Total covered range after merging intervals:", total_covered)
