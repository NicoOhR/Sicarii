/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*"],
  theme: {
    extend: {
      fontFamily: {
        wittgenstein: ['"Wittgenstein"', "serif"],
        bodoni: ['"Bodoni Moda"', "serif", "italic"],
        jetbrains: ['"JetBrains Mono"'],
        garamond: ['"EB Garamond"', "Georgia", "serif", "italic"],
        baskervville: ['"Baskervville SC"', "serif", "italic"],
        news: ["'Newsreader'", "serif", "italic"],
      },
    },
  },
  plugins: [],
};
