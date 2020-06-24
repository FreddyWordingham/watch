function transform(word) {
    console.log("Transforming word: ", word);

    word = word.toUpperCase();

    var score = word.length;
    score += (word.indexOf("A") + 1) * 2;
    score += (word.indexOf("B") + 1) * 2;
    score += (word.indexOf("C") + 1) * 3;
    score += (word.indexOf("D") + 1) * 5;
    score += (word.indexOf("E") + 1) * 4;
    score += (word.indexOf("F") + 1) * 7;
    score += (word.indexOf("G") + 1) * 11;
    score += (word.indexOf("H") + 1) * 13;
    score += (word.indexOf("I") + 1) * 8;
    score += (word.indexOf("J") + 1) * 17;
    score += (word.indexOf("K") + 1) * 19;
    score += (word.indexOf("L") + 1) * 23;
    score += (word.indexOf("M") + 1) * 29;
    score += (word.indexOf("N") + 1) * 31;
    score += (word.indexOf("O") + 1) * 16;
    score += (word.indexOf("P") + 1) * 37;
    score += (word.indexOf("Q") + 1) * 41;
    score += (word.indexOf("R") + 1) * 43;
    score += (word.indexOf("S") + 1) * 47;
    score += (word.indexOf("T") + 1) * 53;
    score += (word.indexOf("U") + 1) * 32;
    score += (word.indexOf("V") + 1) * 59;
    score += (word.indexOf("W") + 1) * 61;
    score += (word.indexOf("X") + 1) * 67;
    score += (word.indexOf("Y") + 1) * 71;
    score += (word.indexOf("Z") + 1) * 73;
    score += (word.indexOf("_") + 1) * 64;
    console.log(score);

    alert("I don't know that word yet...");
    return 0;
}
