const std = @import("std");
const fs = std.fs;
const ArrayList = std.ArrayList;
pub fn solve(allocator: std.mem.Allocator) !void {
    // read file on build time
    // const s = @embedFile("one.txt");
    // std.debug.print("{s}", .{s});

    var file = try fs.cwd().openFile("data/one.txt", .{});
    defer file.close();

    const file_size = try file.getEndPos();
    var file_buf = try allocator.alloc(u8, file_size);
    defer allocator.free(file_buf);
    _ = try file.read(file_buf);

    var lines_iter = std.mem.splitSequence(u8, file_buf, "\n");

    var sum: i32 = 0;
    while (lines_iter.next()) |line| {
        if (line.len != 0) {
            var line_int = try std.fmt.parseInt(i32, line, 0);
            var module_fuel = @floatToInt(i32, @intToFloat(f32, line_int) / 3.0) - 2;
            sum += module_fuel;
        }
    }
    std.debug.print("{d} ", .{sum});
}
