const math = @import("std").math;

fn transform(n: i64) ?i64 {
    return switch (@divFloor(n, 3) - 2) {
        1...math.maxInt(i64) => |num| num,
        else => null,
    };
}

pub const FuelIterator = struct {
    fuel: ?i64,

    const Self = @This();

    pub fn new(fuel: i64) Self {
        return Self { .fuel = fuel };
    }

    pub fn next(self: *Self) ?i64 {
        const fuel = self.fuel orelse return null;
        self.fuel = transform(fuel);
        return self.fuel;
    }
};
