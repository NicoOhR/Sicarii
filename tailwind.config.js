/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./templates/**/*.html', './static/about/index.html'],
  theme: {
    extend: {
      fontFamily: {
        unifraktur: ['"UnifrakturMaguntia"', 'serif'],
        wittgenstein: ['"Wittgenstein"', 'serif'],
        baskervville: ['"Baskervville SC"', 'serif', 'italic'],
        jetbrains: ['"JetBrains Mono"'],
        garamond: ['"EB Garamond"', 'Georgia', 'serif', 'italic']
      },
    },
  },
  plugins: [],
}

