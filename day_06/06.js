const fs = require('fs');

let input = fs.readFileSync("input.txt", "utf8");

let lines = input.split("\n");

var nums = [];
for (let i = 0; i < lines.length - 1; i++) {

    let parts = lines[i].match(/\S+/g) || [];
    var row = []
    
    for (let j = 0; j < parts.length; j++) {
        let num = parseInt(parts[j]);
        row.push(num);
    }
    nums.push(row); 
}


// part 1 
var total = 0;  
for (const [i, symbol] of (lines[lines.length - 1].match(/\S+/g) || []).entries()) {
    let result = accum(nums, i, symbol);
    total += result;
}
console.log("Total:", total);

// len per col
lens = [];
for (let i = 0; i < nums[0].length; i++) {
    let maxlen = 0;
    for (let j = 0; j < nums.length; j++) {
        let numlen = nums[j][i].toString().length;
        if (numlen > maxlen) {
            maxlen = numlen;
        }
    }
    lens.push(maxlen);
}

// wack parsing ensues
var groups = [];
for (let i = 0; i < lines.length - 1; i++) {

    //chunk in maxlen
    var group = [];
    let index = 0;
    for (let j = 0; j < lens.length; j++) {
        let chunk = lines[i].substr(index, lens[j]);
        index += lens[j] + 1; // +1 for space
        group.push(chunk);
    }
    groups.push(group);
}

// replace whitespace with 0
for (let i = 0; i < groups.length; i++) {
    for (let j = 0; j < groups[i].length; j++) {
        let chunk = groups[i][j];
        let newchunk = chunk.replace(/\s/g, "0");
        groups[i][j] = newchunk;
    }
}

var parsed = [];
for (let i = 0; i < lens.length; i++) {
    var block = [];
    for (let j = 0; j < groups.length; j++) {
        block.push(parseInt(groups[j][i]));
    }
    let max = Math.max(...block);
    let order = Math.floor(Math.log10(max));
 
    // console.log("Block:", block);

    var matrix = [];
    for (let r = 0; r < block.length; r++) {
        var mrow = [];
        for (let c = 0; c <= order; c++) {
            let v = block[r] % 10;
            block[r] = Math.floor(block[r] / 10);
            mrow.push(v);
        }
        matrix.push(mrow.reverse());
    }

    // //transpose and combine
    var transposed = [];
    for (let r = 0; r <= order; r++) {
        var trow = [];
        for (let c = 0; c < matrix.length; c++) {
            trow.push(matrix[c][r]);
        }
        transposed.push(trow);
    }

    // flatten back to nums, removing zeros
    var col = []
    for (const [_, num] of transposed.entries()) {
        var string = num.toString();
        string = string.replace(/[0,]/g, "");
        col.push(parseInt(string));
    }
    parsed.push(col);
}


// part 2
total = 0;
for (const [i, symbol] of (lines[lines.length - 1].match(/\S+/g) || []).entries()) {
    let result = accum2(parsed, i, symbol);
    total += result;
}


function accum(arr, row, symbol) {
    let sum = (symbol == "*") ? 1 : 0;
    for (let j = 0; j < arr.length; j++) {
        if (symbol == "*") {
            sum *= arr[j][row];
        }
        else if (symbol == "+") {
            sum += arr[j][row];
        }    
    }
    return sum;
}

function accum2(arr, col, symbol) {
    let sum = (symbol == "*") ? 1 : 0;
    for (let j = 0; j < arr[col].length; j++) {
        if (symbol == "*") {
            sum *= arr[col][j];
        }
        else if (symbol == "+") {
            sum += arr[col][j];
        }    
    }
    return sum;
}


console.log("Total:", total);