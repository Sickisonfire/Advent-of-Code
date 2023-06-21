const std = @import("std");
const Solution = @import("../main.zig").Solution;

pub fn solve(allocator: std.mem.Allocator) !Solution {
    _ = allocator;

    const solution = Solution{
        .part_one = 0,
        .part_two = 0,
    };
    return solution;
}
