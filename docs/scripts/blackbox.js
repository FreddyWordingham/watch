function transform(word) {
    console.log("Transforming word: ", word);

    word = word.toUpperCase();

    if (word == "FIELD") {
        console.log("Found field");
        return 4324382764329402;
    }

    alert("I don't know that word yet...");
    return 0;
}
