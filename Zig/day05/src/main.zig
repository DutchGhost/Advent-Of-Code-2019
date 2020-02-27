const PUZZLE: []const u8 = @embedFile("../../../Inputs/day05.txt");

const libint = @import("libint");
const Machine = libint.Machine;

usingnamespace @import("std");
const Allocator = mem.Allocator;

fn parse(allocator: *Allocator, s: []const u8) !ArrayList(isize) {
    var buffer = ArrayList(isize).init(allocator);
    errdefer buffer.deinit();

    var splitter = mem.separate(s, ",");

    while (splitter.next()) |w| {
        var num = try fmt.parseInt(isize, w, 10);
        try buffer.append(num);
    }

    return buffer;
}

const Inputter = struct {
    const Self = @This();
    const InputError = error{};

    output: isize = 0,

    usingnamespace libint.Read(isize, InputError);
    usingnamespace libint.Write(anyerror);

    sysid: isize,

    fn init(sysid: isize) Self {
        return Self{ .sysid = sysid };
    }
    fn read(self: *Self) PollRead {
        return PollRead{ .Ready = self.sysid };
    }

    fn write(self: *Self, value: isize) PollWrite {
        self.output = value;
        return PollWrite{ .Ready = {} };
    }
};

fn run(allocator: *Allocator, s: []const u8, inputter: Inputter) !isize {
    var buffer = try parse(allocator, s);
    defer buffer.deinit();

    var m = Machine(Inputter).init(buffer.toSlice(), inputter);
    try m.run();
    return m.io.output;
}

pub fn main() anyerror!void {
    var direct_alloc = heap.direct_allocator;
    var arena = heap.ArenaAllocator.init(direct_alloc);
    defer arena.deinit();

    var p1 = try run(&arena.allocator, PUZZLE, Inputter.init(1));
    var p2 = try run(&arena.allocator, PUZZLE, Inputter.init(5));

    debug.warn("Part 1: {}\nPart2: {}", .{ p1, p2 });
}
