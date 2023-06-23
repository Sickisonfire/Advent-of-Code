const std = @import("std");
const Input = @import("../utils/input.zig").Input;
const fs = std.fs;
const Solution = @import("../main.zig").Solution;

pub fn solve(allocator: std.mem.Allocator) !Solution {
    // read file on build time
    // const s = @embedFile("one.txt");
    // std.debug.print("{s}", .{s});

    var input = Input(u8).init(allocator);
    defer input.deinit();
    _ = try input.read_from_file("data/one.txt");

    var lines_iter = input.lines();

    var sum_p1: i32 = 0;
    var sum_p2: i32 = 0;
    while (lines_iter.next()) |line| {
        var line_int = try std.fmt.parseInt(i32, line, 0);
        var fuel_p1 = calc_fuel(line_int);
        var fuel_p2 = calc_fuel_rec(line_int, 0);
        sum_p1 += fuel_p1;
        sum_p2 += fuel_p2;
    }
    const solution = Solution{ .part_one = sum_p1, .part_two = sum_p2 };
    return solution;
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
