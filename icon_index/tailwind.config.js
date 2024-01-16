module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    darkMode: "class",
    theme: {
        extend: {
            colors: {
                primary: "rgb(var(--color-primary) / 1)",
                secondary: "rgb(var(--color-secondary) / 1)",
                emphasis: "rgb(var(--color-emphasis) / 1)",
                'primary-dark': "rgb(var(--color-primary-dark) / 1)",
                'secondary-dark': "rgb(var(--color-secondary-dark) / 1)",
                'emphasis-dark': "rgb(var(--color-emphasis-dark) / 1)",
            }
        }
    },
}
