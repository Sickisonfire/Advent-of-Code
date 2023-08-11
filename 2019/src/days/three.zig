const std = @import("std");
const Input = @import("../utils/input.zig").Input;
const fs = std.fs;
const Solution = @import("../main.zig").Solution;
const ArrayList = std.ArrayList;
const parseInt = std.fmt.parseInt;

const Point = struct {
    x: i32,
    y: i32,
};

const LineSegment = struct {
    p1: Point,
    p2: Point,
};

pub fn solve(allocator: std.mem.Allocator) !Solution {
    var input = Input(u8).init(allocator);
    defer input.deinit();
    _ = try input.read_from_file("data/three.txt");

    var lines = input.lines();

    var first_wire_path_segments = ArrayList(LineSegment).init(allocator);
    defer first_wire_path_segments.deinit();
    var second_wire_path_segments = ArrayList(LineSegment).init(allocator);
    defer second_wire_path_segments.deinit();

    var wire_path_segments = &first_wire_path_segments;

    while (lines.next()) |wire| {
        var instruction_iter = std.mem.tokenizeScalar(u8, wire, ',');
        var prev = Point{ .x = 0, .y = 0 };
        while (instruction_iter.next()) |instruction| {
            try switch (instruction[0]) {
                'U' => {
                    var magnitude = try parseInt(i32, instruction[1..], 10);
                    var p2 = Point{ .x = prev.x, .y = prev.y + magnitude };
                    try wire_path_segments.append(LineSegment{ .p1 = Point{ .x = prev.x, .y = prev.y }, .p2 = p2 });
                    prev = p2;
                },
                'D' => {
                    var magnitude = try parseInt(i32, instruction[1..], 10);
                    var p2 = Point{ .x = prev.x, .y = prev.y - magnitude };
                    try wire_path_segments.append(LineSegment{ .p1 = Point{ .x = prev.x, .y = prev.y }, .p2 = p2 });
                    prev = p2;
                },
                'R' => {
                    var magnitude = try parseInt(i32, instruction[1..], 10);
                    var p2 = Point{ .x = prev.x + magnitude, .y = prev.y };
                    try wire_path_segments.append(LineSegment{ .p1 = Point{ .x = prev.x, .y = prev.y }, .p2 = p2 });
                    prev = p2;
                },
                'L' => {
                    var magnitude = try parseInt(i32, instruction[1..], 10);
                    var p2 = Point{ .x = prev.x - magnitude, .y = prev.y };
                    try wire_path_segments.append(LineSegment{ .p1 = Point{ .x = prev.x, .y = prev.y }, .p2 = p2 });
                    prev = p2;
                },
                else => error.InvalidDirection,
            };
            break;
        }
        std.debug.print("{any}", .{wire_path_segments.pop()});
        wire_path_segments = &second_wire_path_segments;
        break;
    }

    return .{
        .part_one = 0,
        .part_two = 0,
    };
}
