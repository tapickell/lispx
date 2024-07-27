const mecha = @import("mecha");
const std = @import("std");

pub fn main() !void {
    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("LispZ Version 0.0.0.0.1.\n", .{});
    try stdout.print("Interactive LispZ - Press Ctrl+c to exit\n", .{});
    // var history = read_history(history_filename);

    var buffer: [256]u8 = undefined; // Buffer to hold the input
    const stdin = std.io.getStdIn().reader();
    while (true) {
        try stdout.print("lispz> ", .{});
        try bw.flush(); // don't forget to flush!
        const line = try stdin.readUntilDelimiterOrEof(&buffer, '\n');
        // check for (:exit)
        if (std.mem.eql(u8, line, "(:exit)")) {
            break;
        } else {
            // add line to history
            // parse form
            // eval parsed form
            // print evaluated result
            try stdout.print("  => ", .{});
            try stdout.print("{?s}\n", .{line});
        }
    }
}

//fn parseLine(line: []const u8) ![]mecha.Token {
//    var tokenizer = mecha.Tokenizer.init(line);

//    // Define the tokens
//    const OpenParen = tokenizer.tokenId("OpenParen", "(");
//    const CloseParen = tokenizer.tokenId("CloseParen", ")");
//    const Integer = tokenizer.tokenId("Integer", mecha.isDigit);
//    const Plus = tokenizer.tokenId("Plus", "+");
//    const Minus = tokenizer.tokenId("Minus", "-");
//    const Multiply = tokenizer.tokenId("Multiply", "*");
//    const Divide = tokenizer.tokenId("Divide", "/");
//    const Modulo = tokenizer.tokenId("Modulo", "%");
//    const Power = tokenizer.tokenId("Power", "^");

//    // Parse the line into tokens
//    var tokens: []mecha.Token = undefined;
//    while (tokenizer.peekToken().?.id != mecha.Tokenizer.EndOfFile) {
//        const token = try tokenizer.nextToken();
//        try tokens.append(token);
//    }

//    return tokens;
//}

//fn assert_ok(s: []const u8) !void {
//    const res = json.parse(testing.allocator, s) catch @panic("test failure");
//    try testing.expectEqualStrings("", res.rest);
//}

//test "simple test" {
//    try assert_ok("(+ 40 2)");
//}
