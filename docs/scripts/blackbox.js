function transform(word) {
    console.log("Transforming word: ", word);

    word = word.toUpperCase();

    if (word == "DREAM") {
        return 3426372374893290;
    }
    if (word == "FIELD") {
        return 4324382764329402;
    }
    if (word == "POND") {
        return 4632743894942332;
    }
    if (word == "VIVID") {
        return 9375834738739834;
    }
    if (word == "ANTLER") {
        return 5643743724839445;
    }

    alert("I don't know that word yet...");
    return 0;
}
