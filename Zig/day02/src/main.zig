const PUZZLE: []const u8 = @embedFile("../../../Inputs/day02.txt");

usingnamespace @import("std");

fn parse(s: []const u8) !ArrayList(usize) {
    var buffer = ArrayList(usize).init(heap.direct_allocator);
    errdefer buffer.deinit();

    var splitter = mem.separate(PUZZLE, ",");

    while (splitter.next()) |w| {
        var num = try fmt.parseInt(usize, w, 10);
        try buffer.append(num);
    }

    return buffer;
}

fn run(program: []usize, noun: usize, verb: usize) usize {
    var ip = usize(0);
    program[1] = noun;
    program[2] = verb;

    while (true) {
        var rhs_place = usize(0);
        var lhs_place = usize(0);
        var result_place = usize(0);

        switch (program[ip]) {
            1...2 => {
                lhs_place = program[ip + 1];
                rhs_place = program[ip + 2];
                result_place = program[ip + 3];
            },
            99 => break,
            else => @panic("unreachable"),
        }

        switch (program[ip]) {
            1 => program[result_place] = program[lhs_place] + program[rhs_place],
            2 => program[result_place] = program[lhs_place] * program[rhs_place],
            else => @panic("unreachable"),
        }

        ip += 4;
    }

    return program[0];
}

fn part1(s: []const u8) !usize {
    var parsed = try parse(s);
    defer parsed.deinit();
    return run(parsed.toSlice(), 12, 2);
}

fn part2(s: []const u8) !usize {
    var magic_number = usize(19690720);

    var parsed = try parse(s);
    defer parsed.deinit();

    var clone = ArrayList(usize).init(heap.direct_allocator);
    defer clone.deinit();

    var noun = usize(0);
    var verb = usize(0);

    while (noun < 99) : ({ noun += 1; }) {
        while (verb < 99) : ({ verb += 1; }) {
            try clone.resize(0);
            try clone.appendSlice(parsed.toSliceConst());

            if (run(clone.toSlice(), noun, verb) == magic_number) {
                return 100 * noun + verb;
            }
        }

        verb = 0;
    }

    @panic("Unreachable");
}

pub fn main() anyerror!void {
    var p1 = try part1(PUZZLE);
    var p2 = try part2(PUZZLE);

    debug.warn("Part 1: {}\nPart 2: {}", p1, p2);
}
