function transform(word) {
    console.log("Transforming word: ", word);

    word = word.toUpperCase();

    if (word == "DREAM") {
        return 3426372374893290;
    }
    if (word == "FIELD") {
        console.log("Found field");
        return 4324382764329402;
    }
    if (word == "VIVID") {
        return 9375834738739834;
    }

    alert("I don't know that word yet...");
    return 0;
}
