/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./templates/**/*.html', './static/about/index.html'],
  theme: {
    extend: {
      fontFamily: {
        unifraktur: ['"UnifrakturMaguntia"', 'serif'],
        baskervville: ['"Baskervville SC"', 'serif', 'italic'],
      },
    },
  },
  plugins: [],
}

