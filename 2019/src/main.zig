const std = @import("std");
const print = @import("std").debug.print;
const One = @import("days/one.zig");
const Two = @import("days/two.zig");

pub const Solution = struct { part_one: i32, part_two: i32 };

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    for (args[1..]) |arg| {
        var num = std.fmt.parseInt(i32, arg, 0) catch {
            const e = error.InvalidInput;
            return e;
        };

        const solution: Solution = switch (num) {
            1 => try One.solve(allocator),
            2 => try Two.solve(allocator),
            3 => error.NotImplemented,
            4 => error.NotImplemented,
            5 => error.NotImplemented,
            6 => error.NotImplemented,
            7 => error.NotImplemented,
            8 => error.NotImplemented,
            9 => error.NotImplemented,
            10 => error.NotImplemented,
            11 => error.NotImplemented,
            12 => error.NotImplemented,
            13 => error.NotImplemented,
            14 => error.NotImplemented,
            15 => error.NotImplemented,
            16 => error.NotImplemented,
            17 => error.NotImplemented,
            18 => error.NotImplemented,
            19 => error.NotImplemented,
            20 => error.NotImplemented,
            21 => error.NotImplemented,
            22 => error.NotImplemented,
            23 => error.NotImplemented,
            24 => error.NotImplemented,
            25 => error.NotImplemented,
            else => error.InvalidDate,
        } catch |err| {
            print("\n\n", .{});
            return err;
        };
        print("day {d}:\n Part 1: {d}\n Part 2: {d}\n", .{ num, solution.part_one, solution.part_two });
    }
}
