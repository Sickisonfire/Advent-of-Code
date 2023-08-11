const std = @import("std");
const Input = @import("../utils/input.zig").Input;
const fs = std.fs;
const Solution = @import("../main.zig").Solution;

pub fn solve(allocator: std.mem.Allocator) !Solution {
    var input = Input(u8).init(allocator);
    defer input.deinit();
    _ = try input.read_from_file("data/three.txt");
    //split \n
    // split ,
    // parse direction + value

    // var first_wire_path =

    var lines = input.lines();
    while (lines.next()) |wire| {
        var instruction_iter = std.mem.tokenizeScalar(u8, wire, ',');
        while (instruction_iter.next()) |instruction| {
            switch (instruction[0]) {
                'U' => std.debug.print("{}", .{try std.fmt.parseInt(usize, instruction[1..], 10)}),
                'D' => std.debug.print("{}", .{try std.fmt.parseInt(usize, instruction[1..], 10)}),
                'L' => std.debug.print("{}", .{try std.fmt.parseInt(usize, instruction[1..], 10)}),
                'R' => std.debug.print("{}", .{try std.fmt.parseInt(usize, instruction[1..], 10)}),
                else => unreachable,
            }
            break;
        }
    }

    return .{
        .part_one = 0,
        .part_two = 0,
    };
}
