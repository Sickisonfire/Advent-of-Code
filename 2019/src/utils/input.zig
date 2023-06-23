const std = @import("std");
const fs = std.fs;

pub fn Input(comptime T: type) type {
    return struct {
        const Self = @This();

        /// raw Input string
        raw: []T,
        allocator: std.mem.Allocator,

        pub fn init(alloc: std.mem.Allocator) Self {
            return Self{ .allocator = alloc, .raw = &[_]T{} };
        }

        pub fn read_from_file(self: *Self, path: []const T) !usize {
            var file = try fs.cwd().openFile(path, .{});
            defer file.close();

            const file_size = try file.getEndPos();

            self.raw = try self.allocator.alloc(T, file_size - 1);

            return try file.read(self.raw);
        }

        pub fn deinit(self: Self) void {
            self.allocator.free(self.raw);
        }

        pub fn split(self: *Self, delim: []const u8) std.mem.SplitIterator(T, .any) {
            return std.mem.splitAny(T, self.raw, delim);
        }
    };
}

// pub const InputIterator = struct {
//     content: []const []const u8,
//     delim: []const u8,
//     index: usize = 0,

//     pub fn init(content: *Input, delim: []const u8) !InputIterator {
//         var lines_iter = std.mem.splitSequence(u8, content, delim);

//         return InputIterator{ .raw_content = lines_iter };
//     }

//     pub fn split(self: InputIterator) InputIterator {
//         return self;
//     }
// };
