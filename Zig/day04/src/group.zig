pub fn Grouped(comptime T: type) type {
    return struct {
        slice: []T,

        const Self = @This();

        pub fn groups(slice: []T) Self {
            return Self{ .slice = slice };
        }

        pub fn next(self: *Self) ?[]T {
            if (self.slice.len == 0) {
                return null;
            }

            var fst = self.slice[0];

            var count = usize(0);

            for (self.slice) |elem| {
                if (!(elem == fst)) {
                    break;
                }
                count += 1;
            }

            var ret = self.slice[0..count];
            self.slice = self.slice[count..self.slice.len];
            return ret;
        }
    };
}
