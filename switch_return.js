function getApiBasePath(e) {
    switch (e) {
        case "a":
            return "http://a.api";
    
        case "b":
            return "http://b.api";
    }
}

console.log(getApiBasePath("a"), ": ", typeof getApiBasePath("a"));
console.log(getApiBasePath("c"), ": ", typeof getApiBasePath("c"));

console.log(getApiBasePath("a") != null);
console.log(getApiBasePath("c") != null);
console.log(getApiBasePath("c") !== null);
