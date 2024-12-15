/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./templates/*.html', './static/about/index.html',],
  theme: {
    extend: {
      fontFamily: {
        wittgenstein: ['"Wittgenstein"', 'serif'],
      },
      transform: ['hover'],
      scale: ['hover'],
    },
  },
  plugins: [],
}

