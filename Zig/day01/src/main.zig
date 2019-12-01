const PUZZLE: []const u8 = @embedFile("../../../Inputs/day01.txt");

usingnamespace @import("std");

fn part1(s: []const u8) i64 {
    var lines = mem.separate(s, "\r\n");

    var result = i64(0);
    while (lines.next()) |line| {
        var mass = fmt.parseInt(i64, line, 10) catch @panic("Failed to parse int");

        result += @divFloor(mass, 3) - 2;
    }

    return result;
}

fn transform(n: i64) ?i64 {
    return switch (@divFloor(n, 3) - 2) {
        1...math.maxInt(i64) => |num| num,
        else => null,
    };
}

const FuelIterator = struct {
    fuel: ?i64,

    const Self = @This();

    fn next(self: *Self) ?i64 {
        if (self.fuel) |fuel| {
            self.fuel = transform(fuel);
            return fuel;
        } else {
            return null;
        }
    }
};

fn part2(s: []const u8) i64 {
    var lines = mem.separate(s, "\r\n");

    var result = i64(0);

    while (lines.next()) |line| {
        var mass = fmt.parseInt(i64, line, 10) catch @panic("Failed to parse int");

        var fueliter = FuelIterator{ .fuel = mass };

        while (fueliter.next()) |fuel| {
            result += fuel;
        }
    }

    return result;
}

pub fn main() anyerror!void {
    @setEvalBranchQuota(30000);
    comptime var p1 = part1(PUZZLE);
    var p2 = part2(PUZZLE);
    debug.warn("Part 1: {}\nPart 2: {}", p1, p2);
}
