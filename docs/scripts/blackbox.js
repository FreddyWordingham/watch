function transform(word) {
    console.log("Transforming word: ", word);

    word = word.toUpperCase();

    if (word == "DOG") {
        console.log("Dog field");
        return 3084720398509283;
    }
    if (word == "FIELD") {
        console.log("Found field");
        return 4324382764329402;
    }

    alert("I don't know that word yet...");
    return 0;
}
