module p_2;

import std.file;
import std.stdio;
import std.algorithm;
import std.array;
import std.conv;
import std.range;

void main(){
    auto data = File("input", "r")
    .byLineCopy
    .map!((line) => to!(int[])(line.split("   ")))
    .array
    .transposed;

    int result = data[0].fold!((int res, int elt) => res + (elt * count(data[1], elt).to!int) )(0);

    writeln(result);
}