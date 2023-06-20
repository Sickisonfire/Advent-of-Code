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

    var sum_p1: i32 = 0;
    var sum_p2: i32 = 0;
    while (lines_iter.next()) |line| {
        if (line.len != 0) {
            var line_int = try std.fmt.parseInt(i32, line, 0);
            var fuel_p1 = calc_fuel(line_int);
            var fuel_p2 = calc_fuel_rec(line_int, 0);
            sum_p1 += fuel_p1;
            sum_p2 += fuel_p2;
        }
    }
    std.debug.print("Part one: {d} \n", .{sum_p1});
    std.debug.print("Part two: {d} \n", .{sum_p2});
}

// recurson like this may result in undefined behaviour
fn calc_fuel_rec(weight: i32, acc: i32) i32 {
    var total: i32 = acc;
    if (weight == 0) return total;

    var fuel: i32 = calc_fuel(weight);
    total += fuel;

    return calc_fuel_rec(fuel, total);
}

fn calc_fuel(weight: i32) i32 {
    // zero or negative cases
    if (weight <= 8) return 0;

    return @floatToInt(i32, @intToFloat(f32, weight) / 3.0) - 2;
}
