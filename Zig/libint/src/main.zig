const InvalidMode = error{InvalidMode};

const Mode = enum {
    Immediate,
    Position,
    Relative,

    const Self = @This();

    fn from(mode: isize) InvalidMode!Self {
        return switch (mode) {
            0 => Self.Position,
            1 => Self.Immediate,
            2 => Self.Relative,
            else => error.InvalidMode,
        };
    }
};

const PollVariant = enum {
    Ready,
    Pending,
};

fn Poll(comptime T: type) type {
    return union(PollVariant) {
        Ready: T,
        Pending,
    };
}

const MnemonicParseError = error{InvalidMnemonic};

const Mnemonic = enum {
    Add,
    Mul,
    Save,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustBase,
    Halt,

    const Self = @This();

    fn from(opcode: isize) MnemonicParseError!Self {
        return switch (@rem(opcode, 100)) {
            1 => Self.Add,
            2 => Self.Mul,
            3 => Self.Save,
            4 => Self.Output,
            5 => Self.JumpIfTrue,
            6 => Self.JumpIfFalse,
            7 => Self.LessThan,
            8 => Self.Equals,
            9 => Self.AdjustBase,
            99 => Self.Halt,
            else => error.InvalidMnemonic,
        };
    }
};

const Opcode = struct {
    modes: [3]Mode,
    mnemonic: Mnemonic,

    const Self = @This();

    fn from(code: isize) !Self {
        var mnemonic = try Mnemonic.from(code);

        var modes = [3]Mode{
            try Mode.from(@rem(@divFloor(code, 100), 10)),
            try Mode.from(@rem(@divFloor(code, 1000), 10)),
            try Mode.from(@rem(@divFloor(code, 10000), 10)),
        };

        return Self{
            .modes = modes,
            .mnemonic = mnemonic,
        };
    }
};

const MachineError = error{
    OutOfBounds,
    WriteInImmediateMode,
    InvalidMnemonic,
    InvalidMode,
};

pub fn Machine(comptime IO: type) type {
    return struct {
        memory: []isize,
        esp: usize = 0,
        ebp: isize = 0,
        io: IO,

        const Self = @This();
        const Polled = Poll(void);

        /// Creates a new machine from `memory` and `io`.
        pub fn init(memory: []isize, io: IO) Self {
            return Self{
                .memory = memory,
                .io = io,
            };
        }

        fn write(self: *Self, addr: usize, value: isize) !void {
            if (addr >= self.memory.len) {
                return error.OutOfBounds;
            }
            self.memory[addr] = value;
        }

        fn write_operand(self: *Self, index: usize, value: isize, mode: Mode) !void {
            var offset = self.esp + index;

            var addr = switch (mode) {
                Mode.Immediate => return error.WriteInImmediateMode,
                Mode.Position => (try self.read(offset)),
                Mode.Relative => (try self.read(offset)) + self.ebp,
            };

            try self.write(@intCast(usize, addr), value);
        }

        fn read(self: *const Self, addr: usize) !isize {
            if (addr >= self.memory.len) {
                return error.OutOfBounds;
            }
            return self.memory[addr];
        }

        fn read_operand(self: *const Self, index: usize, mode: Mode) !isize {
            var offset = self.esp + index;
            var addr = switch (mode) {
                Mode.Immediate => @intCast(isize, index),
                Mode.Position => (try self.read(offset)),
                Mode.Relative => (try self.read(offset)) + self.ebp,
            };

            return self.read(@intCast(usize, addr));
        }

        fn add(self: *Self, modes: []const Mode) !Polled {
            var r1 = try self.read_operand(1, modes[0]);
            var r2 = try self.read_operand(2, modes[1]);

            try self.write_operand(3, r1 + r2, modes[2]);

            self.esp += 4;
            return Polled.Pending;
        }

        fn mul(self: *Self, modes: []const Mode) !Polled {
            var r1 = try self.read_operand(1, modes[0]);
            var r2 = try self.read_operand(2, modes[1]);

            try self.write_operand(3, r1 * r2, modes[2]);

            self.esp += 4;
            return Polled.Pending;
        }

        fn save(self: *Self, modes: []const Mode) !Polled {
            var value = switch (self.io.read()) {
                PollVariant.Pending => return Polled.Pending,
                PollVariant.Ready => |item| try item,
            };

            var addr = switch (modes[0]) {
                Mode.Immediate => return error.WriteInImmediateMode,
                Mode.Position => (try self.read(self.esp + 1)),
                Mode.Relative => (try self.read(self.esp + 1)) + self.ebp,
            };

            try self.write(@intCast(usize, addr), value);
            self.esp += 2;

            return Polled.Pending;
        }

        fn output(self: *Self, modes: []const Mode) !Polled {
            var value = try self.read_operand(1, modes[0]);
            switch (self.io.write(value)) {
                PollVariant.Pending => return Polled.Pending,
                PollVariant.Ready => |item| try item,
            }

            self.esp += 2;
            return Polled.Pending;
        }

        fn jump_if_true(self: *Self, modes: []const Mode) !Polled {
            var cond = try self.read_operand(1, modes[0]);
            var esp = try self.read_operand(2, modes[1]);

            if (cond != 0) {
                self.esp = @intCast(usize, esp);
            } else {
                self.esp += 3;
            }

            return Polled.Pending;
        }

        fn jump_if_false(self: *Self, modes: []const Mode) !Polled {
            var cond = try self.read_operand(1, modes[0]);
            var esp = try self.read_operand(2, modes[1]);

            if (cond == 0) {
                self.esp = @intCast(usize, esp);
            } else {
                self.esp += 3;
            }

            return Polled.Pending;
        }

        fn less_than(self: *Self, modes: []const Mode) !Polled {
            var r1 = try self.read_operand(1, modes[0]);
            var r2 = try self.read_operand(2, modes[1]);

            var value = @intCast(isize, @boolToInt(r1 < r2));

            try self.write_operand(3, value, modes[2]);
            self.esp += 4;

            return Polled.Pending;
        }

        fn equals(self: *Self, modes: []const Mode) !Polled {
            var r1 = try self.read_operand(1, modes[0]);
            var r2 = try self.read_operand(2, modes[1]);

            var value = @as(isize, @boolToInt(r1 == r2));

            try self.write_operand(3, value, modes[3]);
            self.esp += 4;

            return Polled.Pending;
        }

        fn adjust_base(self: *Self, modes: []const Mode) !Polled {
            var ebp = try self.read_operand(1, modes[0]);
            self.ebp += ebp;

            self.esp += 2;

            return Polled.Pending;
        }

        fn halt(self: *Self) MachineError!Polled {
            return Polled{ .Ready = {} };
        }

        /// Attempts to run one step of the intcode program.
        /// Returns `Pending` when the program has not finished,
        /// `Ready` when the program has finished, or an error
        /// whenever something went wrong.
        pub fn poll(self: *Self) !Polled {
            var opcode = try Opcode.from(try self.read(self.esp));
            var modes = opcode.modes[0..];

            return switch (opcode.mnemonic) {
                Mnemonic.Add => self.add(modes),
                Mnemonic.Mul => self.mul(modes),
                Mnemonic.Save => self.save(modes),
                Mnemonic.Output => self.output(modes),
                Mnemonic.JumpIfTrue => self.jump_if_true(modes),
                Mnemonic.JumpIfFalse => self.jump_if_false(modes),
                Mnemonic.LessThan => self.less_than(modes),
                Mnemonic.Equals => self.equals(modes),
                Mnemonic.AdjustBase => self.adjust_base(modes),
                Mnemonic.Halt => self.halt(),
            };
        }

        /// Runs the whole intcode program. Returns an
        /// error whenever something went wrong, a
        /// void otherwise.
        pub fn run(self: *Self) !void {
            while (true) {
                switch (try self.poll()) {
                    PollVariant.Pending => continue,
                    PollVariant.Ready => return,
                }
            }
        }
    };
}

pub fn Read(comptime __ITEM__: type, comptime __ERROR__: type) type {
    return struct {
        const Item = __ITEM__;
        const Error = __ERROR__;
        pub const PollRead = Poll(Error!Item);
    };
}

pub fn Write(comptime __ERROR__: type) type {
    return struct {
        const Error = __ERROR__;
        pub const PollWrite = Poll(Error!void);
    };
}

test "" {
    const ReadError = error{ReadError};

    const Dummy = struct {
        const Self = @This();

        usingnamespace Read(isize, ReadError);
        usingnamespace Write(anyerror);

        fn init() Self {
            return Self{};
        }

        fn read(self: *Self) PollRead {
            return PollRead{ .Ready = 10 };
        }

        fn write(self: *Self, value: isize) PollWrite {
            return PollWrite{ .Ready = {} };
        }
    };

    comptime {
        var slice = [_]isize{ 1, 1, 1, 4, 99, 5, 6, 0, 99 };

        var m = Machine(Dummy).init(slice[0..], Dummy.init());

        try m.run();

        @import("std").debug.assert(slice[0] == 30);
    }
}

test "opcode" {
    comptime {
        var opcode = try Opcode.from(21003);

        const testing = @import("std").testing;

        testing.expectEqual(opcode.modes[0], Mode.Position);
        testing.expectEqual(opcode.modes[1], Mode.Immediate);
        testing.expectEqual(opcode.modes[2], Mode.Relative);
    }
}
