const std = @import("std");
const mem = std.mem;

const group = @import("group.zig");

const BEGIN: [6]u8 = [_]u8{ 1, 4, 6, 8, 1, 0 };
const END: [6]u8 = [_]u8{ 6, 1, 2, 5, 6, 4 };

pub fn loopy(fun: fn (u8, u8, u8, u8, u8, u8) bool) !usize {
    var valids = usize(0);

    var a = u8(0);
    while (a < 10) : (a += 1) {
        var b = a;

        while (b < 10) : (b += 1) {
            var c = b;

            while (c < 10) : (c += 1) {
                var d = c;

                while (d < 10) : (d += 1) {
                    var e = d;

                    while (e < 10) : (e += 1) {
                        var f = e;

                        while (f < 10) : (f += 1) {
                            var array = [_]u8{ a, b, c, d, e, f };
                            if (mem.lessThan(u8, array, BEGIN)) {
                                continue;
                            }

                            if (mem.lessThan(u8, END, array)) {
                                return valids;
                            }

                            if (fun(a, b, c, d, e, f)) {
                                valids += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    return error.Invalid;
}

fn two_equal(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) bool {
    return a == b or b == c or c == d or d == e or e == f;
}

fn part1() !usize {
    return loopy(two_equal);
}

fn groups(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) bool {
    var array = [_]u8{ a, b, c, d, e, f };

    var grouping = group.Grouped(u8).groups(&array);

    while (grouping.next()) |elem| {
        if (elem.len == 2) {
            return true;
        }
    }

    return false;
}

fn part2() !usize {
    return loopy(groups);
}

pub fn main() anyerror!void {
    @setEvalBranchQuota(10000000);
    var p1 = part1();
    var p2 = part2();
    std.debug.warn("Part 1: {}\nPart 2: {}\n", p1, p2);
}
