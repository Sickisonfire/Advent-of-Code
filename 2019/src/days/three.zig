const std = @import("std");
const Input = @import("../utils/input.zig").Input;
const fs = std.fs;
const Solution = @import("../main.zig").Solution;

pub fn solve(allocator: std.mem.Allocator) !Solution {
    var input = Input(u8).init(allocator);
    defer input.deinit();
    _ = try input.read_from_file("data/one.txt");

    std.debug.print("{}", .{input.lines().next()});
    return .{
        .part_one = 0,
        .part_two = 0,
    };
}
