package day08

import "core:fmt"
import "core:os"
import "core:strings"
import "core:strconv"
import "core:sort"

DistancePair :: struct {
    from: u32,
    to: u32,
    distance: i64
}

JUNCTIONS :: 1000
Vector3 :: [3]i64
main :: proc() {
	data, ok := os.read_entire_file("input.txt", context.allocator)
    if !ok {
        return
    }
    
    coords := parse(string(data))
    distances := [dynamic]DistancePair{}
    for i in 0..<len(coords)-1 {
        for j in i+1..<len(coords) {
            distSq := sq(coords[i][0] - coords[j][0]) + sq(coords[i][1] - coords[j][1]) + sq(coords[i][2] - coords[j][2])
            append(&distances, DistancePair{u32(i), u32(j), distSq})
        }
    }

    sort.quick_sort_proc(distances[:], proc(a: DistancePair, b: DistancePair) -> int {
        return int(a.distance - b.distance)
    })

    groups := [dynamic]map[u32]bool{}
    groupIndices := map[u32]u32{}
    

    // connection loop
    for pair in distances[:JUNCTIONS] {
        connect_one(pair, &groups, &groupIndices)
    }

    // find 3 largest groups
    sizes := [dynamic]u32{}
    for &group in groups {
        append(&sizes, u32(len(group)))
    }
    sort.quick_sort(sizes[:])
    
    total := u32(1)
    for i in 0..<3 {
        total *= sizes[len(sizes)-1 - i]
    }
    fmt.println("Product of sizes of largest groups: ", total)

    // keep connecting until all are connected
    for pair in distances {
        connect_one(pair, &groups, &groupIndices)
        // we are fully connected once any group has all coordinates
        if len(groups[groupIndices[pair.from]]) == len(coords) {
            fmt.println("All connected, x product: ", coords[pair.from].x * coords[pair.to].x)
            return
        }
    }
}

parse :: proc(input: string) -> [dynamic]Vector3 {
    lines := strings.split_lines(input)
    out := [dynamic]Vector3{}
    for line in lines {
        coords := strings.split_n(line, ",", 3)
        x, _ := strconv.parse_int(coords[0])
        y, _ := strconv.parse_int(coords[1])
        z, _ := strconv.parse_int(coords[2])
        append(&out, Vector3{i64(x), i64(y), i64(z)})
    }
    return out
}

sq :: proc(x: i64) -> i64 {
    return x * x
}

connect_one :: proc(pair: DistancePair, groups: ^[dynamic]map[u32]bool, groupIndices: ^map[u32]u32) {
    fromGroup, fromExists := groupIndices[pair.from]
    toGroup, toExists := groupIndices[pair.to]
    if !fromExists && !toExists {
        // new group
        newGroupIndex := u32(len(groups))
        groupIndices[pair.from] = newGroupIndex
        groupIndices[pair.to] = newGroupIndex
        newGroup := map[u32]bool{}
        newGroup[pair.from] = true
        newGroup[pair.to] = true
        append(groups, newGroup)
        return
    }
    if fromExists && toExists {
        if fromGroup == toGroup {
            // already in same group
            return
        }
        // merge groups
        for member in groups[toGroup] {
            groups[fromGroup][member] = true
            groupIndices[member] = fromGroup
        }

        groups[toGroup] = map[u32]bool{}
        return
    }
    if fromExists {
        // add to from group
        groups[fromGroup][pair.to] = true
        groupIndices[pair.to] = fromGroup
        return
    }
    // add to to group
    groups[toGroup][pair.from] = true
    groupIndices[pair.from] = toGroup
    return
}
