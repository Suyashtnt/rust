const PARSED = [
    {
        query: "-> F<P>",
        elems: [],
        foundElems: 1,
        userQuery: "-> F<P>",
        returned: [{
            name: "F",
            fullPath: ["f"],
            pathWithoutLast: [],
            pathLast: "f",
            generics: [
                {
                    name: "P",
                    fullPath: ["p"],
                    pathWithoutLast: [],
                    pathLast: "p",
                    generics: [],
                },
            ],
            typeFilter: -1,
        }],
        error: null,
    },
    {
        query: "-> P",
        elems: [],
        foundElems: 1,
        userQuery: "-> P",
        returned: [{
            name: "P",
            fullPath: ["p"],
            pathWithoutLast: [],
            pathLast: "p",
            generics: [],
            typeFilter: -1,
        }],
        error: null,
    },
    {
        query: "->,a",
        elems: [],
        foundElems: 1,
        userQuery: "->,a",
        returned: [{
            name: "a",
            fullPath: ["a"],
            pathWithoutLast: [],
            pathLast: "a",
            generics: [],
            typeFilter: -1,
        }],
        error: null,
    },
    {
        query: "aaaaa->a",
        elems: [{
            name: "aaaaa",
            fullPath: ["aaaaa"],
            pathWithoutLast: [],
            pathLast: "aaaaa",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 2,
        userQuery: "aaaaa->a",
        returned: [{
            name: "a",
            fullPath: ["a"],
            pathWithoutLast: [],
            pathLast: "a",
            generics: [],
            typeFilter: -1,
        }],
        error: null,
    },
    {
        query: "-> !",
        elems: [],
        foundElems: 1,
        userQuery: "-> !",
        returned: [{
            name: "never",
            fullPath: ["never"],
            pathWithoutLast: [],
            pathLast: "never",
            generics: [],
            typeFilter: 1,
        }],
        error: null,
    },
    {
        query: "a->",
        elems: [{
            name: "a",
            fullPath: ["a"],
            pathWithoutLast: [],
            pathLast: "a",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 1,
        userQuery: "a->",
        returned: [],
        hasReturnArrow: true,
        error: null,
    },
    {
        query: "!->",
        elems: [{
            name: "never",
            fullPath: ["never"],
            pathWithoutLast: [],
            pathLast: "never",
            generics: [],
            typeFilter: 1,
        }],
        foundElems: 1,
        userQuery: "!->",
        returned: [],
        hasReturnArrow: true,
        error: null,
    },
    {
        query: "! ->",
        elems: [{
            name: "never",
            fullPath: ["never"],
            pathWithoutLast: [],
            pathLast: "never",
            generics: [],
            typeFilter: 1,
        }],
        foundElems: 1,
        userQuery: "! ->",
        returned: [],
        hasReturnArrow: true,
        error: null,
    },
    {
        query: "primitive:!->",
        elems: [{
            name: "never",
            fullPath: ["never"],
            pathWithoutLast: [],
            pathLast: "never",
            generics: [],
            typeFilter: 1,
        }],
        foundElems: 1,
        userQuery: "primitive:!->",
        returned: [],
        hasReturnArrow: true,
        error: null,
    },
];
