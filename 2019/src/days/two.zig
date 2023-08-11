const std = @import("std");
const Solution = @import("../main.zig").Solution;
const Input = @import("../utils/input.zig").Input;
const Arraylist = std.ArrayList;

pub fn solve(allocator: std.mem.Allocator) !Solution {
    var input = Input(u8).init(allocator);
    defer input.deinit();
    _ = try input.read_from_file("data/two.txt");

    var iter = input.split(",");

    var arr = Arraylist(usize).init(allocator);
    defer arr.deinit();

    while (iter.next()) |item| {
        const item_int = try std.fmt.parseInt(usize, item, 0);
        try arr.append(item_int);
    }

    var target: usize = 19690720;

    var p1 = try partOne(allocator, &arr.items);
    var p2 = try partTwo(allocator, &arr.items, target);

    const solution = Solution{
        .part_one = @intCast(p1),
        .part_two = @intCast(p2),
    };
    return solution;
}

fn partOne(allocator: std.mem.Allocator, data: *[]usize) !usize {
    var arr = try allocator.dupe(usize, data.*);
    defer allocator.free(arr);
    var lhs: usize = 0;
    var rhs: usize = 0;
    var dest: usize = 0;
    var i: usize = 0;

    arr[1] = 12;
    arr[2] = 2;

    while (i < arr.len) : (i += 4) {
        var item = arr[i];

        try switch (item) {
            1 => {
                lhs = arr[i + 1];
                rhs = arr[i + 2];
                dest = arr[i + 3];
                arr[dest] = arr[lhs] + arr[rhs];
            },
            2 => {
                lhs = arr[i + 1];
                rhs = arr[i + 2];
                dest = arr[i + 3];
                arr[dest] = arr[lhs] * arr[rhs];
            },
            99 => break,
            else => error.UnknownOperation,
        };
    }
    return arr[0];
}

fn partTwo(allocator: std.mem.Allocator, data: *[]usize, target: usize) !usize {
    var lhs: usize = 0;
    var rhs: usize = 0;
    var dest: usize = 0;
    var i: usize = 0;
    var solution: usize = 0;

    var arr = try allocator.dupe(usize, data.*);
    defer allocator.free(arr);

    outer: for (0..100) |n| {
        for (0..100) |v| {
            // init input data
            // for (data[0..data.len], 0..) |x, idx| arr[idx] = x;
            std.mem.copyForwards(usize, arr, data.*);
            arr[1] = n;
            arr[2] = v;

            lhs = 0;
            rhs = 0;
            dest = 0;
            i = 0;
            while (i < arr.len) : (i += 4) {
                var item = arr[i];

                try switch (item) {
                    1 => {
                        lhs = arr[i + 1];
                        rhs = arr[i + 2];
                        dest = arr[i + 3];
                        arr[dest] = arr[lhs] + arr[rhs];
                    },
                    2 => {
                        lhs = arr[i + 1];
                        rhs = arr[i + 2];
                        dest = arr[i + 3];
                        arr[dest] = arr[lhs] * arr[rhs];
                    },
                    99 => break,
                    else => error.UnknownOperation,
                };
            }
            if (arr[0] == target) {
                solution = 100 * n + v;
                std.debug.print("{d}", .{solution});
                break :outer;
            }
        }
    }
    return solution;
}
