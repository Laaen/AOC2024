module p_1;

import std.file;
import std.stdio;
import std.algorithm;
import std.array;
import std.conv;
import std.math;
import std.range;

void main(){
    auto data = File("input", "r")
    .byLineCopy
    .map!((line) => to!(int[])(line.split("   ")))
    .array
    .transposed;

    int result = zip(data[0].array.sort, data[1].array.sort).map!((elt) => abs(elt[0] - elt[1])).sum;
    
    writeln(result);
}