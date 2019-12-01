const PUZZLE: []const u8 = @embedFile("../../../Inputs/day01.txt");

usingnamespace @import("std");
usingnamespace @import("fuel.zig");

fn part1(s: []const u8) i64 {
    var lines = mem.separate(s, "\r\n");

    var result = i64(0);
    while (lines.next()) |line| {
        var mass = fmt.parseInt(i64, line, 10) catch @panic("Failed to parse int");

        result += @divFloor(mass, 3) - 2;
    }

    return result;
}

fn part2(s: []const u8) i64 {
    var lines = mem.separate(s, "\r\n");

    var result = i64(0);

    while (lines.next()) |line| {
        var mass = fmt.parseInt(i64, line, 10) catch @panic("Failed to parse int");

        var fueliter = FuelIterator.new(mass);

        while (fueliter.next()) |fuel| {
            result += fuel;
        }
    }

    return result;
}

pub fn main() anyerror!void {
    @setEvalBranchQuota(15000);
    comptime var p1 = part1(PUZZLE);
    comptime var p2 = part2(PUZZLE);

    debug.warn("Part 1: {}\nPart 2: {}", p1, p2);
}
